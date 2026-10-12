//! `-compile` runs of ONE module with DIFFERENT cfgs, started together,
//! each run their own checker. The cache keys each binary by sources + cfg,
//! but the builds share one cargo target dir: when every generated package
//! had the module's name, they all wrote `target/release/tlcgen-<module>`,
//! and a run could copy out another cfg's binary, which then refused to run
//! ("generated from different sources ... regenerate it"; 7 of 9 parallel
//! runs on 2026-10-12). Now each key builds its own package.
//!
//! Builds two generated crates (release, LTO), so it takes a few minutes.

use std::path::Path;
use std::process::Command;

const SPEC: &str = r#"---- MODULE ParToy ----
EXTENDS Naturals
CONSTANTS N, Up, All
VARIABLE x
Init == x = 0
Next == x' = (x + 1) % N
Spec == Init /\ [][Next]_x
Rises == [][x' > x /\ x' \in Up]_x
Wraps == [][x' = (x + 1) % N /\ x' \in All]_x
====
"#;

fn cfg(prop: &str) -> String {
    format!("SPECIFICATION Spec\nCHECK_DEADLOCK FALSE\nCONSTANTS\n  N = 4\n  Up = {{1, 2, 3}}\n  All = {{0, 1, 2, 3}}\nPROPERTY {prop}\n")
}

#[test]
fn parallel_compile_runs_of_one_module_each_run_their_own_cfg() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("par-toy");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("ParToy.tla"), SPEC).unwrap();
    std::fs::write(dir.join("Rises.cfg"), cfg("Rises")).unwrap();
    std::fs::write(dir.join("Wraps.cfg"), cfg("Wraps")).unwrap();
    let cache = dir.join("cache");
    let spawn = |name: &str| {
        Command::new(env!("CARGO_BIN_EXE_tla-mc"))
            .current_dir(&dir)
            .env("TLAMC_CACHE", &cache)
            .args(["-compile", "-workers", "2", "-metadir", dir.join(format!("md-{name}")).to_str().unwrap(),
                   "-config", &format!("{name}.cfg"), "ParToy.tla"])
            .output()
            .expect("spawn")
    };
    let (rises, wraps) = std::thread::scope(|s| {
        let r = s.spawn(|| spawn("Rises"));
        let w = s.spawn(|| spawn("Wraps"));
        (r.join().unwrap(), w.join().unwrap())
    });
    let text = |o: &std::process::Output| String::from_utf8_lossy(&o.stdout).into_owned() + &String::from_utf8_lossy(&o.stderr);
    let (rt, wt) = (text(&rises), text(&wraps));
    for t in [&rt, &wt] {
        assert!(!t.contains("regenerate it"), "a run loaded another cfg's checker: {t}");
        assert!(t.contains("generated engine"), "the generated checker must run: {t}");
    }
    assert!(rt.contains("Action property Rises is violated"), "Rises: {rt}");
    assert_eq!(rises.status.code(), Some(12), "Rises: {rt}");
    assert!(wt.contains("No error has been found"), "Wraps: {wt}");
    assert_eq!(wraps.status.code(), Some(0), "Wraps: {wt}");
}

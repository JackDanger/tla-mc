//! An error decides nothing: a search that stops on an evaluation or I/O
//! error exits 2, not 12 (a violation). On 2026-10-12 a BFS queue that
//! filled the disk ("queue file: No space left on device") exited 12, the
//! code of a counterexample. A violated invariant still exits 12, a
//! holding run 0.

use std::path::Path;
use std::process::Command;

const SPEC: &str = r#"---- MODULE ExitToy ----
EXTENDS Naturals
VARIABLE x
Init == x = 0
Next == x' = IF x < 3 THEN x + 1 ELSE CHOOSE y \in {} : TRUE
Small == x < 3
Typed == x \in Nat
====
"#;

fn run(dir: &Path, inv: &str) -> (Option<i32>, String) {
    std::fs::write(dir.join(format!("{inv}.cfg")), format!("INIT Init\nNEXT Next\nINVARIANT {inv}\n")).unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_tla-mc"))
        .current_dir(dir)
        .args(["-workers", "2", "-metadir", dir.join(format!("md-{inv}")).to_str().unwrap(), "-config", &format!("{inv}.cfg"), "ExitToy.tla"])
        .output()
        .expect("spawn");
    (o.status.code(), String::from_utf8_lossy(&o.stdout).into_owned() + &String::from_utf8_lossy(&o.stderr))
}

#[test]
fn an_error_exits_2_and_a_violation_12() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("exit-toy");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("ExitToy.tla"), SPEC).unwrap();

    // x reaches 3; its successor is CHOOSE over the empty set: an error
    let (code, out) = run(&dir, "Typed");
    assert!(out.contains("Error:"), "the error must be reported: {out}");
    assert!(!out.contains("is violated"), "an error is not a violation: {out}");
    assert!(out.contains("distinct states found"), "the counts are still printed: {out}");
    assert_eq!(code, Some(2), "an error exits 2: {out}");

    // the violation is reached first (x = 3), before any successor errs
    let (code, out) = run(&dir, "Small");
    assert!(out.contains("Invariant Small is violated"), "{out}");
    assert_eq!(code, Some(12), "a violation exits 12: {out}");
}

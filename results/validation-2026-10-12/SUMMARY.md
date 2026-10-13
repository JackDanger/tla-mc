# tla-mc 0.1.2: a partial re-validation (2026-10-12)

What changed since 0.1.1 (validated in full on 2026-10-07): no evaluation code.

- `-compile`: parallel builds of one module with different cfgs no longer
  clobber each other's binary (`5e25b89`; `tests/compile_parallel_cfgs.rs`).
- `-queue-mem` / `-fpmem` default to an eighth of `MemAvailable` (Linux),
  floored at 256 / 1024 MB (`5e25b89`).
- `-version` (`5e25b89`).
- A search that stops on an evaluation or I/O error exits 2, not 12
  (`a9d204e`; `tests/exit_codes.rs`).

So the check is the 2026-10-07 gate run again on a subset, on flint `04765c80`
(the same tree), on an 8-core Linux box, against that run's `gate-sweep.jsonl`
(`run-val.sh`):

| leg | entries | verdict as 10-07 | same distinct states |
|---|---|---|---|
| interpreter | 343 (every entry under 5 s on 10-07) | 343 | must-hold 69/69, liveness 18/18 |
| `-compile`, a fresh cache (`tla-mc-compiled.sh`) | 10 (strict, mutation, liveness; both dirs) | 10 | must-hold 4/4, liveness 2/2 |

Mutation entries stop at their first violation, so their counts move with the
workers' interleaving; each found its violation (39/256 and 0/4 counts equal).

Not re-run: the 103 slower gate entries, the TLC side, and tlaplus/Examples.

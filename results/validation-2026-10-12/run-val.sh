#!/bin/bash
# tla-mc 5e25b89+a9d204e small validation: the gate (flint 04765c80, the tree
# the 2026-10-07 validation ran), against that run's verdicts and counts.
#   interp: the 343 entries under 5 s on 10-07, 60 s cap each
#   compiled: 10 of them (strict, mutation, liveness; both dirs), 240 s cap
V=/mnt/nvme/val; G=/mnt/nvme/gate-0402
cd $V
FLINT_ROOT=$G ONLY=$V/only-interp.txt nice -n 10 python3 $G/formal/tla-mc-gate/sweep.py /mnt/nvme/tla-mc-fix-target/release/tla-mc 60 $V/interp.jsonl > $V/interp.log 2>&1
FLINT_ROOT=$G ONLY=$V/only-compiled.txt nice -n 10 python3 $G/formal/tla-mc-gate/sweep.py $V/tla-mc-compiled.sh 240 $V/compiled.jsonl > $V/compiled.log 2>&1
echo VALDONE > $V/val.done

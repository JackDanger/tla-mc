#!/bin/bash
# sweep.py's binary for the compiled leg: the fixed tla-mc with -compile, a fresh cache
export TLAMC_CACHE=/mnt/nvme/val-cache; source ~/.cargo/env
exec /mnt/nvme/tla-mc-fix-target/release/tla-mc -compile "$@"

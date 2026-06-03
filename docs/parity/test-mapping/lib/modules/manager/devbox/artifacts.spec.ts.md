# `lib/modules/manager/devbox/artifacts.spec.ts`

[← `manager/devbox`](../../../../_by-module/manager/devbox.md) · [all modules](../../../../README.md)

**0/15 ported** (15 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 39 | skips if no updateddeps and no lockfilemaintenance | pending | — |
| 43 | skips if no lock file in config | pending | — |
| 48 | skips if cannot read lock file | pending | — |
| 55 | returns installed devbox.lock | pending | — |
| 101 | calls install instead of update --no-install if an older version of devbox is constrained | pending | — |
| 151 | returns installed devbox.lock with multiple updated deps | pending | — |
| 214 | returns null if no updateddeps are passed | pending | — |
| 231 | returns null if no updateddeps have depnames | pending | — |
| 253 | returns updated devbox.lock | pending | — |
| 300 | calls update without --no-install flag if an older version of devbox is being used | pending | — |
| 350 | returns null if no changes are found | pending | — |
| 369 | returns null if devbox.lock not found after update | pending | — |
| 396 | returns null if devbox.lock not found | pending | — |
| 422 | returns null if no lock file changes are found | pending | — |
| 450 | returns an artifact error on failure | pending | — |


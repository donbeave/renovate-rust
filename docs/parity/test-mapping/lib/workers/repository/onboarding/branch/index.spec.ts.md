# `lib/workers/repository/onboarding/branch/index.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/26 ported** (26 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 57 | throws if no package files | pending | — |
| 63 | doesn't throw if there are no package files and onboardingnodeps config option is set | pending | — |
| 73 | throws if fork | pending | — |
| 80 | throws if bot disabled | pending | — |
| 87 | _(it.each / template — verify manually)_ | ? | — |
| 127 | uses discovered onboarding config | pending | — |
| 170 | handles skipped onboarding combined with requireconfig = optional | pending | — |
| 181 | handles skipped onboarding, requireconfig=required, and a config file | pending | — |
| 192 | handles skipped onboarding, requireconfig=ignored | pending | — |
| 203 | handles skipped onboarding, requireconfig=required, and no config file | pending | — |
| 216 | detects repo is onboarded via file | pending | — |
| 223 | handles removed cached file name | pending | — |
| 230 | handles cached file name | pending | — |
| 253 | handles cached package.json | pending | — |
| 279 | detects repo is onboarded via package.json config | pending | — |
| 286 | detects repo is onboarded via pr | pending | — |
| 297 | throws if no required config | pending | — |
| 310 | rebases onboarding branch | pending | — |
| 347 | skips processing onboarding branch when main/onboarding shas have not changed | pending | — |
| 379 | processes modified onboarding branch and invalidates extract cache | pending | — |
| 417 | skips processing conflicted onboarding branch | pending | — |
| 440 | sets onboarding cache for existing onboarding branch | pending | — |
| 474 | detects unsupported platfom | pending | — |
| 495 | detects missing rebase checkbox | pending | — |
| 511 | detects manual pr update requested | pending | — |
| 527 | handles unchecked rebase checkbox | pending | — |


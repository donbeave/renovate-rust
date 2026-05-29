# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/aws-machine-image/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/aws-machine-image/index.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** pending

### `modules/datasource/aws-machine-image/index › getSortedAwsMachineImages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| with 3 returned images | 137 | pending | — | — | —|
| with 1 returned image | 147 | pending | — | — | —|
| without returned images | 157 | pending | — | — | —|

### `modules/datasource/aws-machine-image/index › getDigest()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| without newValue, without returned images to be null | 169 | pending | — | — | —|
| without newValue, with one matching image to return that image | 179 | pending | — | — | —|
| without newValue, with 3 matching image to return the newest image | 189 | pending | — | — | —|
| with matching newValue, with 3 matching image to return the matching image | 199 | pending | — | — | —|
| with not matching newValue, with 3 matching images to return the matching image | 212 | pending | — | — | —|

### `modules/datasource/aws-machine-image/index › getPkgReleases()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| without returned images to be null | 227 | pending | — | — | —|
| with one matching image to return that image | 237 | pending | — | — | —|
| with one deprecated matching image to return that image | 256 | pending | — | — | —|
| with 3 matching image to return the newest image | 275 | pending | — | — | —|

### `modules/datasource/aws-machine-image/index › loadConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| loads filters without aws config | 298 | pending | — | — | —|
| loads filters with multiple aws configs | 313 | pending | — | — | —|

---


# `lib/workers/repository/config-migration/branch/migrated-data.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**1/19 in-scope tests ported** (18 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 54 | calls getasync a first when migration not needed | pending | — |
| 62 | calls getasync a first time to initialize the factory | ported | [`crates/renovate-core/src/json_writer.rs:173`](../../../../../../../../crates/renovate-core/src/json_writer.rs#L173) |
| 69 | calls getasync a second time to get the saved data from before | pending | — |
| 77 | gets the filename from the class instance | pending | — |
| 82 | gets the content from the class instance | pending | — |
| 88 | resets the factory and gets a new value | pending | — |
| 95 | resets the factory and gets a new value with default indentation | pending | — |
| 110 | migrate a json5 config file | pending | — |
| 120 | falls back to json.stringify when weave fails | pending | — |
| 138 | uses json.stringify when raw is null | pending | — |
| 150 | returns nothing due to detectrepofileconfig throwing | pending | — |
| 184 | does not format when no prettier config is present | pending | — |
| 193 | does not format when failing to fetch package.json file | pending | — |
| 202 | does not format when there is an invalid package.json file | pending | — |
| 211 | formats when prettier config file is found | pending | — |
| 220 | formats without prettier if in .renovaterc | pending | — |
| 231 | formats when finds prettier config inside the package.json file | pending | — |
| 243 | formats with default 2 spaces | pending | — |
| 259 | formats with printwith=infinity | pending | — |


# `lib/workers/repository/update/branch/schedule.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**63/68 ported** (5 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | returns false for invalid timezone | ported | `crates/renovate-core/src/schedule.rs:2030` |
| 11 | returns true for valid timezone | ported | `crates/renovate-core/src/schedule.rs:2036` |
| 17 | returns true for null | ported | `crates/renovate-core/src/schedule.rs:1880` |
| 21 | returns true for at any time | ported | `crates/renovate-core/src/schedule.rs:1886` |
| 25 | returns false for invalid schedule | ported | `crates/renovate-core/src/schedule.rs:1892` |
| 29 | returns false if any schedule fails to parse | ported | `crates/renovate-core/src/schedule.rs:1898` |
| 33 | returns false if using minutes | ported | `crates/renovate-core/src/schedule.rs:1904` |
| 39 | returns false for wildcard minutes | ported | `crates/renovate-core/src/schedule.rs:1910` |
| 47 | returns false if schedules have no days or time range | ported | `crates/renovate-core/src/schedule.rs:1917` |
| 51 | returns false if any schedule has no days or time range | ported | `crates/renovate-core/src/schedule.rs:1923` |
| 57 | returns false for every xday | ported | `crates/renovate-core/src/schedule.rs:1929` |
| 61 | returns true if schedule has days of week | ported | `crates/renovate-core/src/schedule.rs:1935` |
| 67 | returns true for multi day schedules | ported | `crates/renovate-core/src/schedule.rs:1941` |
| 75 | returns true if schedule has a start time | ported | `crates/renovate-core/src/schedule.rs:1949` |
| 79 | returns true for first day of the month | ported | `crates/renovate-core/src/schedule.rs:1955` |
| 85 | returns true for schedules longer than 1 month | ported | `crates/renovate-core/src/schedule.rs:1963` |
| 91 | returns true if schedule has an end time | ported | `crates/renovate-core/src/schedule.rs:1971` |
| 95 | returns true if schedule has a start and end time | ported | `crates/renovate-core/src/schedule.rs:1977` |
| 101 | returns true if schedule has days and a start and end time | ported | `crates/renovate-core/src/schedule.rs:1985` |
| 109 | returns true if schedule uses cron syntax | ported | `crates/renovate-core/src/schedule.rs:1993` |
| 117 | massages schedules | ported | `crates/renovate-core/src/schedule.rs:2003` |
| 126 | supports hours shorthand | ported | `crates/renovate-core/src/schedule.rs:2012` |
| 154 | returns true if no schedule | ported | `crates/renovate-core/src/schedule.rs:973` |
| 159 | returns true if at any time | ported | `crates/renovate-core/src/schedule.rs:1598` |
| 165 | returns true if at any time array | ported | `crates/renovate-core/src/schedule.rs:979` |
| 171 | returns true if invalid schedule | ported | `crates/renovate-core/src/schedule.rs:2101` |
| 177 | returns true if invalid timezone | ported | `crates/renovate-core/src/schedule.rs:2110` |
| 184 | supports before hours true | ported | `crates/renovate-core/src/schedule.rs:1510` |
| 190 | supports before hours false | ported | `crates/renovate-core/src/schedule.rs:1522` |
| 196 | massages string | ported | `crates/renovate-core/src/schedule.rs:2119` |
| 202 | supports outside hours | ported | `crates/renovate-core/src/schedule.rs:1534` |
| 208 | supports cron syntax with hours | ported | `crates/renovate-core/src/schedule.rs:1546` |
| 218 | supports cron syntax with days | ported | `crates/renovate-core/src/schedule.rs:1557` |
| 228 | supports cron syntax with months | ported | `crates/renovate-core/src/schedule.rs:1568` |
| 238 | supports cron syntax with weekdays | ported | `crates/renovate-core/src/schedule.rs:1579` |
| 253 | approves if the weekday is * | ported | `crates/renovate-core/src/schedule.rs:1753` |
| 259 | approves if the weekday is 0 | ported | `crates/renovate-core/src/schedule.rs:1607` |
| 265 | rejects if the weekday is 1 | ported | `crates/renovate-core/src/schedule.rs:1616` |
| 277 | supports last day of month | ported | `crates/renovate-core/src/schedule.rs:2044` |
| 283 | supports last day of week | ported | `crates/renovate-core/src/schedule.rs:2054` |
| 293 | supports first monday of month | ported | `crates/renovate-core/src/schedule.rs:2071` |
| 303 | _(it.each / template — verify manually)_ | ? | — |
| 319 | _(it.each / template — verify manually)_ | ? | — |
| 337 | reject if day mismatch | ported | `crates/renovate-core/src/schedule.rs:1680` |
| 343 | reject if month mismatch | ported | `crates/renovate-core/src/schedule.rs:1689` |
| 349 | reject if no schedule available | ported | `crates/renovate-core/src/schedule.rs:1762` |
| 355 | supports multiple schedules | ported | `crates/renovate-core/src/schedule.rs:1625` |
| 361 | supports day match | ported | `crates/renovate-core/src/schedule.rs:1635` |
| 367 | supports day mismatch | ported | `crates/renovate-core/src/schedule.rs:1644` |
| 373 | supports every weekday | ported | `crates/renovate-core/src/schedule.rs:1653` |
| 379 | supports every weekend | ported | `crates/renovate-core/src/schedule.rs:1662` |
| 385 | supports every weekday with time | ported | `crates/renovate-core/src/schedule.rs:1671` |
| 391 | supports o every weekday | ported | `crates/renovate-core/src/schedule.rs:1771` |
| 397 | rejects first day of the month | ported | `crates/renovate-core/src/schedule.rs:1698` |
| 403 | approves first day of the month | ported | `crates/renovate-core/src/schedule.rs:1707` |
| 410 | approves valid weeks of year | ported | `crates/renovate-core/src/schedule.rs:2129` |
| 417 | rejects on weeks of year | ported | `crates/renovate-core/src/schedule.rs:2137` |
| 424 | approves on months of year | ported | `crates/renovate-core/src/schedule.rs:1716` |
| 431 | rejects on months of year | ported | `crates/renovate-core/src/schedule.rs:1725` |
| 438 | approves schedule longer than 1 month | ported | `crates/renovate-core/src/schedule.rs:1734` |
| 445 | rejects schedule longer than 1 month | ported | `crates/renovate-core/src/schedule.rs:1744` |
| 452 | approves schedule longer than 1 month with day of month | ported | `crates/renovate-core/src/schedule.rs:1780` |
| 459 | rejects schedule longer than 1 month with day of month | ported | `crates/renovate-core/src/schedule.rs:1788` |
| 466 | supports weekday instances | ported | `crates/renovate-core/src/schedule.rs:2088` |
| 483 | should correctly convert "* 22 4 * *" to human-readable format | pending | — |
| 490 | should correctly convert "* */2 * * *" to human-readable format | pending | — |
| 495 | should correctly convert "* 23 * * *" to human-readable format | pending | — |
| 500 | should not throw an error for an invalid cron expression "* * */2 6#1" | pending | — |


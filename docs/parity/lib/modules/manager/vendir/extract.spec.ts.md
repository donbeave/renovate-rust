# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/vendir/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/vendir/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid yaml file content | 10 | ported | `vendir.rs` | `invalid_yaml_returns_empty` | — |
| returns null for empty yaml file content | 15 | ported | `vendir.rs` | `empty_returns_empty` | — |
| returns null for empty directories key | 20 | ported | `vendir.rs` | `no_helm_charts_returns_empty` | — |
| returns null for nonHelmChart key | 30 | ported | `vendir.rs` | `non_helm_chart_contents_key_returns_empty` | — |
| multiple charts - extracts helm-chart from vendir.yml correctly | 35 | ported | `vendir.rs` | `extracts_helm_charts` (+ extracts_second_chart) | — |

---


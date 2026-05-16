# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/schema.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/docker/schema`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses OCI image manifest | 12 | not-applicable | — | — | Renovate's Docker OCI/distribution manifest Zod schemas are not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| parses OCI helm manifest | 57 | not-applicable | — | — | Renovate's Docker OCI Helm manifest schema is not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| parses OCI image index | 106 | not-applicable | — | — | Renovate's Docker OCI image index schema is not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| parses OCI image index and ignores unknown sub manifests | 155 | not-applicable | — | — | Renovate's Docker OCI sub-manifest filtering schema is not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| parses OCI flux artifact | 210 | not-applicable | — | — | Renovate's Docker OCI Flux artifact schema is not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| parses distribution manifest | 264 | not-applicable | — | — | Renovate's Docker distribution manifest schema is not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| parses distribution manifest list | 307 | not-applicable | — | — | Renovate's Docker distribution manifest-list schema is not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| parses OCI helm chart config | 347 | not-applicable | — | — | Renovate's Docker OCI Helm chart config schema is not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| parses devcontainer manifest | 394 | not-applicable | — | — | Renovate's Docker devcontainer manifest schema is not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |
| throws for invalid manifest | 432 | not-applicable | — | — | Renovate's Docker manifest schema validation errors are not implemented in Rust; Rust Docker support currently targets Docker Hub tag-page lookup and update summaries. |

---


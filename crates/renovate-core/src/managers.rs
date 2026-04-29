//! Package manager detection.
//!
//! Each supported package manager declares a set of file patterns. Given the
//! full file list from a repository, the detection step matches those patterns
//! and returns which managers apply — and which specific files they should
//! process.
//!
//! Renovate reference: `lib/modules/manager/*/index.ts` `defaultConfig.managerFilePatterns`.
//!
//! ## Pattern format
//!
//! Renovate's patterns are JavaScript regex strings (e.g. `"/(^|/)Cargo\\.toml$/"`).
//! This module stores the inner regex (without surrounding `/`) and compiles
//! them with the `regex` crate, which is RE2-compatible.

use std::sync::LazyLock;

use regex::Regex;

/// A detected package manager with the list of matching files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedManager {
    /// Manager identifier matching Renovate's manager names.
    pub name: &'static str,
    /// Files that matched one of the manager's patterns.
    pub matched_files: Vec<String>,
}

/// A single manager's detection record: (name, regex patterns).
///
/// Patterns are the inner regex strings from Renovate's `managerFilePatterns`.
struct ManagerDef {
    name: &'static str,
    patterns: &'static [&'static str],
}

/// Managers that are **disabled by default** in Renovate.
///
/// These managers have `defaultConfig.enabled: false` in their upstream
/// `lib/modules/manager/*/index.ts`.  They only run when the user explicitly
/// lists them in `enabledManagers` in `renovate.json`.
///
/// Reference files:
/// - `lib/modules/manager/azure-pipelines/index.ts`
/// - `lib/modules/manager/git-submodules/index.ts`
/// - `lib/modules/manager/html/index.ts`
/// - `lib/modules/manager/nix/index.ts`
/// - `lib/modules/manager/pre-commit/index.ts`
/// - `lib/modules/manager/travis/index.ts`
pub const DISABLED_BY_DEFAULT: &[&str] = &[
    "azure-pipelines",
    "git-submodules",
    "html",
    "nix",
    "pre-commit",
    "travis",
];

/// Return `true` when `manager_name` is disabled by default in Renovate.
///
/// Disabled-by-default managers require explicit opt-in via `enabledManagers`.
pub fn is_disabled_by_default(manager_name: &str) -> bool {
    DISABLED_BY_DEFAULT.contains(&manager_name)
}

/// Return the language/ecosystem categories for `manager_name`.
///
/// Categories mirror the `categories` export in each upstream
/// `lib/modules/manager/*/index.ts`.  Used to evaluate `matchCategories`
/// in `packageRules`.
///
/// Returns an empty slice when the manager is not known or has no categories.
pub fn manager_categories(manager_name: &str) -> &'static [&'static str] {
    match manager_name {
        // JS / Node
        "npm" | "bun" | "nodenv" | "nvm" | "meteor" | "mint" => &["js"],
        // Python
        "pip_requirements" | "pip-compile" | "pip_setup" | "pipenv" | "poetry" | "pep621"
        | "pep723" | "pyenv" | "runtime-version" | "setup-cfg" => &["python"],
        // Java / JVM
        "maven" | "maven-wrapper" | "gradle" | "gradle-wrapper" | "ant" | "sbt" | "leiningen"
        | "kotlin-script" => &["java"],
        // Go
        "gomod" => &["golang"],
        // Rust
        "cargo" => &["rust"],
        // Ruby
        "bundler" | "gemspec" | "ruby-version" => &["ruby"],
        // PHP
        "composer" => &["php"],
        // .NET
        "nuget" => &["dotnet"],
        // Docker / containers
        "dockerfile" | "docker-compose" | "batect" | "devcontainer" | "quadlet" => &["docker"],
        // Kubernetes
        "kubernetes" | "kustomize" | "helm" | "helm-requirements" | "helm-values" | "helmfile"
        | "helmsman" | "fleet" => &["kubernetes"],
        // Helm (also kubernetes)
        "argocd" | "glasskube" | "sveltos" | "crossplane" => &["kubernetes"],
        // Terraform / IaC
        "terraform" | "terraform-version" | "terragrunt" | "terragrunt-version"
        | "tflint-plugin" | "bicep" => &["terraform", "iac"],
        // CI/CD
        "github-actions"
        | "gitlabci"
        | "gitlabci-include"
        | "circleci"
        | "travis"
        | "droneci"
        | "buildkite"
        | "azure-pipelines"
        | "bitbucket-pipelines"
        | "cloudbuild"
        | "woodpecker"
        | "bitrise"
        | "velaci" => &["ci"],
        // Dart / Flutter
        "pub" | "fvm" => &["dart"],
        // Swift
        "spm" | "xcodegen" => &["swift"],
        // Haskell
        "cabal" => &["haskell"],
        // Elixir
        "mix" => &["elixir"],
        // Perl
        "cpanfile" => &["perl"],
        // Ansible
        "ansible" | "ansible-galaxy" => &["ansible"],
        // Bazel
        "bazel" | "bazel-module" | "bazelisk" => &["bazel"],
        // Nix
        "nix" => &["c"],
        _ => &[],
    }
}

/// Return the primary Renovate datasource ID for a manager.
///
/// Used to populate `DepContext.datasource` so `matchDatasources` rules fire
/// correctly when evaluating packageRules for a dep.  Returns `None` for
/// managers that don't have a single well-known datasource.
///
/// Renovate reference: each manager's `defaultConfig.datasource` in
/// `lib/modules/manager/*/index.ts`.
pub fn manager_default_datasource(manager_name: &str) -> Option<&'static str> {
    match manager_name {
        "cargo" => Some("crate"),
        "npm" | "bun" | "meteor" | "mint" => Some("npm"),
        "pip_requirements" | "pip-compile" | "pip_setup" | "pipenv" | "poetry" | "pep621"
        | "pep723" | "setup-cfg" => Some("pypi"),
        "maven" | "maven-wrapper" | "ant" | "sbt" | "leiningen" | "kotlin-script" => Some("maven"),
        "gradle" | "gradle-wrapper" => Some("maven"),
        "gomod" => Some("go"),
        "bundler" | "gemspec" => Some("rubygems"),
        "composer" => Some("packagist"),
        "nuget" => Some("nuget"),
        "pub" => Some("dart"),
        "dockerfile" | "docker-compose" | "devcontainer" | "quadlet" | "batect" => Some("docker"),
        "github-actions" => Some("github-tags"),
        "terraform" | "terragrunt" => Some("terraform"),
        "helm-requirements" | "helm-values" | "helmfile" => Some("helm"),
        "cabal" => Some("hackage"),
        "cpanfile" => Some("cpan"),
        "hex" | "mix" => Some("hex"),
        "spm" | "xcodegen" => Some("github-tags"),
        "bazel-module" => Some("bazel"),
        _ => None,
    }
}

/// Return the default registry URL(s) for a manager/datasource combination.
///
/// Used to populate `DepContext.registry_urls` so `matchRegistryUrls` rules
/// fire correctly when the dep doesn't specify a custom registry.
///
/// Renovate reference: each datasource's `defaultRegistryUrls` in
/// `lib/modules/datasource/*/index.ts`.
pub fn manager_default_registry_urls(manager_name: &str) -> &'static [&'static str] {
    match manager_name {
        "npm" | "bun" | "meteor" | "mint" => &["https://registry.npmjs.org"],
        "pip_requirements" | "pip-compile" | "pip_setup" | "pipenv" | "poetry" | "pep621"
        | "pep723" | "setup-cfg" => &["https://pypi.org/simple/"],
        "cargo" => &["https://crates.io/"],
        "maven" | "maven-wrapper" | "ant" | "sbt" | "leiningen" | "gradle" | "gradle-wrapper"
        | "kotlin-script" => &["https://repo.maven.apache.org/maven2/"],
        "bundler" | "gemspec" => &["https://rubygems.org/"],
        "composer" => &["https://packagist.org/"],
        "nuget" => &["https://api.nuget.org/v3/index.json"],
        "pub" => &["https://pub.dev/"],
        "cabal" => &["https://hackage.haskell.org/"],
        "hex" | "mix" => &["https://hex.pm/"],
        _ => &[],
    }
}

/// Pre-compiled manager patterns.  Compiled once at first use via
/// `LazyLock` — avoids re-compilation on every `detect()` call.
static COMPILED: LazyLock<Vec<(&'static str, Vec<Regex>)>> = LazyLock::new(|| {
    MANAGER_DEFS
        .iter()
        .filter_map(|def| {
            let compiled: Vec<Regex> = def
                .patterns
                .iter()
                .filter_map(|pat| {
                    Regex::new(pat)
                        .map_err(|e| {
                            // Programmer error: a pattern in the static table
                            // is invalid.  Log and skip the manager.
                            tracing::error!(
                                manager = def.name,
                                pattern = pat,
                                %e,
                                "invalid manager pattern (bug in pattern definition)"
                            );
                        })
                        .ok()
                })
                .collect();
            if compiled.len() == def.patterns.len() {
                Some((def.name, compiled))
            } else {
                None
            }
        })
        .collect()
});

/// The initial set of supported manager definitions, ported from upstream
/// `managerFilePatterns` entries. Coverage grows with each parity slice.
///
/// Sources (all from `lib/modules/manager/*/index.ts`):
/// - cargo:          `/(^|/)Cargo\\.toml$/`
/// - npm:            `/(^|/)package\\.json$/`, `/(^|/)pnpm-workspace\\.yaml$/`, `/(^|/)\\.yarnrc\\.yml$/`
/// - pip_requirements: `/(^|/)[\\w-]*requirements([-._]\\w+)?\\.(txt|pip)$/`
/// - pep621:         `/(^|/)pyproject\\.toml$/`
/// - maven:          `/(^|/|\\.)pom\\.xml$/`, `/^(((\\.mvn)|(\\.m2))/)?settings\\.xml$/`
/// - github-actions: `/(^|/)(workflow-templates|\\.(?:github|gitea|forgejo)/(?:workflows|actions))/.+\\.ya?ml$/`, `/(^|/)action\\.ya?ml$/`
/// - dockerfile:     `/(^|/)(Dockerfile|Containerfile)(\\.[^/]*)?$/`
/// - docker-compose: `/(^|/)(?:docker-)?compose\\.ya?ml$/`
const MANAGER_DEFS: &[ManagerDef] = &[
    ManagerDef {
        name: "ansible",
        patterns: &[r"(^|/)tasks/[^/]+\.ya?ml$"],
    },
    ManagerDef {
        name: "ant",
        patterns: &[r"(^|/)build\.xml$"],
    },
    ManagerDef {
        name: "batect",
        patterns: &[r"(^|/)batect(-bundle)?\.ya?ml$"],
    },
    ManagerDef {
        name: "bicep",
        patterns: &[r"\.bicep$"],
    },
    ManagerDef {
        name: "batect-wrapper",
        patterns: &[r"(^|/)batect$"],
    },
    ManagerDef {
        name: "copier",
        patterns: &[r"(^|/)\.copier-answers(\..+)?\.ya?ml$"],
    },
    ManagerDef {
        name: "haskell-cabal",
        patterns: &[r"\.cabal$"],
    },
    ManagerDef {
        name: "fvm",
        patterns: &[r"(^|/)\.fvm/fvm_config\.json$", r"(^|/)\.fvmrc$"],
    },
    ManagerDef {
        name: "jsonnet-bundler",
        patterns: &[r"(^|/)jsonnetfile\.json$"],
    },
    ManagerDef {
        name: "vendir",
        patterns: &[r"(^|/)vendir\.yml$"],
    },
    ManagerDef {
        name: "crow",
        patterns: &[r"^\.crow(?:/[^/]+)?\.ya?ml$"],
    },
    ManagerDef {
        name: "devbox",
        patterns: &[r"(^|/)devbox\.json$"],
    },
    ManagerDef {
        name: "devcontainer",
        patterns: &[
            r"^\.devcontainer/devcontainer\.json$",
            r"^\.devcontainer\.json$",
        ],
    },
    ManagerDef {
        name: "azure-pipelines",
        patterns: &[
            r"(^|/)\.azuredevops/.+\.ya?ml$",
            r"azure.*pipelines?.*\.ya?ml$",
        ],
    },
    ManagerDef {
        name: "bitbucket-pipelines",
        patterns: &[r".*-pipelines\.yml$"],
    },
    ManagerDef {
        name: "bundler",
        patterns: &[r"(^|/)Gemfile$"],
    },
    ManagerDef {
        name: "gemspec",
        patterns: &[r"(^|/)[^/]*\.gemspec$"],
    },
    ManagerDef {
        name: "cocoapods",
        patterns: &[r"(^|/)Podfile$"],
    },
    ManagerDef {
        name: "mix",
        patterns: &[r"(^|/)mix\.exs$"],
    },
    ManagerDef {
        name: "swift",
        patterns: &[r"(^|/)Package\.swift$"],
    },
    ManagerDef {
        name: "gradle",
        patterns: &[
            r"\.gradle(\.kts)?$",
            r"(^|/)gradle\.properties$",
            r"\.versions\.toml$",
        ],
    },
    ManagerDef {
        name: "gradle-wrapper",
        patterns: &[r"(^|/)gradle/wrapper/gradle-wrapper\.properties$"],
    },
    ManagerDef {
        name: "helm-values",
        patterns: &[r"(^|/)values\.ya?ml$"],
    },
    ManagerDef {
        name: "helmfile",
        patterns: &[
            r"(^|/)helmfile\.ya?ml(?:\.gotmpl)?$",
            r"(^|/)helmfile\.d/.+\.ya?ml(?:\.gotmpl)?$",
        ],
    },
    ManagerDef {
        name: "helmv3",
        patterns: &[r"(^|/)Chart\.ya?ml$", r"(^|/)requirements\.ya?ml$"],
    },
    ManagerDef {
        name: "html",
        patterns: &[r"\.html?$"],
    },
    ManagerDef {
        name: "homeassistant-manifest",
        patterns: &[r"(^|/)manifest\.json$"],
    },
    ManagerDef {
        name: "jenkins",
        patterns: &[r"(^|/)plugins\.(txt|ya?ml)$"],
    },
    ManagerDef {
        name: "kotlin-script",
        patterns: &[r"^.+\.main\.kts$"],
    },
    ManagerDef {
        name: "kustomize",
        patterns: &[r"(^|/)kustomization\.ya?ml$"],
    },
    ManagerDef {
        name: "mint",
        patterns: &[r"(^|/)Mintfile$"],
    },
    ManagerDef {
        name: "nix",
        patterns: &[r"(^|/)flake\.nix$"],
    },
    ManagerDef {
        name: "mise",
        patterns: &[
            r"(^|/)\.?mise(\..*)?\.toml$",
            r"(^|/)\.config/mise(\..*)?\.toml$",
            r"(^|/)\.rtx(\..*)?\.toml$",
        ],
    },
    ManagerDef {
        name: "cloudbuild",
        patterns: &[r"(^|/)cloudbuild\.ya?ml$"],
    },
    ManagerDef {
        name: "droneci",
        patterns: &[r"(^|/)\.drone\.yml$"],
    },
    ManagerDef {
        name: "velaci",
        patterns: &[r"(^|/)\.vela\.ya?ml$"],
    },
    ManagerDef {
        name: "woodpecker",
        patterns: &[r"^\.woodpecker(?:/[^/]+)?\.ya?ml$"],
    },
    ManagerDef {
        name: "xcodegen",
        patterns: &[r"(^|/)project\.yml$"],
    },
    ManagerDef {
        name: "quadlet",
        patterns: &[r".+\.(container|image|volume)$"],
    },
    ManagerDef {
        name: "terraform",
        patterns: &[r"\.tf$", r"\.tofu$"],
    },
    ManagerDef {
        name: "terragrunt",
        patterns: &[r"(^|/)terragrunt\.hcl$"],
    },
    ManagerDef {
        name: "tflint-plugin",
        patterns: &[r"\.tflint\.hcl$"],
    },
    ManagerDef {
        name: "travis",
        patterns: &[r"^\.travis\.ya?ml$"],
    },
    ManagerDef {
        name: "typst",
        patterns: &[r"\.typ$"],
    },
    ManagerDef {
        name: "composer",
        patterns: &[r"(^|/)([\w-]*)composer\.json$"],
    },
    ManagerDef {
        name: "cpanfile",
        patterns: &[r"(^|/)cpanfile$"],
    },
    ManagerDef {
        name: "pub",
        patterns: &[r"(^|/)pubspec\.ya?ml$"],
    },
    ManagerDef {
        name: "nuget",
        patterns: &[r"\.(cs|fs|vb)proj$", r"\.(props|targets)$"],
    },
    ManagerDef {
        name: "osgi",
        patterns: &[r"(^|/)src/main/features/.+\.json$"],
    },
    ManagerDef {
        name: "cargo",
        patterns: &[r"(^|/)Cargo\.toml$"],
    },
    ManagerDef {
        name: "meteor",
        patterns: &[r"(^|/)package\.js$"],
    },
    ManagerDef {
        name: "cake",
        patterns: &[r"\.cake$"],
    },
    ManagerDef {
        name: "conan",
        patterns: &[r"(^|/)conanfile\.(txt|py)$"],
    },
    ManagerDef {
        name: "ruby-version",
        patterns: &[r"(^|/)\.ruby-version$"],
    },
    ManagerDef {
        name: "sbt",
        patterns: &[
            r"\.sbt$",
            r"(^|/)project/[^/]*\.scala$",
            r"(^|/)project/build\.properties$",
        ],
    },
    ManagerDef {
        name: "deps-edn",
        patterns: &[r"(^|/)(?:deps|bb)\.edn$"],
    },
    ManagerDef {
        name: "leiningen",
        patterns: &[r"(^|/)project\.clj$"],
    },
    ManagerDef {
        name: "npm",
        patterns: &[
            r"(^|/)package\.json$",
            r"(^|/)pnpm-workspace\.yaml$",
            r"(^|/)\.yarnrc\.yml$",
        ],
    },
    ManagerDef {
        name: "pip_requirements",
        patterns: &[r"(^|/)[\w-]*requirements([-._]\w+)?\.(txt|pip)$"],
    },
    ManagerDef {
        name: "pip_setup",
        patterns: &[r"(^|/)setup\.py$"],
    },
    ManagerDef {
        name: "scalafmt",
        patterns: &[r"(^|/)\.scalafmt\.conf$"],
    },
    ManagerDef {
        name: "setup-cfg",
        patterns: &[r"(^|/)setup\.cfg$"],
    },
    ManagerDef {
        name: "pipenv",
        patterns: &[r"(^|/)Pipfile$"],
    },
    ManagerDef {
        name: "pre-commit",
        patterns: &[r"(^|/)\.pre-commit-config\.ya?ml$"],
    },
    ManagerDef {
        name: "puppet",
        patterns: &[r"(^|/)Puppetfile$"],
    },
    ManagerDef {
        name: "ansible-galaxy",
        patterns: &[r"(^|/)(galaxy|requirements)(\.ansible)?\.ya?ml$"],
    },
    ManagerDef {
        name: "asdf",
        patterns: &[r"(^|/)\.tool-versions$"],
    },
    ManagerDef {
        name: "terraform-version",
        patterns: &[r"(^|/)\.terraform-version$"],
    },
    ManagerDef {
        name: "terragrunt-version",
        patterns: &[r"(^|/)\.terragrunt-version$"],
    },
    ManagerDef {
        name: "go-version",
        patterns: &[r"(^|/)\.go-version$"],
    },
    ManagerDef {
        name: "python-version",
        patterns: &[r"(^|/)\.python-version$"],
    },
    ManagerDef {
        name: "node-version",
        patterns: &[r"(^|/)\.node-version$"],
    },
    ManagerDef {
        name: "nvmrc",
        patterns: &[r"(^|/)\.nvmrc$"],
    },
    ManagerDef {
        name: "bun-version",
        patterns: &[r"(^|/)\.bun-version$"],
    },
    ManagerDef {
        name: "bazel-module",
        patterns: &[r"(^|/|\.)MODULE\.bazel$"],
    },
    ManagerDef {
        name: "bazelisk",
        patterns: &[r"(^|/)\.bazelversion$"],
    },
    ManagerDef {
        name: "gitlabci",
        patterns: &[r"(^|/)\.gitlab-ci\.ya?ml$"],
    },
    ManagerDef {
        name: "gitlabci-include",
        patterns: &[r"(^|/)\.gitlab-ci\.ya?ml$"],
    },
    ManagerDef {
        name: "circleci",
        patterns: &[r"(^|/)\.circleci/.+\.ya?ml$"],
    },
    ManagerDef {
        name: "buildkite",
        patterns: &[r"buildkite\.ya?ml", r"(^|/)\.buildkite/.+\.ya?ml$"],
    },
    ManagerDef {
        name: "pep621",
        patterns: &[r"(^|/)pyproject\.toml$"],
    },
    ManagerDef {
        name: "poetry",
        patterns: &[r"(^|/)pyproject\.toml$"],
    },
    ManagerDef {
        name: "gleam",
        patterns: &[r"(^|/)gleam\.toml$"],
    },
    ManagerDef {
        name: "gomod",
        patterns: &[r"(^|/)go\.mod$"],
    },
    ManagerDef {
        name: "maven",
        patterns: &[r"(^|/|\.)(pom\.xml)$", r"^((\.mvn|\.m2)/)?settings\.xml$"],
    },
    ManagerDef {
        name: "maven-wrapper",
        patterns: &[r"(^|/)\.mvn/wrapper/maven-wrapper\.properties$"],
    },
    ManagerDef {
        name: "fleet",
        patterns: &[r"(^|/)fleet\.ya?ml"],
    },
    ManagerDef {
        name: "flux",
        patterns: &[r"(^|/)gotk-components\.ya?ml$"],
    },
    ManagerDef {
        name: "github-actions",
        patterns: &[
            r"(^|/)(workflow-templates|\.(?:github|gitea|forgejo)/(?:workflows|actions))/.+\.ya?ml$",
            r"(^|/)action\.ya?ml$",
        ],
    },
    ManagerDef {
        name: "dockerfile",
        patterns: &[r"(^|/)(Dockerfile|Containerfile)(\.[^/]*)?$"],
    },
    ManagerDef {
        name: "docker-compose",
        patterns: &[r"(^|/)(?:docker-)?compose\.ya?ml$"],
    },
    ManagerDef {
        name: "homebrew",
        patterns: &[r"(^|/)Formula/[^/]+\.rb$"],
    },
    ManagerDef {
        name: "bitrise",
        patterns: &[r"(^|/)bitrise\.ya?ml$"],
    },
    ManagerDef {
        name: "pixi",
        patterns: &[r"(^|/)pixi\.toml$"],
    },
    ManagerDef {
        name: "unity3d",
        patterns: &[r"(^|/)ProjectSettings/ProjectVersion\.txt$"],
    },
    ManagerDef {
        name: "buildpacks",
        patterns: &[r"(^|/)project\.toml$"],
    },
    ManagerDef {
        name: "helmsman",
        patterns: &[r"(^|/)helmsman\.ya?ml$", r"(^|/)helmsman\.d/.+\.ya?ml$"],
    },
    ManagerDef {
        name: "runtime-version",
        patterns: &[r"(^|/)runtime\.txt$"],
    },
    ManagerDef {
        // Bun lockfile presence signals that bun manages this project's packages.
        // Dep extraction happens from the sibling package.json (see pipeline).
        name: "bun",
        patterns: &[r"(^|/)bun\.lockb?$"],
    },
    ManagerDef {
        name: "nodenv",
        patterns: &[r"(^|/)\.node-version$"],
    },
    ManagerDef {
        name: "nvm",
        patterns: &[r"(^|/)\.nvmrc$"],
    },
    ManagerDef {
        name: "pyenv",
        patterns: &[r"(^|/)\.python-version$"],
    },
    ManagerDef {
        // Common convention: ArgoCD Applications stored in argocd/ directory.
        // Upstream default is [] (user-configured); we add a practical common pattern.
        name: "argocd",
        patterns: &[r"(^|/)argocd/.+\.ya?ml$", r"(^|/)argo-cd/.+\.ya?ml$"],
    },
    ManagerDef {
        // Upstream default is [] — user must configure patterns.
        // Common conventions for K8s manifests.
        name: "kubernetes",
        patterns: &[
            r"(^|/)k8s/.+\.ya?ml$",
            r"(^|/)kubernetes/.+\.ya?ml$",
            r"(^|/)manifests/.+\.ya?ml$",
        ],
    },
    ManagerDef {
        // Tekton tasks/pipelines stored in tekton/ directory.
        name: "tekton",
        patterns: &[r"(^|/)tekton/.+\.ya?ml$"],
    },
    ManagerDef {
        name: "bazel",
        patterns: &[
            r"(^|/)WORKSPACE(\.bazel|\.bzlmod)?$",
            r"(^|/)\.WORKSPACE\.bazel$",
            r"\.bzl$",
        ],
    },
    ManagerDef {
        // Crossplane packages stored in crossplane/ directory convention.
        name: "crossplane",
        patterns: &[r"(^|/)crossplane/.+\.ya?ml$"],
    },
    ManagerDef {
        name: "glasskube",
        patterns: &[r"(^|/)glasskube/.+\.ya?ml$"],
    },
    ManagerDef {
        name: "renovate-config-presets",
        patterns: &[
            r"(^|/)renovate\.json5?$",
            r"(^|/)\.renovaterc(\.json5?)?$",
            r"(^|/)\.github/renovate\.json5?$",
            r"(^|/)\.gitlab/renovate\.json5?$",
        ],
    },
    ManagerDef {
        // helm-requirements handles Helm v2 requirements.yaml; already covered by
        // helmv3 extractor, but registered here so users can reference this name.
        name: "helm-requirements",
        patterns: &[r"(^|/)requirements\.ya?ml$"],
    },
    ManagerDef {
        // Sveltos ClusterProfile/Profile manifests stored in sveltos/ directory.
        name: "sveltos",
        patterns: &[r"(^|/)sveltos/.+\.ya?ml$"],
    },
    ManagerDef {
        // OpenTelemetry Collector Builder config files.
        name: "ocb",
        patterns: &[r"(^|/)otelcol-builder\.ya?ml$", r"(^|/)ocb\.ya?ml$"],
    },
    ManagerDef {
        // PEP 723 inline script metadata — upstream has empty managerFilePatterns;
        // we register a common `.py` convention so detection works out of the box.
        name: "pep723",
        patterns: &[r"(^|/)scripts?/[^/]+\.py$", r"(^|/)[^/]+\.script\.py$"],
    },
    ManagerDef {
        // cdnurl — Cloudflare CDN URL extractor; upstream has empty patterns (user-configured).
        // Uses the same cloudflare URL regex as the html manager but without SRI hash updates.
        name: "cdnurl",
        patterns: &[],
    },
    ManagerDef {
        // git-submodules — upstream patterns: `/(^|/)\.gitmodules$/`.
        // Stub registration; full Git-ref lookup requires local git operations (deferred).
        name: "git-submodules",
        patterns: &[r"(^|/)\.gitmodules$"],
    },
    ManagerDef {
        // hermit — reads `.*.pkg` files in the bin/ directory.
        // Stub registration; full extraction requires directory listing (deferred).
        name: "hermit",
        patterns: &[r"(^|/)bin/\.?hermit\.hcl$"],
    },
    ManagerDef {
        // pip-compile — delegates to pip_requirements/pep621/pip_setup managers.
        // Stub registration; full multi-file extraction deferred.
        name: "pip-compile",
        patterns: &[r"(^|/)requirements\.in$", r"(^|/)requirements-.*\.in$"],
    },
    ManagerDef {
        // custom — user-defined regex/jsonpath patterns; runtime-configured only.
        // Stub registration so the manager name is valid in config references.
        name: "custom",
        patterns: &[],
    },
];

/// Detect which package managers are present in the repository.
///
/// Uses pre-compiled regex patterns (compiled once via [`COMPILED`]).
/// Managers with at least one matching file are included in the result.
pub fn detect(files: &[String]) -> Vec<DetectedManager> {
    let mut results = Vec::new();

    for (name, patterns) in COMPILED.iter() {
        let matched: Vec<String> = files
            .iter()
            .filter(|f| patterns.iter().any(|re| re.is_match(f)))
            .cloned()
            .collect();

        if !matched.is_empty() {
            results.push(DetectedManager {
                name,
                matched_files: matched,
            });
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn detects_cargo() {
        let f = files(&["Cargo.toml", "src/main.rs", "crates/foo/Cargo.toml"]);
        let result = detect(&f);
        let cargo = result.iter().find(|m| m.name == "cargo").unwrap();
        assert_eq!(
            cargo.matched_files,
            vec!["Cargo.toml", "crates/foo/Cargo.toml"]
        );
    }

    #[test]
    fn detects_npm_package_json() {
        let f = files(&["package.json", "frontend/package.json", "README.md"]);
        let result = detect(&f);
        let npm = result.iter().find(|m| m.name == "npm").unwrap();
        assert!(npm.matched_files.contains(&"package.json".to_owned()));
        assert!(
            npm.matched_files
                .contains(&"frontend/package.json".to_owned())
        );
    }

    #[test]
    fn detects_pip_requirements() {
        let f = files(&["requirements.txt", "requirements-dev.txt", "src/setup.py"]);
        let result = detect(&f);
        let pip = result
            .iter()
            .find(|m| m.name == "pip_requirements")
            .unwrap();
        assert!(pip.matched_files.contains(&"requirements.txt".to_owned()));
        assert!(
            pip.matched_files
                .contains(&"requirements-dev.txt".to_owned())
        );
        // setup.py should NOT match
        assert!(!pip.matched_files.contains(&"src/setup.py".to_owned()));
    }

    #[test]
    fn detects_github_actions_workflow() {
        let f = files(&[
            ".github/workflows/ci.yml",
            ".github/workflows/deploy.yaml",
            "README.md",
        ]);
        let result = detect(&f);
        let ga = result.iter().find(|m| m.name == "github-actions").unwrap();
        assert_eq!(ga.matched_files.len(), 2);
    }

    #[test]
    fn detects_dockerfile() {
        let f = files(&["Dockerfile", "docker/Dockerfile.prod", "src/main.rs"]);
        let result = detect(&f);
        let df = result.iter().find(|m| m.name == "dockerfile").unwrap();
        assert!(df.matched_files.contains(&"Dockerfile".to_owned()));
    }

    #[test]
    fn detects_docker_compose() {
        let f = files(&["docker-compose.yml", "compose.yaml"]);
        let result = detect(&f);
        let dc = result.iter().find(|m| m.name == "docker-compose").unwrap();
        assert_eq!(dc.matched_files.len(), 2);
    }

    #[test]
    fn detects_maven_pom() {
        let f = files(&["pom.xml", "module/pom.xml", "parent.pom.xml"]);
        let result = detect(&f);
        let maven = result.iter().find(|m| m.name == "maven").unwrap();
        assert!(maven.matched_files.contains(&"pom.xml".to_owned()));
        assert!(maven.matched_files.contains(&"module/pom.xml".to_owned()));
        assert!(maven.matched_files.contains(&"parent.pom.xml".to_owned()));
    }

    #[test]
    fn empty_file_list_returns_no_managers() {
        assert!(detect(&[]).is_empty());
    }

    #[test]
    fn unrelated_files_return_no_managers() {
        let f = files(&["README.md", "LICENSE", "src/lib.rs"]);
        // .rs files don't match any manager pattern
        let result = detect(&f);
        assert!(!result.iter().any(|m| m.name == "cargo"));
    }

    #[test]
    fn detects_multiple_managers_in_same_repo() {
        let f = files(&["Cargo.toml", "package.json", ".github/workflows/ci.yml"]);
        let result = detect(&f);
        assert!(result.iter().any(|m| m.name == "cargo"));
        assert!(result.iter().any(|m| m.name == "npm"));
        assert!(result.iter().any(|m| m.name == "github-actions"));
    }

    #[test]
    fn manager_default_datasource_known_managers() {
        assert_eq!(manager_default_datasource("cargo"), Some("crate"));
        assert_eq!(manager_default_datasource("npm"), Some("npm"));
        assert_eq!(manager_default_datasource("pip_requirements"), Some("pypi"));
        assert_eq!(manager_default_datasource("maven"), Some("maven"));
        assert_eq!(manager_default_datasource("gradle"), Some("maven"));
        assert_eq!(manager_default_datasource("dockerfile"), Some("docker"));
        assert_eq!(
            manager_default_datasource("github-actions"),
            Some("github-tags")
        );
        assert_eq!(manager_default_datasource("bundler"), Some("rubygems"));
        assert_eq!(manager_default_datasource("composer"), Some("packagist"));
        assert_eq!(manager_default_datasource("nuget"), Some("nuget"));
    }

    #[test]
    fn manager_default_datasource_unknown_returns_none() {
        assert_eq!(manager_default_datasource("unknown-manager"), None);
        assert_eq!(manager_default_datasource("hermit"), None);
    }
}

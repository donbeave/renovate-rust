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

/// A single manager's detection record: (name, regex patterns, metadata).
///
/// Patterns are the inner regex strings from Renovate's `managerFilePatterns`.
struct ManagerDef {
    name: &'static str,
    patterns: &'static [&'static str],
    #[allow(dead_code)]
    supported_datasources: &'static [&'static str],
    #[allow(dead_code)]
    categories: &'static [&'static str],
    #[allow(dead_code)]
    url: Option<&'static str>,
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
        "cargo" | "rust-toolchain" => &["rust"],
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
        supported_datasources: &["docker"],
        categories: &["ansible", "iac"],
        url: Some("https://docs.ansible.com"),
    },
    ManagerDef {
        name: "ant",
        patterns: &[r"(^|/)build\.xml$"],
        supported_datasources: &["maven"],
        categories: &["java"],
        url: Some("https://ant.apache.org"),
    },
    ManagerDef {
        name: "batect",
        patterns: &[r"(^|/)batect(-bundle)?\.ya?ml$"],
        supported_datasources: &["docker", "git-tags"],
        categories: &["batect"],
        url: Some("https://batect.dev/docs"),
    },
    ManagerDef {
        name: "bicep",
        patterns: &[r"\.bicep$"],
        supported_datasources: &["azure-bicep-resource"],
        categories: &["iac"],
        url: Some("https://docs.microsoft.com/azure/azure-resource-manager/bicep/overview"),
    },
    ManagerDef {
        name: "batect-wrapper",
        patterns: &[r"(^|/)batect$"],
        supported_datasources: &["github-releases"],
        categories: &["batect"],
        url: None,
    },
    ManagerDef {
        name: "copier",
        patterns: &[r"(^|/)\.copier-answers(\..+)?\.ya?ml$"],
        supported_datasources: &["git-tags"],
        categories: &["python"],
        url: Some("https://copier.readthedocs.io"),
    },
    ManagerDef {
        name: "haskell-cabal",
        patterns: &[r"\.cabal$"],
        supported_datasources: &["hackage"],
        categories: &["haskell"],
        url: None,
    },
    ManagerDef {
        name: "fvm",
        patterns: &[r"(^|/)\.fvm/fvm_config\.json$", r"(^|/)\.fvmrc$"],
        supported_datasources: &["flutter-version"],
        categories: &["dart"],
        url: Some("https://fvm.app"),
    },
    ManagerDef {
        name: "jsonnet-bundler",
        patterns: &[r"(^|/)jsonnetfile\.json$"],
        supported_datasources: &["git-tags"],
        categories: &["kubernetes"],
        url: Some("https://github.com/jsonnet-bundler/jsonnet-bundler#readme"),
    },
    ManagerDef {
        name: "vendir",
        patterns: &[r"(^|/)vendir\.yml$"],
        supported_datasources: &["helm", "docker"],
        categories: &["helm", "kubernetes"],
        url: Some("https://carvel.dev/vendir/docs/latest"),
    },
    ManagerDef {
        name: "crow",
        patterns: &[r"^\.crow(?:/[^/]+)?\.ya?ml$"],
        supported_datasources: &["docker"],
        categories: &["ci"],
        url: Some("https://crowci.dev"),
    },
    ManagerDef {
        name: "devbox",
        patterns: &[r"(^|/)devbox\.json$"],
        supported_datasources: &["devbox"],
        categories: &[],
        url: None,
    },
    ManagerDef {
        name: "devcontainer",
        patterns: &[
            r"^\.devcontainer/devcontainer\.json$",
            r"^\.devcontainer\.json$",
        ],
        supported_datasources: &[
            "docker",
            "golang-version",
            "node-version",
            "python-version",
            "ruby-version",
        ],
        categories: &["docker"],
        url: Some("https://code.visualstudio.com/docs/devcontainers/containers"),
    },
    ManagerDef {
        name: "azure-pipelines",
        patterns: &[
            r"(^|/)\.azuredevops/.+\.ya?ml$",
            r"azure.*pipelines?.*\.ya?ml$",
        ],
        supported_datasources: &["azure-pipelines-tasks", "git-tags"],
        categories: &["ci"],
        url: Some("https://learn.microsoft.com/azure/devops/pipelines"),
    },
    ManagerDef {
        name: "bitbucket-pipelines",
        patterns: &[r".*-pipelines\.yml$"],
        supported_datasources: &["docker"],
        categories: &["ci"],
        url: Some(
            "https://support.atlassian.com/bitbucket-cloud/docs/get-started-with-bitbucket-pipelines",
        ),
    },
    ManagerDef {
        name: "bundler",
        patterns: &[r"(^|/)Gemfile$"],
        supported_datasources: &["rubygems", "ruby-version"],
        categories: &["ruby"],
        url: Some("https://bundler.io/docs.html"),
    },
    ManagerDef {
        name: "gemspec",
        patterns: &[r"(^|/)[^/]*\.gemspec$"],
        supported_datasources: &["rubygems"],
        categories: &["ruby"],
        url: None,
    },
    ManagerDef {
        name: "cocoapods",
        patterns: &[r"(^|/)Podfile$"],
        supported_datasources: &["git-tags", "github-tags", "gitlab-tags", "pod"],
        categories: &["swift"],
        url: Some("https://cocoapods.org"),
    },
    ManagerDef {
        name: "mix",
        patterns: &[r"(^|/)mix\.exs$"],
        supported_datasources: &["github-tags", "git-tags", "hex"],
        categories: &["elixir"],
        url: Some("https://hexdocs.pm/mix/Mix.html"),
    },
    ManagerDef {
        name: "swift",
        patterns: &[r"(^|/)Package\.swift$"],
        supported_datasources: &["git-tags", "github-tags", "gitlab-tags"],
        categories: &["swift"],
        url: Some("https://www.swift.org/package-manager"),
    },
    ManagerDef {
        name: "gradle",
        patterns: &[
            r"\.gradle(\.kts)?$",
            r"(^|/)gradle\.properties$",
            r"\.versions\.toml$",
        ],
        supported_datasources: &["maven"],
        categories: &["java"],
        url: Some("https://docs.gradle.org/current/userguide/getting_started_dep_man.html"),
    },
    ManagerDef {
        name: "gradle-wrapper",
        patterns: &[r"(^|/)gradle/wrapper/gradle-wrapper\.properties$"],
        supported_datasources: &["gradle-version"],
        categories: &["java"],
        url: Some("https://docs.gradle.org/current/userguide/gradle_wrapper.html"),
    },
    ManagerDef {
        name: "helm-values",
        patterns: &[r"(^|/)values\.ya?ml$"],
        supported_datasources: &["docker"],
        categories: &["helm", "kubernetes"],
        url: Some("https://helm.sh/docs/chart_template_guide/values_files"),
    },
    ManagerDef {
        name: "helmfile",
        patterns: &[
            r"(^|/)helmfile\.ya?ml(?:\.gotmpl)?$",
            r"(^|/)helmfile\.d/.+\.ya?ml(?:\.gotmpl)?$",
        ],
        supported_datasources: &["helm", "docker"],
        categories: &["cd", "helm", "kubernetes"],
        url: Some("https://helmfile.readthedocs.io"),
    },
    ManagerDef {
        name: "helmv3",
        patterns: &[r"(^|/)Chart\.ya?ml$", r"(^|/)requirements\.ya?ml$"],
        supported_datasources: &["docker", "helm"],
        categories: &["helm", "kubernetes"],
        url: Some("https://helm.sh/docs"),
    },
    ManagerDef {
        name: "html",
        patterns: &[r"\.html?$"],
        supported_datasources: &["cdnjs"],
        categories: &["cd"],
        url: None,
    },
    ManagerDef {
        name: "homeassistant-manifest",
        patterns: &[r"(^|/)manifest\.json$"],
        supported_datasources: &["pypi", "git-tags"],
        categories: &["python"],
        url: Some(
            "https://developers.home-assistant.io/docs/creating_integration_manifest/#requirements",
        ),
    },
    ManagerDef {
        name: "jenkins",
        patterns: &[r"(^|/)plugins\.(txt|ya?ml)$"],
        supported_datasources: &["jenkins-plugins"],
        categories: &["ci"],
        url: Some("https://www.jenkins.io/doc"),
    },
    ManagerDef {
        name: "kotlin-script",
        patterns: &[r"^.+\.main\.kts$"],
        supported_datasources: &["maven"],
        categories: &["java"],
        url: Some("https://kotlinlang.org/docs/custom-script-deps-tutorial.html"),
    },
    ManagerDef {
        name: "kustomize",
        patterns: &[r"(^|/)kustomization\.ya?ml$"],
        supported_datasources: &["docker", "git-tags", "github-tags", "helm"],
        categories: &["kubernetes"],
        url: Some("https://kubectl.docs.kubernetes.io/references/kustomize"),
    },
    ManagerDef {
        name: "mint",
        patterns: &[r"(^|/)Mintfile$"],
        supported_datasources: &["git-tags"],
        categories: &["swift"],
        url: Some("https://github.com/yonaskolb/Mint#readme"),
    },
    ManagerDef {
        name: "nix",
        patterns: &[r"(^|/)flake\.nix$"],
        supported_datasources: &["git-refs"],
        categories: &[],
        url: Some("https://nix.dev"),
    },
    ManagerDef {
        name: "mise",
        patterns: &[
            r"(^|/)\.?mise(\..*)?\.toml$",
            r"(^|/)\.?mise/config(\..*)?\.toml$",
            r"(^|/)\.config/mise(\..*)?\.toml$",
            r"(^|/)\.config/mise/(mise|config)(\..*)?\.toml$",
            r"(^|/)\.config/mise/conf\.d/[^/]+\.toml$",
            r"(^|/)\.rtx(\..*)?\.toml$",
        ],
        supported_datasources: &[
            "crate",
            "dart-version",
            "docker",
            "dotnet-version",
            "flutter-version",
            "git-refs",
            "git-tags",
            "github-releases",
            "github-tags",
            "go",
            "hexpm-bob",
            "java-version",
            "node-version",
            "npm",
            "nuget",
            "pypi",
            "ruby-version",
            "rubygems",
        ],
        categories: &[],
        url: Some("https://mise.jdx.dev"),
    },
    ManagerDef {
        name: "cloudbuild",
        patterns: &[r"(^|/)cloudbuild\.ya?ml$"],
        supported_datasources: &["docker"],
        categories: &["ci"],
        url: Some("https://cloud.google.com/build/docs"),
    },
    ManagerDef {
        name: "droneci",
        patterns: &[r"(^|/)\.drone\.yml$"],
        supported_datasources: &["docker"],
        categories: &["ci"],
        url: Some("https://docs.drone.io"),
    },
    ManagerDef {
        name: "velaci",
        patterns: &[r"(^|/)\.vela\.ya?ml$"],
        supported_datasources: &["docker"],
        categories: &["ci"],
        url: Some("https://go-vela.github.io/docs"),
    },
    ManagerDef {
        name: "woodpecker",
        patterns: &[r"^\.woodpecker(?:/[^/]+)?\.ya?ml$"],
        supported_datasources: &["docker"],
        categories: &["ci"],
        url: Some("https://woodpecker-ci.org"),
    },
    ManagerDef {
        name: "xcodegen",
        patterns: &[r"(^|/)project\.yml$"],
        supported_datasources: &["git-tags", "github-tags", "gitlab-tags"],
        categories: &["swift"],
        url: Some("https://github.com/yonaskolb/XcodeGen"),
    },
    ManagerDef {
        name: "quadlet",
        patterns: &[r".+\.(container|image|volume)$"],
        supported_datasources: &["docker"],
        categories: &["docker"],
        url: Some("https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html"),
    },
    ManagerDef {
        name: "terraform",
        patterns: &[r"\.tf$", r"\.tofu$"],
        supported_datasources: &[
            "bitbucket-tags",
            "docker",
            "git-tags",
            "github-tags",
            "github-releases",
            "helm",
            "terraform-module",
            "terraform-provider",
        ],
        categories: &["iac", "terraform"],
        url: Some("https://developer.hashicorp.com/terraform/docs"),
    },
    ManagerDef {
        name: "terragrunt",
        patterns: &[r"(^|/)terragrunt\.hcl$"],
        supported_datasources: &[
            "git-tags",
            "github-tags",
            "gitlab-tags",
            "bitbucket-tags",
            "gitea-tags",
            "terraform-module",
        ],
        categories: &["iac", "terraform"],
        url: Some("https://terragrunt.gruntwork.io/docs"),
    },
    ManagerDef {
        name: "tflint-plugin",
        patterns: &[r"\.tflint\.hcl$"],
        supported_datasources: &["github-releases"],
        categories: &["terraform"],
        url: Some(
            "https://github.com/terraform-linters/tflint/blob/master/docs/user-guide/plugins.md",
        ),
    },
    ManagerDef {
        name: "travis",
        patterns: &[r"^\.travis\.ya?ml$"],
        supported_datasources: &["node-version"],
        categories: &["ci"],
        url: Some("https://docs.travis-ci.com"),
    },
    ManagerDef {
        name: "typst",
        patterns: &[r"\.typ$"],
        supported_datasources: &["typst"],
        categories: &[],
        url: None,
    },
    ManagerDef {
        name: "composer",
        patterns: &[r"(^|/)([\w-]*)composer\.json$"],
        supported_datasources: &["bitbucket-tags", "git-tags", "packagist"],
        categories: &["php"],
        url: Some("https://getcomposer.org/doc"),
    },
    ManagerDef {
        name: "cpanfile",
        patterns: &[r"(^|/)cpanfile$"],
        supported_datasources: &["cpan", "github-tags"],
        categories: &["perl"],
        url: Some("https://metacpan.org/dist/Module-CPANfile/view/lib/cpanfile.pod"),
    },
    ManagerDef {
        name: "pub",
        patterns: &[r"(^|/)pubspec\.ya?ml$"],
        supported_datasources: &["dart", "dart-version", "flutter-version"],
        categories: &["dart"],
        url: Some("https://dart.dev/tools/pub/packages"),
    },
    ManagerDef {
        name: "nuget",
        patterns: &[r"\.(cs|fs|vb)proj$", r"\.(props|targets)$"],
        supported_datasources: &["docker", "dotnet-version", "nuget"],
        categories: &["dotnet"],
        url: Some("https://learn.microsoft.com/nuget"),
    },
    ManagerDef {
        name: "osgi",
        patterns: &[r"(^|/)src/main/features/.+\.json$"],
        supported_datasources: &["maven"],
        categories: &["java"],
        url: None,
    },
    ManagerDef {
        name: "cargo",
        patterns: &[r"(^|/)Cargo\.toml$"],
        supported_datasources: &[
            "crate",
            "github-tags",
            "gitlab-tags",
            "git-refs",
            "git-tags",
        ],
        categories: &["rust"],
        url: Some("https://doc.rust-lang.org/cargo"),
    },
    ManagerDef {
        name: "meteor",
        patterns: &[r"(^|/)package\.js$"],
        supported_datasources: &["npm"],
        categories: &["js"],
        url: Some("https://docs.meteor.com"),
    },
    ManagerDef {
        name: "cake",
        patterns: &[r"\.cake$"],
        supported_datasources: &["nuget"],
        categories: &["dotnet"],
        url: Some("https://cakebuild.net/docs"),
    },
    ManagerDef {
        name: "conan",
        patterns: &[r"(^|/)conanfile\.(txt|py)$"],
        supported_datasources: &["conan"],
        categories: &["c"],
        url: Some("https://docs.conan.io"),
    },
    ManagerDef {
        name: "ruby-version",
        patterns: &[r"(^|/)\.ruby-version$"],
        supported_datasources: &["ruby-version"],
        categories: &["ruby"],
        url: None,
    },
    ManagerDef {
        name: "sbt",
        patterns: &[
            r"\.sbt$",
            r"(^|/)project/[^/]*\.scala$",
            r"(^|/)project/build\.properties$",
        ],
        supported_datasources: &["maven", "sbt-package", "sbt-plugin", "github-releases"],
        categories: &["java"],
        url: Some("https://www.scala-sbt.org"),
    },
    ManagerDef {
        name: "deps-edn",
        patterns: &[r"(^|/)(?:deps|bb)\.edn$"],
        supported_datasources: &["clojure"],
        categories: &["java"],
        url: Some("https://clojure.org/reference/deps_edn"),
    },
    ManagerDef {
        name: "leiningen",
        patterns: &[r"(^|/)project\.clj$"],
        supported_datasources: &["clojure"],
        categories: &["java"],
        url: Some("https://leiningen.org"),
    },
    ManagerDef {
        name: "npm",
        patterns: &[
            r"(^|/)package\.json$",
            r"(^|/)pnpm-workspace\.yaml$",
            r"(^|/)\.yarnrc\.yml$",
        ],
        supported_datasources: &["github-tags", "npm", "node-version"],
        categories: &["js"],
        url: Some("https://docs.npmjs.com"),
    },
    ManagerDef {
        name: "pip_requirements",
        patterns: &[r"(^|/)[\w-]*requirements([-._]\w+)?\.(txt|pip)$"],
        supported_datasources: &["pypi", "git-tags"],
        categories: &["python"],
        url: Some("https://pip.pypa.io/en/stable/reference/requirements-file-format"),
    },
    ManagerDef {
        name: "pip_setup",
        patterns: &[r"(^|/)setup\.py$"],
        supported_datasources: &["pypi"],
        categories: &["python"],
        url: Some("https://pip.pypa.io/en/latest/reference/build-system/setup-py"),
    },
    ManagerDef {
        name: "scalafmt",
        patterns: &[r"(^|/)\.scalafmt\.conf$"],
        supported_datasources: &["github-releases"],
        categories: &["java"],
        url: Some("https://scalameta.org/scalafmt/docs/configuration.html#version"),
    },
    ManagerDef {
        name: "setup-cfg",
        patterns: &[r"(^|/)setup\.cfg$"],
        supported_datasources: &["pypi"],
        categories: &["python"],
        url: Some("https://setuptools.pypa.io/en/latest/userguide/declarative_config.html"),
    },
    ManagerDef {
        name: "pipenv",
        patterns: &[r"(^|/)Pipfile$"],
        supported_datasources: &["pypi"],
        categories: &["python"],
        url: Some("https://pipenv.pypa.io/en/latest"),
    },
    ManagerDef {
        name: "pre-commit",
        patterns: &[r"(^|/)\.pre-commit-config\.ya?ml$"],
        supported_datasources: &["github-tags", "gitlab-tags"],
        categories: &["python"],
        url: Some("https://pre-commit.com"),
    },
    ManagerDef {
        name: "puppet",
        patterns: &[r"(^|/)Puppetfile$"],
        supported_datasources: &["puppet-forge", "github-tags", "git-tags"],
        categories: &["iac", "ruby"],
        url: Some("https://www.puppet.com/docs/index.html"),
    },
    ManagerDef {
        name: "ansible-galaxy",
        patterns: &[r"(^|/)(galaxy|requirements)(\.ansible)?\.ya?ml$"],
        supported_datasources: &["galaxy-collection", "git-tags", "github-tags"],
        categories: &["ansible", "iac"],
        url: Some("https://docs.ansible.com/ansible/latest/galaxy/user_guide.html"),
    },
    ManagerDef {
        name: "asdf",
        patterns: &[r"(^|/)\.tool-versions$"],
        supported_datasources: &[
            "dart-version",
            "docker",
            "dotnet-version",
            "flutter-version",
            "github-releases",
            "github-tags",
            "hexpm-bob",
            "java-version",
            "node-version",
            "npm",
            "pypi",
            "ruby-version",
        ],
        categories: &[],
        url: Some("https://asdf-vm.com"),
    },
    ManagerDef {
        name: "terraform-version",
        patterns: &[r"(^|/)\.terraform-version$"],
        supported_datasources: &["github-releases"],
        categories: &["terraform"],
        url: None,
    },
    ManagerDef {
        name: "terragrunt-version",
        patterns: &[r"(^|/)\.terragrunt-version$"],
        supported_datasources: &["github-releases"],
        categories: &["terraform"],
        url: None,
    },
    ManagerDef {
        name: "go-version",
        patterns: &[r"(^|/)\.go-version$"],
        supported_datasources: &["go"],
        categories: &["golang"],
        url: None,
    },
    ManagerDef {
        name: "python-version",
        patterns: &[r"(^|/)\.python-version$"],
        supported_datasources: &["docker"],
        categories: &["python"],
        url: None,
    },
    ManagerDef {
        name: "node-version",
        patterns: &[r"(^|/)\.node-version$"],
        supported_datasources: &["node-version"],
        categories: &["js"],
        url: None,
    },
    ManagerDef {
        name: "nvmrc",
        patterns: &[r"(^|/)\.nvmrc$"],
        supported_datasources: &["node-version"],
        categories: &["js"],
        url: None,
    },
    ManagerDef {
        name: "bun-version",
        patterns: &[r"(^|/)\.bun-version$"],
        supported_datasources: &["npm"],
        categories: &["js"],
        url: None,
    },
    ManagerDef {
        name: "bazel-module",
        patterns: &[r"(^|/|\.)MODULE\.bazel$"],
        supported_datasources: &["bazel", "crate", "docker", "github-tags", "maven"],
        categories: &["bazel"],
        url: Some("https://bazel.build/external/module"),
    },
    ManagerDef {
        name: "bazelisk",
        patterns: &[r"(^|/)\.bazelversion$"],
        supported_datasources: &["github-releases"],
        categories: &["bazel"],
        url: Some("https://github.com/bazelbuild/bazelisk"),
    },
    ManagerDef {
        name: "gitlabci",
        patterns: &[r"(^|/)\.gitlab-ci\.ya?ml$"],
        supported_datasources: &["docker", "gitlab-tags"],
        categories: &["ci"],
        url: Some("https://docs.gitlab.com/ee/ci"),
    },
    ManagerDef {
        name: "gitlabci-include",
        patterns: &[r"(^|/)\.gitlab-ci\.ya?ml$"],
        supported_datasources: &["gitlab-tags"],
        categories: &["ci"],
        url: Some("https://docs.gitlab.com/ee/ci/yaml/includes.html"),
    },
    ManagerDef {
        name: "circleci",
        patterns: &[r"(^|/)\.circleci/.+\.ya?ml$"],
        supported_datasources: &["docker", "orb"],
        categories: &["ci"],
        url: Some("https://circleci.com/docs/configuration-reference"),
    },
    ManagerDef {
        name: "buildkite",
        patterns: &[r"buildkite\.ya?ml", r"(^|/)\.buildkite/.+\.ya?ml$"],
        supported_datasources: &["github-tags", "bitbucket-tags"],
        categories: &["ci"],
        url: Some("https://buildkite.com/docs"),
    },
    ManagerDef {
        name: "pep621",
        patterns: &[r"(^|/)pyproject\.toml$"],
        supported_datasources: &["pypi"],
        categories: &["python"],
        url: Some("https://peps.python.org/pep-0621"),
    },
    ManagerDef {
        name: "poetry",
        patterns: &[r"(^|/)pyproject\.toml$"],
        supported_datasources: &[
            "pypi",
            "github-tags",
            "github-releases",
            "gitlab-tags",
            "git-refs",
            "git-tags",
        ],
        categories: &["python"],
        url: Some("https://python-poetry.org/docs"),
    },
    ManagerDef {
        name: "gleam",
        patterns: &[r"(^|/)gleam\.toml$"],
        supported_datasources: &["hex"],
        categories: &["elixir"],
        url: Some("https://gleam.run/documentation"),
    },
    ManagerDef {
        name: "gomod",
        patterns: &[r"(^|/)go\.mod$"],
        supported_datasources: &["go", "golang-version"],
        categories: &["golang"],
        url: Some("https://go.dev/ref/mod"),
    },
    ManagerDef {
        name: "maven",
        patterns: &[r"(^|/|\.)(pom\.xml)$", r"^((\.mvn|\.m2)/)?settings\.xml$"],
        supported_datasources: &["maven", "docker"],
        categories: &["java"],
        url: Some("https://maven.apache.org"),
    },
    ManagerDef {
        name: "maven-wrapper",
        patterns: &[r"(^|/)\.mvn/wrapper/maven-wrapper\.properties$"],
        supported_datasources: &["maven"],
        categories: &["java"],
        url: Some("https://maven.apache.org/tools/wrapper"),
    },
    ManagerDef {
        name: "fleet",
        patterns: &[r"(^|/)fleet\.ya?ml"],
        supported_datasources: &["git-tags", "helm", "docker"],
        categories: &["cd", "kubernetes"],
        url: Some("https://fleet.rancher.io"),
    },
    ManagerDef {
        name: "flux",
        patterns: &[r"(^|/)gotk-components\.ya?ml$"],
        supported_datasources: &[
            "github-releases",
            "git-refs",
            "github-tags",
            "gitlab-tags",
            "git-tags",
            "bitbucket-tags",
            "helm",
            "docker",
        ],
        categories: &["cd", "kubernetes"],
        url: Some("https://fluxcd.io/flux"),
    },
    ManagerDef {
        name: "github-actions",
        patterns: &[
            r"(^|/)(workflow-templates|\.(?:github|gitea|forgejo)/(?:workflows|actions))/.+\.ya?ml$",
            r"(^|/)action\.ya?ml$",
        ],
        supported_datasources: &[
            "gitea-tags",
            "github-digest",
            "github-runners",
            "github-tags",
        ],
        categories: &["ci"],
        url: Some("https://docs.github.com/en/actions"),
    },
    ManagerDef {
        name: "dockerfile",
        patterns: &[r"(^|/)(Dockerfile|Containerfile)(\.[^/]*)?$"],
        supported_datasources: &["docker"],
        categories: &["docker"],
        url: Some("https://docs.docker.com/build/concepts/dockerfile"),
    },
    ManagerDef {
        name: "docker-compose",
        patterns: &[r"(^|/)(?:docker-)?compose\.ya?ml$"],
        supported_datasources: &["docker"],
        categories: &["docker"],
        url: Some("https://docs.docker.com/compose"),
    },
    ManagerDef {
        name: "homebrew",
        patterns: &[r"(^|/)Formula/[^/]+\.rb$"],
        supported_datasources: &["github-tags", "github-releases", "npm"],
        categories: &[],
        url: Some("https://brew.sh"),
    },
    ManagerDef {
        name: "bitrise",
        patterns: &[r"(^|/)bitrise\.ya?ml$"],
        supported_datasources: &["bitrise", "git-tags"],
        categories: &["ci"],
        url: Some("https://devcenter.bitrise.io"),
    },
    ManagerDef {
        name: "pixi",
        patterns: &[r"(^|/)pixi\.toml$"],
        supported_datasources: &["pypi", "conda"],
        categories: &["python"],
        url: Some("https://github.com/prefix-dev/pixi/"),
    },
    ManagerDef {
        name: "unity3d",
        patterns: &[r"(^|/)ProjectSettings/ProjectVersion\.txt$"],
        supported_datasources: &["unity3d"],
        categories: &["dotnet"],
        url: None,
    },
    ManagerDef {
        name: "buildpacks",
        patterns: &[r"(^|/)project\.toml$"],
        supported_datasources: &["docker", "buildpacks-registry"],
        categories: &["docker", "ci", "cd"],
        url: None,
    },
    ManagerDef {
        name: "helmsman",
        patterns: &[r"(^|/)helmsman\.ya?ml$", r"(^|/)helmsman\.d/.+\.ya?ml$"],
        supported_datasources: &["helm", "docker"],
        categories: &["cd", "helm", "kubernetes"],
        url: Some("https://github.com/Praqma/helmsman#readme"),
    },
    ManagerDef {
        name: "runtime-version",
        patterns: &[r"(^|/)runtime\.txt$"],
        supported_datasources: &["docker"],
        categories: &["python"],
        url: None,
    },
    ManagerDef {
        name: "bun",
        patterns: &[r"(^|/)bun\.lockb?$"],
        supported_datasources: &["github-tags", "npm"],
        categories: &["js"],
        url: Some("https://bun.sh/docs/cli/install"),
    },
    ManagerDef {
        name: "nodenv",
        patterns: &[r"(^|/)\.node-version$"],
        supported_datasources: &["node-version"],
        categories: &["js"],
        url: Some("https://github.com/nodenv/nodenv#readme"),
    },
    ManagerDef {
        name: "nvm",
        patterns: &[r"(^|/)\.nvmrc$"],
        supported_datasources: &["node-version"],
        categories: &["js"],
        url: Some("https://github.com/nvm-sh/nvm#readme"),
    },
    ManagerDef {
        name: "pyenv",
        patterns: &[r"(^|/)\.python-version$"],
        supported_datasources: &["docker"],
        categories: &["python"],
        url: Some("https://github.com/pyenv/pyenv#readme"),
    },
    ManagerDef {
        name: "argocd",
        patterns: &[r"(^|/)argocd/.+\.ya?ml$", r"(^|/)argo-cd/.+\.ya?ml$"],
        supported_datasources: &["docker", "git-tags", "helm"],
        categories: &["kubernetes", "cd"],
        url: Some("https://argo-cd.readthedocs.io"),
    },
    ManagerDef {
        name: "kubernetes",
        patterns: &[
            r"(^|/)k8s/.+\.ya?ml$",
            r"(^|/)kubernetes/.+\.ya?ml$",
            r"(^|/)manifests/.+\.ya?ml$",
        ],
        supported_datasources: &["docker", "kubernetes-api"],
        categories: &["kubernetes"],
        url: Some("https://kubernetes.io/docs"),
    },
    ManagerDef {
        name: "tekton",
        patterns: &[r"(^|/)tekton/.+\.ya?ml$"],
        supported_datasources: &["docker", "git-tags"],
        categories: &["ci", "cd"],
        url: Some("https://tekton.dev/docs"),
    },
    ManagerDef {
        name: "bazel",
        patterns: &[
            r"(^|/)WORKSPACE(\.bazel|\.bzlmod)?$",
            r"(^|/)\.WORKSPACE\.bazel$",
            r"\.bzl$",
        ],
        supported_datasources: &["docker", "github-releases", "github-tags", "go"],
        categories: &["bazel"],
        url: Some("https://bazel.build/docs"),
    },
    ManagerDef {
        name: "crossplane",
        patterns: &[r"(^|/)crossplane/.+\.ya?ml$"],
        supported_datasources: &["docker"],
        categories: &["kubernetes", "iac"],
        url: Some("https://docs.crossplane.io"),
    },
    ManagerDef {
        name: "glasskube",
        patterns: &[r"(^|/)glasskube/.+\.ya?ml$"],
        supported_datasources: &["glasskube-packages"],
        categories: &["kubernetes", "cd"],
        url: Some("https://glasskube.dev/docs"),
    },
    ManagerDef {
        name: "renovate-config-presets",
        patterns: &[
            r"(^|/)renovate\.json5?$",
            r"(^|/)\.renovaterc(\.json5?)?$",
            r"(^|/)\.github/renovate\.json5?$",
            r"(^|/)\.gitlab/renovate\.json5?$",
        ],
        supported_datasources: &["github-tags", "gitlab-tags", "gitea-tags"],
        categories: &[],
        url: None,
    },
    ManagerDef {
        name: "helm-requirements",
        patterns: &[r"(^|/)requirements\.ya?ml$"],
        supported_datasources: &["helm"],
        categories: &["helm", "kubernetes"],
        url: Some("https://v2.helm.sh/docs/developing_charts/#chart-dependencies"),
    },
    ManagerDef {
        name: "sveltos",
        patterns: &[r"(^|/)sveltos/.+\.ya?ml$"],
        supported_datasources: &["docker", "helm"],
        categories: &["kubernetes", "cd"],
        url: Some("https://projectsveltos.github.io/sveltos"),
    },
    ManagerDef {
        name: "ocb",
        patterns: &[r"(^|/)otelcol-builder\.ya?ml$", r"(^|/)ocb\.ya?ml$"],
        supported_datasources: &["go"],
        categories: &["golang"],
        url: Some(
            "https://github.com/open-telemetry/opentelemetry-collector/tree/main/cmd/builder",
        ),
    },
    ManagerDef {
        name: "pep723",
        patterns: &[r"(^|/)scripts?/[^/]+\.py$", r"(^|/)[^/]+\.script\.py$"],
        supported_datasources: &["pypi"],
        categories: &["python"],
        url: Some("https://peps.python.org/pep-0723"),
    },
    ManagerDef {
        name: "proto",
        patterns: &[r"(^|/)\.prototools$"],
        supported_datasources: &[
            "github-releases",
            "github-tags",
            "node-version",
            "npm",
            "ruby-version",
        ],
        categories: &[],
        url: Some("https://moonrepo.dev/proto"),
    },
    ManagerDef {
        name: "cdnurl",
        patterns: &[],
        supported_datasources: &["cdnjs"],
        categories: &["cd"],
        url: None,
    },
    ManagerDef {
        name: "git-submodules",
        patterns: &[r"(^|/)\.gitmodules$"],
        supported_datasources: &["git-refs"],
        categories: &[],
        url: Some("https://git-scm.com/docs/git-submodule"),
    },
    ManagerDef {
        name: "hermit",
        patterns: &[r"(^|/)bin/\.?hermit\.hcl$"],
        supported_datasources: &["hermit"],
        categories: &[],
        url: Some("https://cashapp.github.io/hermit"),
    },
    ManagerDef {
        name: "pip-compile",
        patterns: &[r"(^|/)requirements\.in$", r"(^|/)requirements-.*\.in$"],
        supported_datasources: &["pypi", "git-tags"],
        categories: &["python"],
        url: Some("https://pip-tools.readthedocs.io/en/latest/cli/pip-compile"),
    },
    ManagerDef {
        name: "custom",
        patterns: &[],
        supported_datasources: &[],
        categories: &[],
        url: None,
    },
    ManagerDef {
        name: "rust-toolchain",
        patterns: &[r"(^|/)rust-toolchain(\.toml)?$"],
        supported_datasources: &["rust-version"],
        categories: &["rust"],
        url: Some("https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file"),
    },
    ManagerDef {
        name: "deno",
        patterns: &[r"(^|/)deno\.json[c]?$", r"(^|/)deno\.lock$"],
        supported_datasources: &["npm", "jsr", "deno"],
        categories: &["js"],
        url: Some("https://docs.deno.com/runtime/getting_started/installation/"),
    },
];

/// Detect which package managers are present in the repository.
///
/// List of custom manager identifiers.
///
/// Mirrors `lib/modules/manager/custom/api.ts`.
pub const CUSTOM_MANAGER_LIST: &[&str] = &["regex", "jsonata"];

/// Return `true` if `manager` is a custom manager name.
///
/// Mirrors `lib/modules/manager/custom/index.ts` `isCustomManager()`.
pub fn is_custom_manager(manager: &str) -> bool {
    CUSTOM_MANAGER_LIST.contains(&manager)
}

/// Return all known manager IDs.
pub fn all_manager_ids() -> Vec<&'static str> {
    let mut ids: Vec<&str> = MANAGER_DEFS.iter().map(|m| m.name).collect();
    // Also include custom managers
    for cm in CUSTOM_MANAGER_LIST {
        if !ids.contains(cm) {
            ids.push(cm);
        }
    }
    ids
}

// ═══════════════════════════════════════════════════════════════════════════
// Manager registry API — lib/modules/manager/index.ts
// ═══════════════════════════════════════════════════════════════════════════

/// Return the list of built-in (non-custom) manager names.
/// Mirrors `getManagerList()` from `lib/modules/manager/index.ts`.
pub fn get_manager_list() -> Vec<&'static str> {
    MANAGER_DEFS.iter().map(|m| m.name).collect()
}

/// Return list of all managers (built-in + custom).
/// Mirrors `allManagersList` from `lib/modules/manager/index.ts`.
pub fn all_managers_list() -> Vec<&'static str> {
    let mut ids: Vec<&str> = get_manager_list();
    for cm in CUSTOM_MANAGER_LIST {
        if !ids.contains(cm) {
            ids.push(cm);
        }
    }
    ids
}

/// Return the list of enabled managers.
///
/// When `enabled_managers` is `None` or empty, returns all managers (built-in
/// + custom).  When given a list, normalizes custom-prefixed names
///   (`"custom.regex"` → `"regex"`) and filters to only those that exist, sorted.
///
/// Mirrors `getEnabledManagersList()` from `lib/modules/manager/index.ts`.
pub fn get_enabled_managers_list(enabled_managers: Option<&[String]>) -> Vec<&'static str> {
    let all = all_managers_list();
    let Some(config) = enabled_managers else {
        return all;
    };
    if config.is_empty() {
        return all;
    }
    // Normalize "custom.X" → "X" and filter to known managers.
    let mut result: Vec<&'static str> = config
        .iter()
        .map(|m| m.strip_prefix("custom.").unwrap_or(m.as_str()))
        .filter_map(|name| all.iter().copied().find(|&m| m == name))
        .collect();
    result.sort_unstable();
    result.dedup();
    result
}

/// Detect global config by iterating all managers.
/// Returns an empty map since no built-in manager currently implements
/// `detectGlobalConfig`.
/// Mirrors `detectAllGlobalConfig()` from `lib/modules/manager/index.ts`.
pub fn detect_all_global_config() -> std::collections::HashMap<String, String> {
    std::collections::HashMap::new()
}

/// Return `true` when the named manager exists in the built-in or custom list.
/// Mirrors `managers.has(manager)` from `lib/modules/manager/index.ts`.
pub fn manager_exists(name: &str) -> bool {
    let normalized = name.strip_prefix("custom.").unwrap_or(name);
    MANAGER_DEFS.iter().any(|m| m.name == normalized) || CUSTOM_MANAGER_LIST.contains(&normalized)
}
/// Return the human-readable dep type name for a given manager and depType.
///
/// Mirrors `getPrettyDepType()` from `lib/modules/manager/index.ts`.
/// Only npm dep type metadata is currently implemented.
pub fn get_pretty_dep_type(manager: &str, dep_type: &str) -> Option<&'static str> {
    if !manager_exists(manager) {
        return None;
    }
    // npm dep types — mirrors lib/modules/manager/npm/dep-types.ts
    if matches!(manager, "npm" | "pnpm" | "yarn") {
        return match dep_type {
            "dependencies" => Some("dependency"),
            "devDependencies" => Some("devDependency"),
            "optionalDependencies" => Some("optionalDependency"),
            "peerDependencies" => Some("peerDependency"),
            "engines" => Some("engine"),
            "volta" => Some("volta"),
            "packageManager" => Some("packageManager"),
            "resolutions" => Some("resolution"),
            "overrides" => Some("override"),
            "pnpm.overrides" => Some("pnpmOverride"),
            _ => None,
        };
    }
    None
}

/// Apply a regex repeatedly to content, collecting all non-overlapping matches.
///
/// Mirrors `lib/modules/manager/custom/regex/utils.ts` `regexMatchAll()`.
/// Capped at 10 000 results to guard against runaway lazy patterns.
pub fn regex_match_all(re: &regex::Regex, content: &str) -> Vec<String> {
    re.find_iter(content)
        .take(10_000)
        .map(|m| m.as_str().to_owned())
        .collect()
}

/// Managers that supersede other managers.
///
/// Mirrors `supersedesManagers` from various `lib/modules/manager/*/index.ts`.
const SUPERSEDES_MANAGERS: &[(&str, &[&str])] = &[
    ("bun", &["npm"]),
    ("deno", &["npm"]),
    ("poetry", &["pep621"]),
];

/// Returns the list of managers that the given manager supersedes.
pub fn supersedes_managers(manager: &str) -> &'static [&'static str] {
    SUPERSEDES_MANAGERS
        .iter()
        .find(|(m, _)| *m == manager)
        .map(|(_, s)| *s)
        .unwrap_or(&[])
}

/// A package file entry for `process_supersedes_managers`.
#[derive(Debug, Clone, PartialEq)]
pub struct PackageFileEntry {
    pub package_file: String,
    pub lock_files: Vec<String>,
}

/// An extraction result for `process_supersedes_managers`.
/// `package_files` is `None` to represent "undefined" in TypeScript semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractResult {
    pub manager: String,
    pub package_files: Option<Vec<PackageFileEntry>>,
}

/// Remove package files superseded by a higher-priority manager.
///
/// Mirrors `lib/workers/repository/extract/supersedes.ts`
/// `processSupersedesManagers()`.
pub fn process_supersedes_managers(extracts: &mut [ExtractResult]) {
    let mut rejected: std::collections::HashMap<String, Vec<String>> = Default::default();

    for i in 0..extracts.len() {
        let primary_manager = extracts[i].manager.clone();
        let secondary_managers = supersedes_managers(&primary_manager);
        if secondary_managers.is_empty() {
            continue;
        }

        let Some(ref primary_pkg_files) = extracts[i].package_files else {
            continue;
        };
        let primary_files: Vec<String> = primary_pkg_files
            .iter()
            .map(|f| f.package_file.clone())
            .collect();

        for &secondary_manager in secondary_managers {
            let secondary_idx = extracts.iter().position(|e| e.manager == secondary_manager);
            let Some(sidx) = secondary_idx else { continue };

            let Some(ref secondary_files) = extracts[sidx].package_files.clone() else {
                continue;
            };
            for entry in secondary_files {
                if !entry.lock_files.is_empty() {
                    rejected
                        .entry(primary_manager.clone())
                        .or_default()
                        .push(entry.package_file.clone());
                    continue;
                }
                if primary_files.contains(&entry.package_file) {
                    rejected
                        .entry(secondary_manager.to_owned())
                        .or_default()
                        .push(entry.package_file.clone());
                }
            }
        }
    }

    for extract in extracts.iter_mut() {
        if let Some(rejected_files) = rejected.get(&extract.manager)
            && !rejected_files.is_empty()
            && let Some(ref mut files) = extract.package_files
        {
            files.retain(|f| !rejected_files.contains(&f.package_file));
        }
    }
}

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

// ── file-match (mirrors lib/workers/repository/extract/file-match.ts) ────────

/// Return only files that match any of the `include_paths` (exact or glob).
///
/// Returns all files if `include_paths` is empty.
/// Mirrors `getIncludedFiles` from `lib/workers/repository/extract/file-match.ts`.
pub fn get_included_files<'a>(file_list: &'a [String], include_paths: &[&str]) -> Vec<&'a str> {
    if include_paths.is_empty() {
        return file_list.iter().map(|s| s.as_str()).collect();
    }
    file_list
        .iter()
        .filter(|file| {
            include_paths
                .iter()
                .any(|pattern| file.as_str() == *pattern || glob_matches(pattern, file))
        })
        .map(|s| s.as_str())
        .collect()
}

/// Return files that do NOT match any of the `ignore_paths` (substring or glob).
///
/// Returns all files if `ignore_paths` is empty.
/// Mirrors `filterIgnoredFiles` from `lib/workers/repository/extract/file-match.ts`.
pub fn filter_ignored_files<'a>(file_list: &'a [String], ignore_paths: &[&str]) -> Vec<&'a str> {
    if ignore_paths.is_empty() {
        return file_list.iter().map(|s| s.as_str()).collect();
    }
    file_list
        .iter()
        .filter(|file| {
            !ignore_paths
                .iter()
                .any(|pattern| file.contains(pattern) || glob_matches(pattern, file))
        })
        .map(|s| s.as_str())
        .collect()
}

/// Return files matching any of the `manager_patterns` (regex or glob), with
/// include/ignore filtering applied first.
///
/// Results are deduped and sorted.
/// Mirrors `getMatchingFiles` from `lib/workers/repository/extract/file-match.ts`.
pub fn get_matching_files(
    file_list: &[String],
    include_paths: &[&str],
    ignore_paths: &[&str],
    manager_patterns: &[&str],
) -> Vec<String> {
    let filtered: Vec<&str> = file_list
        .iter()
        .filter(|f| {
            let included = if include_paths.is_empty() {
                true
            } else {
                include_paths
                    .iter()
                    .any(|p| f.as_str() == *p || glob_matches(p, f))
            };
            if !included {
                return false;
            }
            !ignore_paths
                .iter()
                .any(|p| f.contains(p) || glob_matches(p, f))
        })
        .map(|s| s.as_str())
        .collect();

    use crate::string_match::match_regex_or_glob;
    let mut matched: Vec<String> = Vec::new();
    for pattern in manager_patterns {
        for file in &filtered {
            if match_regex_or_glob(file, pattern) {
                matched.push((*file).to_owned());
            }
        }
    }
    // Dedup and sort.
    matched.sort();
    matched.dedup();
    matched
}

/// Case-insensitive glob match using the `globset` crate.
fn glob_matches(pattern: &str, path: &str) -> bool {
    globset::GlobBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .ok()
        .map(|g| g.compile_matcher().is_match(path))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|s| (*s).to_owned()).collect()
    }

    // Rust-specific: unit test for cargo manager detection
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

    // Rust-specific: unit test for npm manager detection
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

    // Rust-specific: unit test for pip_requirements manager detection
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

    // Ported: "default config file pattern" — lib/modules/manager/pip_requirements/index.spec.ts line 5
    #[test]
    fn pip_requirements_file_patterns_match_spec() {
        // Verifies our managerFilePatterns regex matches the same files as Renovate.
        let all_files = files(&[
            "requirements.txt",
            "requirements-dev.txt",
            "requirements_test.txt",
            "requirements_test_all.txt",
            "requirements.dev.txt",
            "requirements-dev.pip",
            "requirements_test.pip",
            "requirements_test_all.pip",
            "requirements.dev.pip",
            // Negative cases
            "setup.py",
            "pyproject.toml",
            "requirements.in",
        ]);
        let result = detect(&all_files);
        let pip = result
            .iter()
            .find(|m| m.name == "pip_requirements")
            .unwrap();
        for name in &[
            "requirements.txt",
            "requirements-dev.txt",
            "requirements_test.txt",
            "requirements_test_all.txt",
            "requirements.dev.txt",
            "requirements-dev.pip",
            "requirements_test.pip",
            "requirements_test_all.pip",
            "requirements.dev.pip",
        ] {
            assert!(
                pip.matched_files.contains(&(*name).to_owned()),
                "{name} should match pip_requirements"
            );
        }
        for name in &["setup.py", "pyproject.toml"] {
            assert!(
                !pip.matched_files.contains(&(*name).to_owned()),
                "{name} should NOT match pip_requirements"
            );
        }
    }

    // Ported: "matchRegexOrGlobList("$path") === $expected" — lib/modules/manager/circleci/index.spec.ts line 6
    #[test]
    fn circleci_file_patterns_match_spec() {
        let should_match = &[
            ".circleci/config.yml",
            ".circleci/config.yaml",
            ".circleci/foo.yaml",
            ".circleci/foo.yml",
            ".circleci/foo/config.yaml",
            ".circleci/foo/bar.yml",
            "foo/.circleci/bar.yaml",
        ];
        let should_not_match = &[
            "foo.yml",
            "circleci/foo.yml",
            ".circleci_foo/bar.yml",
            ".circleci/foo.toml",
        ];
        let all_files: Vec<String> = should_match
            .iter()
            .chain(should_not_match.iter())
            .map(|s| (*s).to_owned())
            .collect();
        let file_refs: Vec<&str> = all_files.iter().map(|s| s.as_str()).collect();
        let f = files(&file_refs);
        let result = detect(&f);
        let mgr = result.iter().find(|m| m.name == "circleci").unwrap();
        for name in should_match {
            assert!(
                mgr.matched_files.contains(&(*name).to_owned()),
                "{name} should match circleci"
            );
        }
        for name in should_not_match {
            assert!(
                !mgr.matched_files.contains(&(*name).to_owned()),
                "{name} should NOT match circleci"
            );
        }
    }

    // Ported: "matchRegexOrGlobList(\"$path\") === $expected" — lib/modules/manager/mise/index.spec.ts line 6
    #[test]
    fn mise_file_patterns_match_spec() {
        let should_match = &[
            "mise.toml",
            ".mise.toml",
            "mise.local.toml",
            ".mise.local.toml",
            "mise.production.toml",
            ".mise.dev.toml",
            "mise/config.toml",
            ".mise/config.toml",
            "mise/config.local.toml",
            ".mise/config.production.toml",
            ".config/mise.toml",
            ".config/mise.local.toml",
            ".config/mise.staging.toml",
            ".config/mise/config.toml",
            ".config/mise/config.local.toml",
            ".config/mise/config.production.toml",
            ".config/mise/mise.toml",
            ".config/mise/mise.local.toml",
            ".config/mise/mise.dev.toml",
            ".rtx.toml",
            ".rtx.local.toml",
            ".rtx.production.toml",
            "subdir/mise.toml",
            "subdir/.mise.toml",
            "subdir/.config/mise.toml",
            "subdir/.config/mise/config.toml",
            "deep/nested/path/mise.toml",
            "deep/nested/.config/mise/mise.toml",
        ];
        let should_not_match = &[
            "foo.toml",
            "mise.json",
            "mise.yaml",
            "mise-config.toml",
            "rtx.toml",
            ".config/other.toml",
            "mise.toml.backup",
            ".mise.toml.bak",
        ];
        let all_files: Vec<String> = should_match
            .iter()
            .chain(should_not_match.iter())
            .map(|s| (*s).to_owned())
            .collect();
        let file_refs: Vec<&str> = all_files.iter().map(|s| s.as_str()).collect();
        let f = files(&file_refs);
        let result = detect(&f);
        let mise = result
            .iter()
            .find(|m| m.name == "mise")
            .expect("mise manager not detected");
        for name in should_match {
            assert!(
                mise.matched_files.contains(&(*name).to_owned()),
                "{name} should match mise"
            );
        }
        for name in should_not_match {
            assert!(
                !mise.matched_files.contains(&(*name).to_owned()),
                "{name} should NOT match mise"
            );
        }
    }

    // Ported: "managerFilePatterns regex is correct" — lib/modules/manager/kotlin-script/index.spec.ts line 6
    #[test]
    fn kotlin_script_manager_file_patterns_regex_is_correct() {
        let def = MANAGER_DEFS
            .iter()
            .find(|manager| manager.name == "kotlin-script")
            .expect("kotlin-script manager must be registered");
        assert_eq!(def.patterns.len(), 1);
        let regex = Regex::new(def.patterns[0]).expect("kotlin-script pattern must compile");
        assert!(regex.is_match("build.main.kts"));
        assert!(regex.is_match("scripts/deps.main.kts"));
        assert!(!regex.is_match("build.gradle.kts"));
    }

    // Ported: "matchRegexOrGlobList(\"$path\") === $expected" — lib/modules/manager/proto/index.spec.ts line 6
    #[test]
    fn proto_file_patterns_match_spec() {
        let should_match = &[
            ".prototools",
            "subdir/.prototools",
            "deep/nested/path/.prototools",
        ];
        let should_not_match = &[
            "prototools",
            ".prototools.bak",
            ".prototools.toml",
            "prototools.toml",
        ];
        let all_files: Vec<String> = should_match
            .iter()
            .chain(should_not_match.iter())
            .map(|s| (*s).to_owned())
            .collect();
        let file_refs: Vec<&str> = all_files.iter().map(|s| s.as_str()).collect();
        let f = files(&file_refs);
        let result = detect(&f);
        let proto = result
            .iter()
            .find(|m| m.name == "proto")
            .expect("proto manager not detected");
        for name in should_match {
            assert!(
                proto.matched_files.contains(&(*name).to_owned()),
                "{name} should match proto"
            );
        }
        for name in should_not_match {
            assert!(
                !proto.matched_files.contains(&(*name).to_owned()),
                "{name} should NOT match proto"
            );
        }
    }

    // Rust-specific: unit test for github-actions manager detection
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

    // Rust-specific: unit test for dockerfile manager detection
    #[test]
    fn detects_dockerfile() {
        let f = files(&["Dockerfile", "docker/Dockerfile.prod", "src/main.rs"]);
        let result = detect(&f);
        let df = result.iter().find(|m| m.name == "dockerfile").unwrap();
        assert!(df.matched_files.contains(&"Dockerfile".to_owned()));
    }

    // Rust-specific: unit test for docker-compose manager detection
    #[test]
    fn detects_docker_compose() {
        let f = files(&["docker-compose.yml", "compose.yaml"]);
        let result = detect(&f);
        let dc = result.iter().find(|m| m.name == "docker-compose").unwrap();
        assert_eq!(dc.matched_files.len(), 2);
    }

    // Rust-specific: unit test for maven manager detection
    #[test]
    fn detects_maven_pom() {
        let f = files(&["pom.xml", "module/pom.xml", "parent.pom.xml"]);
        let result = detect(&f);
        let maven = result.iter().find(|m| m.name == "maven").unwrap();
        assert!(maven.matched_files.contains(&"pom.xml".to_owned()));
        assert!(maven.matched_files.contains(&"module/pom.xml".to_owned()));
        assert!(maven.matched_files.contains(&"parent.pom.xml".to_owned()));
    }

    // Rust-specific: unit test for empty file list handling
    #[test]
    fn empty_file_list_returns_no_managers() {
        assert!(detect(&[]).is_empty());
    }

    // Rust-specific: unit test for unrelated files handling
    #[test]
    fn unrelated_files_return_no_managers() {
        let f = files(&["README.md", "LICENSE", "src/lib.rs"]);
        // .rs files don't match any manager pattern
        let result = detect(&f);
        assert!(!result.iter().any(|m| m.name == "cargo"));
    }

    // Rust-specific: unit test for multi-manager detection
    #[test]
    fn detects_multiple_managers_in_same_repo() {
        let f = files(&["Cargo.toml", "package.json", ".github/workflows/ci.yml"]);
        let result = detect(&f);
        assert!(result.iter().any(|m| m.name == "cargo"));
        assert!(result.iter().any(|m| m.name == "npm"));
        assert!(result.iter().any(|m| m.name == "github-actions"));
    }

    // Rust-specific: unit test for manager_default_datasource helper
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

    // Rust-specific: unit test for manager_default_datasource helper
    #[test]
    fn manager_default_datasource_unknown_returns_none() {
        assert_eq!(manager_default_datasource("unknown-manager"), None);
        assert_eq!(manager_default_datasource("hermit"), None);
    }

    // Ported: "getCustomManagerList" — lib/modules/manager/custom/index.spec.ts line 4
    #[test]
    fn custom_manager_list_contains_strings() {
        assert!(!CUSTOM_MANAGER_LIST.is_empty());
        assert!(CUSTOM_MANAGER_LIST.iter().all(|s| !s.is_empty()));
    }

    // Ported: "works" — lib/modules/manager/custom/index.spec.ts line 9
    #[test]
    fn is_custom_manager_returns_correct_values() {
        assert!(!is_custom_manager("npm"));
        assert!(is_custom_manager("regex"));
        assert!(!is_custom_manager("custom.regex"));
        assert!(is_custom_manager("jsonata"));
        assert!(!is_custom_manager("custom.jsonata"));
    }

    fn pkg(file: &str) -> PackageFileEntry {
        PackageFileEntry {
            package_file: file.to_owned(),
            lock_files: vec![],
        }
    }
    fn pkg_locked(file: &str, lock: &str) -> PackageFileEntry {
        PackageFileEntry {
            package_file: file.to_owned(),
            lock_files: vec![lock.to_owned()],
        }
    }
    fn extract(manager: &str, files: &[PackageFileEntry]) -> ExtractResult {
        ExtractResult {
            manager: manager.to_owned(),
            package_files: Some(files.to_vec()),
        }
    }
    fn extract_none(manager: &str) -> ExtractResult {
        ExtractResult {
            manager: manager.to_owned(),
            package_files: None,
        }
    }

    // Ported: "handles empty input" — lib/workers/repository/extract/supersedes.spec.ts line 6
    #[test]
    fn supersedes_handles_empty_input() {
        let mut results: Vec<ExtractResult> = vec![];
        process_supersedes_managers(&mut results);
        assert!(results.is_empty());
    }

    // Ported: "ignores extracts without superseding managers" — lib/workers/repository/extract/supersedes.spec.ts line 12
    #[test]
    fn supersedes_ignores_non_superseding_managers() {
        let mut results = vec![extract("ansible", &[pkg("test.yml")])];
        process_supersedes_managers(&mut results);
        assert_eq!(results[0].package_files.as_ref().unwrap().len(), 1);
    }

    // Ported: "removes superseded package files without lock files" — lib/workers/repository/extract/supersedes.spec.ts line 28
    #[test]
    fn supersedes_removes_superseded_files_without_lock() {
        let mut results = vec![
            extract("bun", &[pkg("package.json")]),
            extract("npm", &[pkg("package.json")]),
        ];
        process_supersedes_managers(&mut results);
        assert_eq!(results[0].package_files.as_ref().unwrap().len(), 1);
        assert!(results[1].package_files.as_ref().unwrap().is_empty());
    }

    // Ported: "keeps superseded package files with lock files" — lib/workers/repository/extract/supersedes.spec.ts line 52
    #[test]
    fn supersedes_keeps_files_with_lock_files() {
        let mut results = vec![
            extract("bun", &[pkg("package.json")]),
            extract("npm", &[pkg_locked("package.json", "package-lock.json")]),
        ];
        process_supersedes_managers(&mut results);
        // bun loses package.json (npm has lock, so npm is not superseded, bun is)
        assert!(results[0].package_files.as_ref().unwrap().is_empty());
        assert_eq!(results[1].package_files.as_ref().unwrap().len(), 1);
    }

    // Ported: "keeps non-superseded package files" — lib/workers/repository/extract/supersedes.spec.ts line 88
    #[test]
    fn supersedes_keeps_non_superseded_files() {
        let mut results = vec![
            extract("bun", &[pkg("package.json")]),
            extract("npm", &[pkg("package.json"), pkg("other/package.json")]),
        ];
        process_supersedes_managers(&mut results);
        assert_eq!(results[0].package_files.as_ref().unwrap().len(), 1);
        assert_eq!(results[1].package_files.as_ref().unwrap().len(), 1);
        assert_eq!(
            results[1].package_files.as_ref().unwrap()[0].package_file,
            "other/package.json"
        );
    }

    // Ported: "handles primary extract with undefined packageFiles" — lib/workers/repository/extract/supersedes.spec.ts line 115
    #[test]
    fn supersedes_handles_primary_with_no_package_files() {
        let mut results = vec![extract_none("bun"), extract("npm", &[pkg("package.json")])];
        process_supersedes_managers(&mut results);
        assert!(results[0].package_files.is_none());
        assert_eq!(results[1].package_files.as_ref().unwrap().len(), 1);
    }

    // Ported: "handles missing secondary extract manager" — lib/workers/repository/extract/supersedes.spec.ts line 137
    #[test]
    fn supersedes_handles_missing_secondary_manager() {
        let mut results = vec![extract("bun", &[pkg("package.json")])];
        process_supersedes_managers(&mut results);
        assert_eq!(results[0].package_files.as_ref().unwrap().len(), 1);
    }

    // Ported: "handles secondary extract with undefined packageFiles" — lib/workers/repository/extract/supersedes.spec.ts line 153
    #[test]
    fn supersedes_handles_secondary_with_no_package_files() {
        let mut results = vec![extract("bun", &[pkg("package.json")]), extract_none("npm")];
        process_supersedes_managers(&mut results);
        assert_eq!(results[0].package_files.as_ref().unwrap().len(), 1);
        assert!(results[1].package_files.is_none());
    }

    // Ported: "does not crash for lazy regex" — lib/modules/manager/custom/regex/utils.spec.ts line 16
    #[test]
    fn regex_match_all_does_not_crash_for_lazy_regex() {
        let re = regex::Regex::new(r"(?P<currentDigest>.*?)").unwrap();
        let results = regex_match_all(&re, "1f699d2bfc99bbbe4c1ed5bb8fc21e6911d69c6e\n");
        // Should not panic and return a Vec (capped at 10_000)
        assert!(results.len() <= 10_000);
    }

    // ── file-match tests ──────────────────────────────────────────────────────

    fn file_list() -> Vec<String> {
        vec![
            "package.json".to_owned(),
            "frontend/package.json".to_owned(),
        ]
    }

    // Ported: "returns fileList if no includePaths" — lib/workers/repository/extract/file-match.spec.ts line 8
    #[test]
    fn get_included_files_returns_all_when_no_include_paths() {
        let fl = file_list();
        let res = get_included_files(&fl, &[]);
        assert_eq!(res, vec!["package.json", "frontend/package.json"]);
    }

    // Ported: "returns exact matches" — lib/workers/repository/extract/file-match.spec.ts line 13
    #[test]
    fn get_included_files_exact_match() {
        let fl = file_list();
        let res = get_included_files(&fl, &["frontend/package.json"]);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "frontend/package.json");
    }

    // Ported: "returns minimatch matches" — lib/workers/repository/extract/file-match.spec.ts line 20
    #[test]
    fn get_included_files_glob_match() {
        let fl = file_list();
        let res = get_included_files(&fl, &["frontend/**"]);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "frontend/package.json");
    }

    // Ported: "returns fileList if no ignoredPaths" — lib/workers/repository/extract/file-match.spec.ts line 29
    #[test]
    fn filter_ignored_files_returns_all_when_no_ignore_paths() {
        let fl = file_list();
        let res = filter_ignored_files(&fl, &[]);
        assert_eq!(res, vec!["package.json", "frontend/package.json"]);
    }

    // Ported: "ignores partial matches" — lib/workers/repository/extract/file-match.spec.ts line 34
    #[test]
    fn filter_ignored_files_ignores_substring_matches() {
        let fl = file_list();
        let res = filter_ignored_files(&fl, &["frontend"]);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "package.json");
    }

    // Ported: "returns minimatch matches" — lib/workers/repository/extract/file-match.spec.ts line 41
    #[test]
    fn filter_ignored_files_glob_match() {
        let fl = file_list();
        let res = filter_ignored_files(&fl, &["frontend/**"]);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "package.json");
    }

    // Ported: "returns npm files" — lib/workers/repository/extract/file-match.spec.ts line 57
    #[test]
    fn get_matching_files_npm_pattern() {
        let mut fl = file_list();
        fl.push("Dockerfile".to_owned());
        let res = get_matching_files(&fl, &[], &[], &["/(^|/)package\\.json$/"]);
        assert_eq!(res.len(), 2);
        assert!(res.contains(&"package.json".to_owned()));
        assert!(res.contains(&"frontend/package.json".to_owned()));
    }

    // Ported: "deduplicates" — lib/workers/repository/extract/file-match.spec.ts line 64
    #[test]
    fn get_matching_files_deduplicates() {
        let mut fl = file_list();
        fl.push("Dockerfile".to_owned());
        // Two patterns both matching package.json should not duplicate
        let res = get_matching_files(&fl, &[], &[], &["/(^|/)package\\.json$/", "package.json"]);
        assert_eq!(res.len(), 2);
    }

    // ── manager registry tests ────────────────────────────────────────────

    // Ported: "gets" — lib/modules/manager/index.spec.ts line 45
    #[test]
    fn manager_registry_get_manager_list() {
        let list = get_manager_list();
        assert!(!list.is_empty());
        assert!(list.contains(&"npm"));
        assert!(list.contains(&"cargo"));
        assert!(list.contains(&"maven"));
    }

    // Ported: "works" — lib/modules/manager/index.spec.ts line 51
    #[test]
    fn manager_registry_get_enabled_managers_all() {
        // No config → all managers
        let all = get_enabled_managers_list(None);
        assert_eq!(all, all_managers_list());
    }

    // Ported: "works" — lib/modules/manager/index.spec.ts line 51
    #[test]
    fn manager_registry_get_enabled_managers_filtered() {
        let config = vec!["custom.regex".to_owned(), "npm".to_owned()];
        let result = get_enabled_managers_list(Some(&config));
        // custom.regex → regex, npm stays npm; sorted: npm < regex
        assert_eq!(result, vec!["npm", "regex"]);
    }

    // Ported: "when no manager found, returns undefined" — lib/modules/manager/index.spec.ts line 265
    // Ported: "when manager found, but no prettyDepType found, returns undefined" — lib/modules/manager/index.spec.ts line 271
    // Ported: "when manager found, and a prettyDepType found in knownDepTypes, returns the defined prettyDepType" — lib/modules/manager/index.spec.ts line 279
    #[test]
    fn manager_get_pretty_dep_type() {
        // Unknown manager → None
        assert_eq!(get_pretty_dep_type("invalid-manager", "unused"), None);
        // Known manager, unknown depType → None
        assert_eq!(get_pretty_dep_type("npm", "foo-bar-baz"), None);
        assert_eq!(get_pretty_dep_type("regex", "foo-bar-baz"), None);
        // Known manager, known depType → prettyDepType
        assert_eq!(
            get_pretty_dep_type("npm", "dependencies"),
            Some("dependency")
        );
    }

    // Ported: "gets something" — lib/modules/manager/index.spec.ts line 38
    // Ported: "returns true" — lib/modules/manager/index.spec.ts line 252
    // Ported: "returns false" — lib/modules/manager/index.spec.ts line 258
    #[test]
    fn manager_registry_manager_exists() {
        assert!(manager_exists("dockerfile"));
        assert!(manager_exists("regex"));
        assert!(manager_exists("custom.regex"));
        assert!(!manager_exists("unknown-manager"));
        // isKnownManager 'returns true' cases:
        assert!(manager_exists("npm"));
        // isKnownManager 'returns false' cases:
        assert!(!manager_exists("npm-unkown"));
        assert!(!manager_exists("custom.unknown"));
    }

    // Ported: "iterates through managers" — lib/modules/manager/index.spec.ts line 108
    #[test]
    fn manager_registry_detect_all_global_config_empty() {
        let result = detect_all_global_config();
        assert!(result.is_empty());
    }

    #[test]
    fn is_disabled_by_default_known() {
        assert!(is_disabled_by_default("azure-pipelines"));
        assert!(is_disabled_by_default("git-submodules"));
    }

    #[test]
    fn is_disabled_by_default_unknown() {
        assert!(!is_disabled_by_default("npm"));
        assert!(!is_disabled_by_default("cargo"));
    }

    #[test]
    fn manager_categories_known() {
        assert_eq!(manager_categories("npm"), &["js"]);
        assert_eq!(manager_categories("cargo"), &["rust"]);
        assert_eq!(manager_categories("maven"), &["java"]);
    }

    #[test]
    fn manager_categories_unknown() {
        assert!(manager_categories("unknown").is_empty());
    }

    #[test]
    fn manager_default_datasource_known() {
        assert_eq!(manager_default_datasource("npm"), Some("npm"));
        assert_eq!(manager_default_datasource("cargo"), Some("crate"));
        assert_eq!(manager_default_datasource("maven"), Some("maven"));
    }

    #[test]
    fn manager_default_datasource_unknown() {
        assert_eq!(manager_default_datasource("unknown"), None);
    }

    #[test]
    fn manager_default_registry_urls_known() {
        assert_eq!(
            manager_default_registry_urls("npm"),
            &["https://registry.npmjs.org"]
        );
        assert_eq!(
            manager_default_registry_urls("cargo"),
            &["https://crates.io/"]
        );
    }

    #[test]
    fn manager_default_registry_urls_unknown() {
        assert!(manager_default_registry_urls("unknown").is_empty());
    }

    #[test]
    fn all_manager_ids_non_empty() {
        let ids = all_manager_ids();
        assert!(!ids.is_empty());
        assert!(ids.contains(&"npm"));
        assert!(ids.contains(&"cargo"));
    }

    #[test]
    fn all_managers_list_non_empty() {
        let list = all_managers_list();
        assert!(!list.is_empty());
    }

    #[test]
    fn get_enabled_managers_list_none() {
        let list = get_enabled_managers_list(None);
        assert!(!list.is_empty());
    }

    #[test]
    fn get_enabled_managers_list_some() {
        let list = get_enabled_managers_list(Some(&["npm".into(), "cargo".into()]));
        assert!(list.contains(&"npm"));
        assert!(list.contains(&"cargo"));
    }

    #[test]
    fn manager_exists_known() {
        assert!(manager_exists("npm"));
        assert!(manager_exists("cargo"));
    }

    #[test]
    fn manager_exists_unknown() {
        assert!(!manager_exists("unknown-manager"));
    }

    #[test]
    fn is_custom_manager_true() {
        assert!(is_custom_manager("regex"));
        assert!(is_custom_manager("jsonata"));
    }

    #[test]
    fn is_custom_manager_false() {
        assert!(!is_custom_manager("npm"));
        assert!(!is_custom_manager("custom.regex"));
    }

    #[test]
    fn get_pretty_dep_type_known() {
        assert_eq!(
            get_pretty_dep_type("npm", "dependencies"),
            Some("dependency")
        );
        assert_eq!(
            get_pretty_dep_type("npm", "devDependencies"),
            Some("devDependency")
        );
    }

    #[test]
    fn get_pretty_dep_type_unknown() {
        assert_eq!(get_pretty_dep_type("npm", "foo-bar-baz"), None);
        assert_eq!(get_pretty_dep_type("unknown", "dependencies"), None);
    }

    #[test]
    fn supersedes_managers_known() {
        assert_eq!(supersedes_managers("bun"), &["npm"]);
        assert_eq!(supersedes_managers("deno"), &["npm"]);
        assert_eq!(supersedes_managers("poetry"), &["pep621"]);
    }

    #[test]
    fn supersedes_managers_unknown() {
        assert!(supersedes_managers("unknown").is_empty());
    }

    #[test]
    fn get_included_files_basic() {
        let files = vec!["a.txt".into(), "b.txt".into(), "c.txt".into()];
        let included = get_included_files(&files, &["a.txt", "b.txt"]);
        assert_eq!(included, vec!["a.txt", "b.txt"]);
    }

    #[test]
    fn filter_ignored_files_basic() {
        let files = vec!["a.txt".into(), "b.txt".into(), "c.txt".into()];
        let filtered = filter_ignored_files(&files, &["b.txt"]);
        assert_eq!(filtered, vec!["a.txt", "c.txt"]);
    }
}

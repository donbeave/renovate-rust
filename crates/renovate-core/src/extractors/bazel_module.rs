//! Bazel `MODULE.bazel` (Bzlmod) dependency extractor.
//!
//! Parses `bazel_dep()` and `single_version_override()` / `archive_override()`
//! calls from Bazel module files to extract Bazel Central Registry deps.
//!
//! Renovate reference:
//! - `lib/modules/manager/bazel-module/extract.ts`
//! - Pattern: `/(^|/|\.)MODULE\.bazel$/`
//! - Datasource: Bazel Central Registry
//!
//! ## File format
//!
//! ```starlark
//! module(name = "my_module", version = "1.0.0")
//!
//! bazel_dep(name = "rules_go", version = "0.41.0")
//! bazel_dep(name = "gazelle", version = "0.32.0", dev_dependency = True)
//!
//! single_version_override(
//!     module_name = "rules_go",
//!     version = "0.42.0",
//! )
//! ```

use std::collections::BTreeMap;
use std::sync::LazyLock;

use regex::Regex;

/// A single extracted Bazel module dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelModuleDep {
    /// Module name (e.g. `rules_go`).
    pub name: String,
    /// Version string (e.g. `0.41.0`).
    pub current_value: String,
    /// Which MODULE.bazel declaration produced this dep.
    pub dep_type: BazelModuleDepType,
    /// Optional Bazel registry URLs declared by overrides.
    pub registry_urls: Vec<String>,
    /// Whether this is a dev dependency.
    pub dev_dependency: bool,
    /// Set when the dep should be skipped.
    pub skip_reason: Option<BazelSkipReason>,
}

/// A dependency extracted from `crate.spec(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelCrateSpecDep {
    pub name: String,
    pub current_value: String,
    pub datasource: &'static str,
    pub package_name: Option<String>,
    pub registry_urls: Vec<String>,
    pub nested_version: bool,
    pub skip_reason: Option<BazelSkipReason>,
}

/// A dependency extracted from `maven.install(...)` or `maven.artifact(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelMavenDep {
    pub dep_name: String,
    pub current_value: String,
    pub dep_type: &'static str,
    pub registry_urls: Vec<String>,
}

/// A dependency extracted from `oci.pull(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelOciPullDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub datasource: &'static str,
    pub dep_type: &'static str,
}

/// A dependency extracted from `git_repository(...)` or `new_git_repository(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelGitRepositoryDep {
    pub dep_name: String,
    pub package_name: Option<String>,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub datasource: &'static str,
    pub dep_type: &'static str,
}

/// A dependency extracted from `rules_img` pull repository rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelRulesImgPullDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub registry_urls: Vec<String>,
    pub datasource: &'static str,
    pub dep_type: &'static str,
}

/// A parsed `.bazelrc` option.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelrcOption {
    pub name: String,
    pub value: Option<String>,
}

/// A parsed `.bazelrc` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BazelrcEntry {
    Import {
        path: String,
        is_try: bool,
    },
    Command {
        command: String,
        options: Vec<BazelrcOption>,
        config: Option<String>,
    },
}

impl BazelrcEntry {
    pub fn get_option(&self, name: &str) -> Option<&BazelrcOption> {
        match self {
            Self::Command { options, .. } => options.iter().find(|option| option.name == name),
            Self::Import { .. } => None,
        }
    }
}

/// Parser fragment produced while reading `MODULE.bazel` Starlark calls.
///
/// Renovate reference: `lib/modules/manager/bazel-module/parser/fragments.ts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BazelFragment {
    String {
        value: String,
        is_complete: bool,
    },
    Boolean {
        value: bool,
        is_complete: bool,
    },
    Array {
        items: Vec<BazelFragment>,
        is_complete: bool,
    },
    Rule {
        rule: String,
        children: BTreeMap<String, BazelFragment>,
        is_complete: bool,
    },
    PreparedExtensionTag {
        extension: String,
        raw_extension: String,
        offset: usize,
        is_complete: bool,
    },
    ExtensionTag {
        extension: String,
        raw_extension: String,
        tag: String,
        offset: usize,
        children: BTreeMap<String, BazelFragment>,
        raw_string: Option<String>,
        is_complete: bool,
    },
    Attribute {
        name: String,
        value: Option<Box<BazelFragment>>,
        is_complete: bool,
    },
    UseRepoRule {
        variable_name: String,
        bzl_file: String,
        rule_name: String,
        is_complete: bool,
    },
    RepoRuleCall {
        function_name: String,
        children: BTreeMap<String, BazelFragment>,
        is_complete: bool,
    },
}

impl BazelFragment {
    fn type_name(&self) -> &'static str {
        match self {
            BazelFragment::String { .. } => "string",
            BazelFragment::Boolean { .. } => "boolean",
            BazelFragment::Array { .. } => "array",
            BazelFragment::Rule { .. } => "rule",
            BazelFragment::PreparedExtensionTag { .. } => "preparedExtensionTag",
            BazelFragment::ExtensionTag { .. } => "extensionTag",
            BazelFragment::Attribute { .. } => "attribute",
            BazelFragment::UseRepoRule { .. } => "useRepoRule",
            BazelFragment::RepoRuleCall { .. } => "repoRuleCall",
        }
    }

    fn is_complete(&self) -> bool {
        match self {
            BazelFragment::String { is_complete, .. }
            | BazelFragment::Boolean { is_complete, .. }
            | BazelFragment::Array { is_complete, .. }
            | BazelFragment::Rule { is_complete, .. }
            | BazelFragment::PreparedExtensionTag { is_complete, .. }
            | BazelFragment::ExtensionTag { is_complete, .. }
            | BazelFragment::Attribute { is_complete, .. }
            | BazelFragment::UseRepoRule { is_complete, .. }
            | BazelFragment::RepoRuleCall { is_complete, .. } => *is_complete,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelCtxProcessingError {
    pub message: String,
    pub current: BazelFragment,
    pub parent: Option<BazelFragment>,
}

impl BazelCtxProcessingError {
    pub fn new(current: BazelFragment, parent: Option<BazelFragment>) -> Self {
        let parent_type = parent
            .as_ref()
            .map(BazelFragment::type_name)
            .unwrap_or("none");
        Self {
            message: format!(
                "Invalid context state. current: {}, parent: {}",
                current.type_name(),
                parent_type
            ),
            current,
            parent,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BazelCtxError {
    Message(&'static str),
    Processing(Box<BazelCtxProcessingError>),
}

/// Stack context for Bazel module parser fragments.
///
/// This mirrors the checked state transitions from
/// `lib/modules/manager/bazel-module/parser/context.ts`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BazelCtx {
    source: String,
    stack: Vec<BazelFragment>,
    results: Vec<BazelFragment>,
}

impl BazelCtx {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            stack: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn start_rule(&mut self, name: &str) {
        self.stack.push(fragment_rule(name, BTreeMap::new(), false));
    }

    pub fn end_rule(&mut self) -> Result<(), BazelCtxError> {
        let Some(current) = self.stack.last_mut() else {
            return Err(BazelCtxError::Message("Requested current, but no value."));
        };
        match current {
            BazelFragment::Rule { is_complete, .. } => {
                *is_complete = true;
                self.process_stack()
            }
            _ => Err(BazelCtxError::Message(
                "Requested current rule, but does not exist.",
            )),
        }
    }

    pub fn start_array(&mut self) {
        self.stack.push(fragment_array(Vec::new(), false));
    }

    pub fn end_array(&mut self) -> Result<(), BazelCtxError> {
        let Some(current) = self.stack.last_mut() else {
            return Err(BazelCtxError::Message("Requested current, but no value."));
        };
        match current {
            BazelFragment::Array { is_complete, .. } => {
                *is_complete = true;
                self.process_stack()
            }
            _ => Err(BazelCtxError::Message(
                "Requested current array, but does not exist.",
            )),
        }
    }

    pub fn start_attribute(&mut self, name: &str) -> Result<(), BazelCtxError> {
        self.stack.push(fragment_attribute(name, None, false));
        self.process_stack()
    }

    pub fn add_string(&mut self, value: &str) -> Result<(), BazelCtxError> {
        self.stack.push(fragment_string(value));
        self.process_stack()
    }

    pub fn prepare_extension_tag(&mut self, extension: &str, raw_extension: &str, offset: usize) {
        self.stack.push(fragment_prepared_extension_tag(
            extension,
            raw_extension,
            offset,
        ));
    }

    pub fn start_extension_tag(&mut self, tag: &str) -> Result<(), BazelCtxError> {
        let Some(current) = self.stack.pop() else {
            return Err(BazelCtxError::Message("Requested current, but no value."));
        };
        match current {
            BazelFragment::PreparedExtensionTag {
                extension,
                raw_extension,
                offset,
                ..
            } => {
                self.stack.push(fragment_extension_tag(
                    &extension,
                    &raw_extension,
                    tag,
                    offset,
                    BTreeMap::new(),
                    None,
                    false,
                ));
                Ok(())
            }
            other => {
                self.stack.push(other);
                Err(BazelCtxError::Message(
                    "Requested current prepared extension tag, but does not exist.",
                ))
            }
        }
    }

    pub fn end_extension_tag(&mut self, offset: usize) -> Result<(), BazelCtxError> {
        let Some(current) = self.stack.last_mut() else {
            return Err(BazelCtxError::Message("Requested current, but no value."));
        };
        match current {
            BazelFragment::ExtensionTag {
                is_complete,
                raw_string,
                ..
            } => {
                *is_complete = true;
                *raw_string = Some(self.source.chars().take(offset).collect());
                self.process_stack()
            }
            _ => Err(BazelCtxError::Message(
                "Requested current extension tag, but does not exist.",
            )),
        }
    }

    pub fn end_use_repo_rule(&mut self) -> Result<(), BazelCtxError> {
        let Some(current) = self.stack.last() else {
            return Err(BazelCtxError::Message("Requested current, but no value."));
        };
        if current.type_name() == "useRepoRule" {
            self.process_stack()
        } else {
            Err(BazelCtxError::Message(
                "Requested current use repo rule, but does not exist.",
            ))
        }
    }

    pub fn end_repo_rule_call(&mut self, _offset: usize) -> Result<(), BazelCtxError> {
        let Some(current) = self.stack.last() else {
            return Err(BazelCtxError::Message("Requested current, but no value."));
        };
        if current.type_name() == "repoRuleCall" {
            self.process_stack()
        } else {
            Err(BazelCtxError::Message(
                "Requested current repo rule call, but does not exist.",
            ))
        }
    }

    fn process_stack(&mut self) -> Result<(), BazelCtxError> {
        while self.pop_stack()? {}
        Ok(())
    }

    fn pop_stack(&mut self) -> Result<bool, BazelCtxError> {
        let Some(current) = self.stack.pop() else {
            return Ok(false);
        };
        if !current.is_complete() {
            self.stack.push(current);
            return Ok(false);
        }

        let parent = self.stack.last_mut();
        if let Some(parent) = parent {
            match parent {
                BazelFragment::Attribute {
                    value, is_complete, ..
                } if fragment_is_value(&current) => {
                    *value = Some(Box::new(current));
                    *is_complete = true;
                    return Ok(true);
                }
                BazelFragment::Array { items, .. } if fragment_is_primitive(&current) => {
                    items.push(current);
                    return Ok(true);
                }
                BazelFragment::Rule { children, .. } => {
                    if let BazelFragment::Attribute {
                        name,
                        value: Some(value),
                        ..
                    } = current
                    {
                        children.insert(name, *value);
                        return Ok(true);
                    }
                }
                BazelFragment::ExtensionTag { children, .. } => {
                    if let BazelFragment::Attribute {
                        name,
                        value: Some(value),
                        ..
                    } = current
                    {
                        children.insert(name, *value);
                        return Ok(true);
                    }
                }
                _ => {}
            }
        } else if matches!(
            current,
            BazelFragment::Rule { .. } | BazelFragment::ExtensionTag { .. }
        ) {
            self.results.push(current);
            return Ok(true);
        }

        Err(BazelCtxError::Processing(Box::new(
            BazelCtxProcessingError::new(current, self.stack.last().cloned()),
        )))
    }
}

pub fn fragment_string(value: &str) -> BazelFragment {
    BazelFragment::String {
        value: value.to_owned(),
        is_complete: true,
    }
}

pub fn fragment_boolean(value: bool) -> BazelFragment {
    BazelFragment::Boolean {
        value,
        is_complete: true,
    }
}

pub fn fragment_rule(
    rule: &str,
    children: BTreeMap<String, BazelFragment>,
    is_complete: bool,
) -> BazelFragment {
    BazelFragment::Rule {
        rule: rule.to_owned(),
        children,
        is_complete,
    }
}

pub fn fragment_prepared_extension_tag(
    extension: &str,
    raw_extension: &str,
    offset: usize,
) -> BazelFragment {
    BazelFragment::PreparedExtensionTag {
        extension: extension.to_owned(),
        raw_extension: raw_extension.to_owned(),
        offset,
        is_complete: false,
    }
}

pub fn fragment_extension_tag(
    extension: &str,
    raw_extension: &str,
    tag: &str,
    offset: usize,
    children: BTreeMap<String, BazelFragment>,
    raw_string: Option<String>,
    is_complete: bool,
) -> BazelFragment {
    BazelFragment::ExtensionTag {
        extension: extension.to_owned(),
        raw_extension: raw_extension.to_owned(),
        tag: tag.to_owned(),
        offset,
        children,
        raw_string,
        is_complete,
    }
}

pub fn fragment_attribute(
    name: &str,
    value: Option<BazelFragment>,
    is_complete: bool,
) -> BazelFragment {
    BazelFragment::Attribute {
        name: name.to_owned(),
        value: value.map(Box::new),
        is_complete,
    }
}

pub fn fragment_array(items: Vec<BazelFragment>, is_complete: bool) -> BazelFragment {
    BazelFragment::Array { items, is_complete }
}

pub fn fragment_use_repo_rule(
    variable_name: &str,
    bzl_file: &str,
    rule_name: &str,
    is_complete: bool,
) -> BazelFragment {
    BazelFragment::UseRepoRule {
        variable_name: variable_name.to_owned(),
        bzl_file: bzl_file.to_owned(),
        rule_name: rule_name.to_owned(),
        is_complete,
    }
}

pub fn fragment_repo_rule_call(
    function_name: &str,
    children: BTreeMap<String, BazelFragment>,
    is_complete: bool,
) -> BazelFragment {
    BazelFragment::RepoRuleCall {
        function_name: function_name.to_owned(),
        children,
        is_complete,
    }
}

pub fn fragment_is_primitive(fragment: &BazelFragment) -> bool {
    matches!(
        fragment,
        BazelFragment::String { .. } | BazelFragment::Boolean { .. }
    )
}

pub fn fragment_is_value(fragment: &BazelFragment) -> bool {
    matches!(
        fragment,
        BazelFragment::String { .. } | BazelFragment::Boolean { .. } | BazelFragment::Array { .. }
    )
}

/// Parse a MODULE.bazel file into a vector of [`BazelFragment`]s.
///
/// This is a lightweight recognizer (not a full Starlark parser) that handles
/// the constructs Renovate cares about: rule calls, extension tags,
/// `use_repo_rule` assignments, and repo rule calls.
///
/// Renovate reference: `lib/modules/manager/bazel-module/parser/index.ts`
pub fn parse_module_bazel(input: &str) -> Vec<BazelFragment> {
    let mut results = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            i += 1;
            continue;
        }

        // Try use_repo_rule assignment: var = use_repo_rule("...", "...")
        if let Some(fragment) = try_parse_use_repo_rule(line) {
            results.push(fragment);
            i += 1;
            continue;
        }

        // Try repo rule call: function_name(key = value, ...)
        // Only match if it looks like a simple rule call (not an extension tag or known rule)
        if let Some(rule_name) = extract_rule_name(line) {
            if !rule_name.contains('.')
                && rule_name != "bazel_dep"
                && rule_name != "git_override"
                && rule_name != "archive_override"
                && rule_name != "local_path_override"
                && rule_name != "single_version_override"
                && rule_name != "git_repository"
            {
                if let Some((_fragment, consumed)) = try_parse_rule_block(&lines[i..]) {
                    // Could be a repo rule call
                    if let Some(children) =
                        parse_rule_children(&extract_rule_body(&lines[i..]).unwrap_or_default())
                    {
                        results.push(BazelFragment::RepoRuleCall {
                            function_name: rule_name.to_owned(),
                            children,
                            is_complete: true,
                        });
                        i += consumed;
                        continue;
                    }
                }
            }
        }

        // Try extension tag: ext.tag(...)
        if line.contains('.') && line.contains('(') {
            if let Some((fragment, consumed)) = try_parse_extension_tag_block(&lines[i..]) {
                results.push(fragment);
                i += consumed;
                continue;
            }
        }

        // Try regular rule call
        if line.contains('(') {
            if let Some((fragment, consumed)) = try_parse_rule_block(&lines[i..]) {
                results.push(fragment);
                i += consumed;
                continue;
            }
        }

        i += 1;
    }

    results
}

/// Try to parse a `use_repo_rule` assignment.
fn try_parse_use_repo_rule(line: &str) -> Option<BazelFragment> {
    let trimmed = line.trim();
    // Pattern: var = use_repo_rule("bzl_file", "rule_name")
    let re = regex::Regex::new(
        r#"^(\w+)\s*=\s*use_repo_rule\s*\(\s*['"]([^'"]+)['"]\s*,\s*['"]([^'"]+)['"]\s*\)$"#,
    )
    .ok()?;
    let cap = re.captures(trimmed)?;
    let variable_name = cap.get(1)?.as_str();
    let bzl_file = cap.get(2)?.as_str();
    let rule_name = cap.get(3)?.as_str();
    Some(BazelFragment::UseRepoRule {
        variable_name: variable_name.to_owned(),
        bzl_file: bzl_file.to_owned(),
        rule_name: rule_name.to_owned(),
        is_complete: true,
    })
}

/// Extract the rule name from the start of a line like `rule_name(...)`.
fn extract_rule_name(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let idx = trimmed.find('(')?;
    let name = trimmed[..idx].trim();
    if name.is_empty() || name.contains(' ') {
        return None;
    }
    Some(name)
}

/// Extract the full body of a rule call (including multi-line) by matching parentheses.
fn extract_rule_body(lines: &[&str]) -> Option<String> {
    let first = lines.first()?;
    let start_idx = first.find('(')?;
    let mut body = String::new();
    let mut depth = 0;
    let mut started = false;

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            if !started {
                if i >= start_idx {
                    started = true;
                    if ch == '(' {
                        depth += 1;
                    }
                    body.push(ch);
                }
                continue;
            }
            body.push(ch);
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
                if depth == 0 {
                    return Some(body);
                }
            }
        }
    }

    None
}

/// Parse the children of a rule from its body string (the content inside outer parentheses).
fn parse_rule_children(body: &str) -> Option<BTreeMap<String, BazelFragment>> {
    let mut children = BTreeMap::new();
    // Remove outer parentheses if present
    let inner = body.strip_prefix('(')?.strip_suffix(')')?;
    let inner = inner.trim();
    if inner.is_empty() {
        return Some(children);
    }

    // Split by top-level commas
    for arg in split_top_level_args(inner) {
        let arg = arg.trim();
        if arg.is_empty() {
            continue;
        }
        // Pattern: key = value
        let eq_idx = arg.find('=')?;
        let key = arg[..eq_idx].trim();
        let value_str = arg[eq_idx + 1..].trim();

        if let Some(value) = parse_value(value_str) {
            children.insert(key.to_owned(), value);
        }
    }

    Some(children)
}

/// Split arguments by top-level commas (not inside brackets or parentheses).
fn split_top_level_args(s: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut start = 0;
    let mut depth_paren = 0;
    let mut depth_bracket = 0;

    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth_paren += 1,
            ')' => depth_paren -= 1,
            '[' => depth_bracket += 1,
            ']' => depth_bracket -= 1,
            ',' if depth_paren == 0 && depth_bracket == 0 => {
                args.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }

    if start < s.len() {
        args.push(&s[start..]);
    }

    args
}

/// Parse a single value string into a `BazelFragment`.
fn parse_value(s: &str) -> Option<BazelFragment> {
    let trimmed = s.trim();

    // String literal
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        let value = &trimmed[1..trimmed.len() - 1];
        return Some(BazelFragment::String {
            value: value.to_owned(),
            is_complete: true,
        });
    }

    // Boolean
    if trimmed.eq_ignore_ascii_case("true") {
        return Some(BazelFragment::Boolean {
            value: true,
            is_complete: true,
        });
    }
    if trimmed.eq_ignore_ascii_case("false") {
        return Some(BazelFragment::Boolean {
            value: false,
            is_complete: true,
        });
    }

    // Array
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let inner = &trimmed[1..trimmed.len() - 1];
        let items: Vec<BazelFragment> = split_top_level_args(inner)
            .into_iter()
            .filter_map(|item| {
                let item = item.trim();
                if item.is_empty() {
                    return None;
                }
                parse_value(item)
            })
            .collect();
        return Some(BazelFragment::Array {
            items,
            is_complete: true,
        });
    }

    None
}

/// Try to parse a rule call block (potentially multi-line).
fn try_parse_rule_block(lines: &[&str]) -> Option<(BazelFragment, usize)> {
    let rule_name = extract_rule_name(lines.first()?)?;
    let body = extract_rule_body(lines)?;
    let consumed = count_consumed_lines(lines, &body)?;
    let children = parse_rule_children(&body)?;

    Some((
        BazelFragment::Rule {
            rule: rule_name.to_owned(),
            children,
            is_complete: true,
        },
        consumed,
    ))
}

/// Try to parse an extension tag block (potentially multi-line).
fn try_parse_extension_tag_block(lines: &[&str]) -> Option<(BazelFragment, usize)> {
    let first = lines.first()?.trim();
    let dot_idx = first.find('.')?;
    let paren_idx = first.find('(')?;
    if dot_idx >= paren_idx {
        return None;
    }

    let raw_extension = first[..dot_idx].trim();
    let tag = first[dot_idx + 1..paren_idx].trim();

    let body = extract_rule_body(lines)?;
    let consumed = count_consumed_lines(lines, &body)?;
    let children = parse_rule_children(&body)?;

    // Compute offset (character position in first line)
    let offset = first.find(raw_extension)?;

    // Build raw_string from consumed lines
    let raw_string = lines[..consumed].join("\n");

    Some((
        BazelFragment::ExtensionTag {
            extension: raw_extension.to_owned(),
            raw_extension: raw_extension.to_owned(),
            tag: tag.to_owned(),
            offset,
            children,
            raw_string: Some(raw_string),
            is_complete: true,
        },
        consumed,
    ))
}

/// Count how many lines were consumed to form the rule body.
fn count_consumed_lines(lines: &[&str], _body: &str) -> Option<usize> {
    let first = lines.first()?;
    let start_idx = first.find('(')?;
    let mut consumed = 0;
    let mut depth = 0;
    let mut started = false;

    for line in lines {
        consumed += 1;
        for (i, ch) in line.chars().enumerate() {
            if !started {
                if i >= start_idx {
                    started = true;
                    if ch == '(' {
                        depth += 1;
                    }
                    if depth == 0 && ch == ')' {
                        return Some(consumed);
                    }
                }
                continue;
            }
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
                if depth == 0 {
                    return Some(consumed);
                }
            }
        }
    }

    None
}

/// Which Bazel module declaration produced the dep.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BazelModuleDepType {
    /// `bazel_dep(...)`
    BazelDep,
    /// `single_version_override(...)`
    SingleVersionOverride,
    /// `archive_override(...)`
    ArchiveOverride,
    /// `local_path_override(...)`
    LocalPathOverride,
}

/// Why a Bazel dep is skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BazelSkipReason {
    /// No version attribute in the `bazel_dep()` call.
    UnspecifiedVersion,
    /// Version is pinned by an override declaration.
    IsPinned,
    /// Override declarations are metadata for pinning and are not updated.
    Ignored,
    /// Module is pinned to an archive URL.
    FileDependency,
    /// Module is pinned to a local path.
    LocalDependency,
    /// Override declaration does not use a supported datasource.
    UnsupportedDatasource,
    /// Crate is local-path based.
    PathDependency,
    /// Crate spec has neither a version nor a supported alternate source.
    InvalidDependencySpecification,
}

/// Matches a `bazel_dep(name = "...", version = "...", ...)` call.
/// Handles multi-line calls by matching `name` and `version` attributes anywhere.
static BAZEL_DEP_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)bazel_dep\s*\(([^)]+)\)").unwrap());

/// Matches a `single_version_override(...)` call.
static SINGLE_VERSION_OVERRIDE_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)single_version_override\s*\(([^)]+)\)").unwrap());

/// Matches an `archive_override(...)` call.
static ARCHIVE_OVERRIDE_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)archive_override\s*\(([^)]+)\)").unwrap());

/// Matches a `local_path_override(...)` call.
static LOCAL_PATH_OVERRIDE_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)local_path_override\s*\(([^)]+)\)").unwrap());

/// Matches a `crate.spec(...)` call.
static CRATE_SPEC_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)crate\.spec\s*\(([^)]+)\)").unwrap());

/// Matches a `maven.install(...)` call.
static MAVEN_INSTALL_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)maven\.install\s*\(([^)]+)\)").unwrap());

/// Matches a `maven.artifact(...)` call.
static MAVEN_ARTIFACT_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)maven\.artifact\s*\(([^)]+)\)").unwrap());

/// Matches an `oci.pull(...)` call.
static OCI_PULL_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)oci\.pull\s*\(([^)]+)\)").unwrap());

/// Matches a `git_repository(...)` call without matching `new_git_repository(...)`.
static GIT_REPOSITORY_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)(?:^|[^\w.])git_repository\s*\(([^)]+)\)").unwrap());

/// Matches a `new_git_repository(...)` call.
static NEW_GIT_REPOSITORY_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)new_git_repository\s*\(([^)]+)\)").unwrap());

/// Captures aliases assigned from the rules_img pull repo rule.
static RULES_IMG_PULL_ALIAS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*use_repo_rule\(\s*['"]@rules_img//img:pull\.bzl['"]\s*,\s*['"]pull['"]\s*\)"#,
    )
    .unwrap()
});

/// Extracts `name = "value"` or `name = 'value'` from a call argument list.
static ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\w+)\s*=\s*['"]([^'"]+)['"]"#).unwrap());

/// Extracts quoted strings from a Starlark list body.
static STRING_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"['"]([^'"]+)['"]"#).unwrap());

/// Extracts `dev_dependency = True` flag.
static DEV_DEP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"dev_dependency\s*=\s*True").unwrap());

struct SingleVersionOverride {
    name: String,
    version: String,
    registry_urls: Vec<String>,
}

struct UnsupportedOverride {
    name: String,
    dep_type: BazelModuleDepType,
    bazel_dep_skip_reason: BazelSkipReason,
}

/// Extract Bazel module deps from a `MODULE.bazel` file.
pub fn extract(content: &str) -> Vec<BazelModuleDep> {
    // Strip single-line comments
    let stripped = strip_comments(content);

    let overrides = parse_single_version_overrides(&stripped);
    let unsupported_overrides = parse_unsupported_overrides(&stripped);
    let mut deps = Vec::new();

    for cap in BAZEL_DEP_BLOCK_RE.captures_iter(&stripped) {
        let args = &cap[1];

        let mut name = String::new();
        let mut version = String::new();

        for kv in ATTR_RE.captures_iter(args) {
            let key = &kv[1];
            let val = kv[2].to_owned();
            match key {
                "name" => name = val,
                "version" => version = val,
                _ => {}
            }
        }

        if name.is_empty() {
            continue;
        }

        let dev_dependency = DEV_DEP_RE.is_match(args);
        let override_metadata = overrides
            .iter()
            .find(|override_dep| override_dep.name == name);
        let pinned = override_metadata.filter(|override_dep| !override_dep.version.is_empty());
        let unsupported_override = unsupported_overrides
            .iter()
            .find(|override_dep| override_dep.name == name);
        let registry_urls = pinned
            .or(override_metadata)
            .map(|override_dep| override_dep.registry_urls.clone())
            .unwrap_or_default();
        let skip_reason = unsupported_override
            .map(|override_dep| override_dep.bazel_dep_skip_reason)
            .or_else(|| pinned.map(|_| BazelSkipReason::IsPinned));

        if version.is_empty() {
            deps.push(BazelModuleDep {
                name,
                current_value: String::new(),
                dep_type: BazelModuleDepType::BazelDep,
                registry_urls,
                dev_dependency,
                skip_reason: Some(skip_reason.unwrap_or(BazelSkipReason::UnspecifiedVersion)),
            });
        } else {
            deps.push(BazelModuleDep {
                name,
                current_value: version,
                dep_type: BazelModuleDepType::BazelDep,
                registry_urls,
                dev_dependency,
                skip_reason,
            });
        }
    }

    let bazel_dep_names = deps
        .iter()
        .map(|dep| dep.name.clone())
        .collect::<std::collections::BTreeSet<_>>();

    deps.extend(
        unsupported_overrides
            .into_iter()
            .filter(|override_dep| bazel_dep_names.contains(&override_dep.name))
            .map(|override_dep| BazelModuleDep {
                name: override_dep.name,
                current_value: String::new(),
                dep_type: override_dep.dep_type,
                registry_urls: Vec::new(),
                dev_dependency: false,
                skip_reason: Some(BazelSkipReason::UnsupportedDatasource),
            }),
    );
    deps.extend(
        overrides
            .into_iter()
            .filter(|override_dep| !override_dep.version.is_empty())
            .filter(|override_dep| bazel_dep_names.contains(&override_dep.name))
            .map(|override_dep| BazelModuleDep {
                name: override_dep.name,
                current_value: override_dep.version,
                dep_type: BazelModuleDepType::SingleVersionOverride,
                registry_urls: override_dep.registry_urls,
                dev_dependency: false,
                skip_reason: Some(BazelSkipReason::Ignored),
            }),
    );
    deps
}

/// Extract `crate.spec(...)` dependencies from a `MODULE.bazel` file.
pub fn extract_crate_specs(content: &str) -> Vec<BazelCrateSpecDep> {
    let stripped = strip_comments(content);
    CRATE_SPEC_BLOCK_RE
        .captures_iter(&stripped)
        .filter_map(|cap| {
            let args = &cap[1];
            let attrs = attrs_from_args(args);
            let name = attrs.get("package")?.clone();

            if let Some(tag) = attrs.get("tag")
                && let Some(git) = attrs.get("git")
            {
                return Some(BazelCrateSpecDep {
                    name,
                    current_value: tag.clone(),
                    datasource: "github-tags",
                    package_name: github_package_name(git),
                    registry_urls: vec!["https://github.com".to_owned()],
                    nested_version: false,
                    skip_reason: None,
                });
            }

            if attrs.contains_key("path") {
                return Some(BazelCrateSpecDep {
                    name,
                    current_value: String::new(),
                    datasource: "crate",
                    package_name: None,
                    registry_urls: Vec::new(),
                    nested_version: false,
                    skip_reason: Some(BazelSkipReason::PathDependency),
                });
            }

            let Some(version) = attrs.get("version") else {
                return Some(BazelCrateSpecDep {
                    name,
                    current_value: String::new(),
                    datasource: "crate",
                    package_name: None,
                    registry_urls: Vec::new(),
                    nested_version: false,
                    skip_reason: Some(BazelSkipReason::InvalidDependencySpecification),
                });
            };

            Some(BazelCrateSpecDep {
                name,
                current_value: version.clone(),
                datasource: "crate",
                package_name: None,
                registry_urls: Vec::new(),
                nested_version: true,
                skip_reason: None,
            })
        })
        .collect()
}

/// Extract Maven dependencies from `maven.install(...)` and `maven.artifact(...)`.
pub fn extract_maven_deps(content: &str) -> Vec<BazelMavenDep> {
    let stripped = strip_comments(content);
    let mut deps = Vec::new();

    for cap in MAVEN_INSTALL_BLOCK_RE.captures_iter(&stripped) {
        let args = &cap[1];
        let registries = list_attr(args, "repositories");
        for artifact in list_attr(args, "artifacts") {
            if let Some((dep_name, current_value)) = parse_maven_coordinate(&artifact) {
                deps.push(BazelMavenDep {
                    dep_name,
                    current_value,
                    dep_type: "maven_install",
                    registry_urls: registries.clone(),
                });
            }
        }
    }

    for cap in MAVEN_ARTIFACT_BLOCK_RE.captures_iter(&stripped) {
        let args = &cap[1];
        let attrs = attrs_from_args(args);
        let Some(group) = attrs.get("group") else {
            continue;
        };
        let Some(artifact) = attrs.get("artifact") else {
            continue;
        };
        let Some(version) = attrs.get("version") else {
            continue;
        };
        deps.push(BazelMavenDep {
            dep_name: format!("{group}:{artifact}"),
            current_value: version.clone(),
            dep_type: "maven_install",
            registry_urls: Vec::new(),
        });
    }

    let install_registries = deps
        .iter()
        .find(|dep| !dep.registry_urls.is_empty())
        .map(|dep| dep.registry_urls.clone());
    if let Some(registries) = install_registries {
        for dep in &mut deps {
            if dep.registry_urls.is_empty() {
                dep.registry_urls = registries.clone();
            }
        }
    }

    deps
}

/// Extract OCI image dependencies from `oci.pull(...)`.
pub fn extract_oci_pull_deps(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<BazelOciPullDep> {
    let stripped = strip_comments(content);
    OCI_PULL_BLOCK_RE
        .captures_iter(&stripped)
        .filter_map(|cap| {
            let attrs = attrs_from_args(&cap[1]);
            let dep_name = attrs.get("name")?.clone();
            let image = attrs.get("image")?;

            Some(BazelOciPullDep {
                dep_name,
                package_name: apply_registry_alias(image, registry_aliases),
                current_value: attrs.get("tag").cloned(),
                current_digest: attrs.get("digest").cloned(),
                datasource: "docker",
                dep_type: "oci_pull",
            })
        })
        .collect()
}

/// Extract Git repository dependencies from Bazel repository rules.
pub fn extract_git_repository_deps(content: &str) -> Vec<BazelGitRepositoryDep> {
    let stripped = strip_comments(content);
    let mut deps = Vec::new();
    deps.extend(parse_git_repository_deps(
        &stripped,
        &GIT_REPOSITORY_BLOCK_RE,
        "git_repository",
    ));
    deps.extend(parse_git_repository_deps(
        &stripped,
        &NEW_GIT_REPOSITORY_BLOCK_RE,
        "new_git_repository",
    ));
    deps
}

/// Extract Docker image dependencies from `rules_img` pull repo rule aliases.
pub fn extract_rules_img_pull_deps(content: &str) -> Vec<BazelRulesImgPullDep> {
    let stripped = strip_comments(content);
    rules_img_pull_aliases(&stripped)
        .into_iter()
        .flat_map(|alias| parse_rules_img_pull_alias(&stripped, &alias))
        .collect()
}

/// Transform pre-parsed fragment objects for `rules_img` pull calls into dep objects.
///
/// Mirrors `lib/modules/manager/bazel-module/rules-img.ts` `transformRulesImgCalls()`.
/// Input fragments are JSON objects with `type`, `variableName`/`functionName`, etc.
pub fn transform_rules_img_calls(fragments: &[serde_json::Value]) -> Vec<BazelRulesImgPullDep> {
    let mut repo_rule_vars: std::collections::HashMap<String, String> = Default::default();
    for frag in fragments {
        if frag.get("type").and_then(|t| t.as_str()) == Some("useRepoRule")
            && let (Some(var), Some(bzl)) = (
                frag.get("variableName").and_then(|v| v.as_str()),
                frag.get("bzlFile").and_then(|v| v.as_str()),
            )
        {
            repo_rule_vars.insert(var.to_owned(), bzl.to_owned());
        }
    }

    let mut deps = Vec::new();
    for frag in fragments {
        if frag.get("type").and_then(|t| t.as_str()) != Some("repoRuleCall") {
            continue;
        }
        let Some(func_name) = frag.get("functionName").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(bzl_file) = repo_rule_vars.get(func_name) else {
            continue;
        };
        if !bzl_file.contains("@rules_img//img:pull.bzl") {
            continue;
        }
        let Some(children) = frag.get("children") else {
            continue;
        };
        let str_val = |key: &str| -> Option<String> {
            children.get(key)?.get("value")?.as_str().map(str::to_owned)
        };
        let Some(name) = str_val("name") else {
            continue;
        };
        let Some(repository) = str_val("repository") else {
            continue;
        };
        let registry = str_val("registry");
        let tag = str_val("tag");
        let digest = str_val("digest");
        let raw_string = frag
            .get("rawString")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();

        let package_name = registry
            .as_deref()
            .map(|r| format!("{r}/{repository}"))
            .unwrap_or(repository);
        let registry_urls = registry
            .as_deref()
            .map(|r| vec![format!("https://{r}")])
            .unwrap_or_default();

        deps.push(BazelRulesImgPullDep {
            dep_name: name,
            package_name,
            registry_urls,
            current_value: tag,
            current_digest: digest,
            dep_type: "rules_img_pull",
            datasource: "docker",
        });
        let _ = raw_string; // used for replaceString in TS but not in our struct
    }
    deps
}

/// Extract unconfigured Bazel registry URLs from a workspace `.bazelrc` file set.
pub fn extract_bazelrc_registry_urls(files: &[(&str, &str)]) -> Vec<String> {
    let mut read_files = std::collections::BTreeSet::new();
    let mut registry_urls = Vec::new();
    collect_bazelrc_registry_urls(".bazelrc", files, &mut read_files, &mut registry_urls);
    registry_urls
}

/// Parse a `.bazelrc` option string.
///
/// Renovate reference: `lib/modules/manager/bazel-module/bazelrc.ts`
/// `BazelOption.parse`.
pub fn parse_bazelrc_options(input: &str) -> Vec<BazelrcOption> {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    let mut options = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        let Some(without_prefix) = part.strip_prefix("--") else {
            continue;
        };

        if let Some((name, value)) = without_prefix.split_once('=') {
            options.push(BazelrcOption {
                name: name.to_owned(),
                value: Some(value.to_owned()),
            });
            continue;
        }

        let value = parts
            .get(index + 1)
            .filter(|next| !next.starts_with("--"))
            .map(|value| (*value).to_owned());
        options.push(BazelrcOption {
            name: without_prefix.to_owned(),
            value,
        });
    }
    options
}

/// Parse `.bazelrc` entries from file contents.
///
/// Renovate reference: `lib/modules/manager/bazel-module/bazelrc.ts`
/// `parse`.
pub fn parse_bazelrc(contents: &str) -> Vec<BazelrcEntry> {
    contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(parse_bazelrc_entry)
        .collect()
}

/// Expand `%workspace%` in a `.bazelrc` option if the resolved path is valid.
///
/// The caller supplies path validation so tests and higher-level file readers can
/// enforce their own local-path policy.
pub fn expand_bazelrc_workspace_path<F>(
    value: &str,
    workspace_dir: &str,
    mut is_valid_local_path: F,
) -> Option<String>
where
    F: FnMut(&str) -> bool,
{
    if !value.contains("%workspace%") {
        return Some(value.to_owned());
    }

    let workspace = workspace_dir.trim_end_matches('/');
    let expanded = value.replace("%workspace%", workspace);
    is_valid_local_path(&expanded).then_some(expanded)
}

/// Expand `%workspace%` paths across a list of `.bazelrc` options.
pub fn sanitize_bazelrc_options<F>(
    options: &[BazelrcOption],
    workspace_dir: &str,
    mut is_valid_local_path: F,
) -> Vec<BazelrcOption>
where
    F: FnMut(&str) -> bool,
{
    options
        .iter()
        .filter_map(|option| {
            let Some(value) = option.value.as_deref() else {
                return Some(option.clone());
            };
            Some(BazelrcOption {
                name: option.name.clone(),
                value: Some(expand_bazelrc_workspace_path(
                    value,
                    workspace_dir,
                    &mut is_valid_local_path,
                )?),
            })
        })
        .collect()
}

/// Recursively read `.bazelrc` and its imports from an in-memory file map.
///
/// Mirrors `lib/modules/manager/bazel-module/bazelrc.ts` `read()` + `readFile()`.
///
/// `files`: pairs of (path, content) — `None` content means the file does not exist.
/// `valid_paths`: paths considered local (equivalent to `isValidLocalPath` mock in tests).
///
/// Returns an `Err` if a circular import is detected.
pub fn read_bazelrc_from_files<'a>(
    workspace_dir: &str,
    files: &[(&'a str, Option<&'a str>)],
    valid_paths: &[&str],
) -> Result<Vec<BazelrcEntry>, String> {
    let bazelrc_path = if workspace_dir == "." {
        ".bazelrc".to_owned()
    } else {
        format!("{workspace_dir}/.bazelrc")
    };
    let mut read_files: Vec<String> = Vec::new();
    read_bazelrc_file_inner(
        &bazelrc_path,
        workspace_dir,
        files,
        valid_paths,
        &mut read_files,
    )
}

fn normalize_import_path(path: &str) -> String {
    path.strip_prefix("./").unwrap_or(path).to_owned()
}

fn read_bazelrc_file_inner(
    file_path: &str,
    workspace_dir: &str,
    files: &[(&str, Option<&str>)],
    valid_paths: &[&str],
    read_files: &mut Vec<String>,
) -> Result<Vec<BazelrcEntry>, String> {
    if read_files.iter().any(|f| f == file_path) {
        return Err(format!(
            "Attempted to read a bazelrc multiple times. file: {file_path}"
        ));
    }
    read_files.push(file_path.to_owned());

    let content = match files.iter().find(|(p, _)| *p == file_path) {
        Some((_, Some(content))) => *content,
        _ => return Ok(vec![]),
    };

    let entries = parse_bazelrc(content);
    let mut results = Vec::new();

    for entry in entries {
        match entry {
            BazelrcEntry::Command {
                command,
                options,
                config,
            } => {
                let sanitized = sanitize_bazelrc_options(&options, workspace_dir, |path| {
                    valid_paths.contains(&path)
                });
                results.push(BazelrcEntry::Command {
                    command,
                    options: sanitized,
                    config,
                });
            }
            BazelrcEntry::Import { path, .. } => {
                let expanded = path.replace("%workspace%", workspace_dir);
                let normalized = normalize_import_path(&expanded);
                if valid_paths.iter().any(|p| *p == normalized) {
                    let imported = read_bazelrc_file_inner(
                        &normalized,
                        workspace_dir,
                        files,
                        valid_paths,
                        read_files,
                    )?;
                    results.extend(imported);
                }
            }
        }
    }

    Ok(results)
}

fn parse_unsupported_overrides(content: &str) -> Vec<UnsupportedOverride> {
    let mut deps = Vec::new();
    deps.extend(parse_named_overrides(
        content,
        &ARCHIVE_OVERRIDE_BLOCK_RE,
        BazelModuleDepType::ArchiveOverride,
        BazelSkipReason::FileDependency,
    ));
    deps.extend(parse_named_overrides(
        content,
        &LOCAL_PATH_OVERRIDE_BLOCK_RE,
        BazelModuleDepType::LocalPathOverride,
        BazelSkipReason::LocalDependency,
    ));
    deps
}

fn parse_named_overrides(
    content: &str,
    regex: &Regex,
    dep_type: BazelModuleDepType,
    bazel_dep_skip_reason: BazelSkipReason,
) -> Vec<UnsupportedOverride> {
    regex
        .captures_iter(content)
        .filter_map(|cap| {
            let args = &cap[1];
            let name = ATTR_RE.captures_iter(args).find_map(|kv| {
                if &kv[1] == "module_name" {
                    Some(kv[2].to_owned())
                } else {
                    None
                }
            })?;
            Some(UnsupportedOverride {
                name,
                dep_type,
                bazel_dep_skip_reason,
            })
        })
        .collect()
}

fn parse_bazelrc_entry(line: &str) -> Option<BazelrcEntry> {
    if let Some((kind, path)) = line.split_once(char::is_whitespace)
        && (kind == "import" || kind == "try-import")
        && !path.trim().is_empty()
        && !path.trim().contains(char::is_whitespace)
    {
        return Some(BazelrcEntry::Import {
            path: path.trim().to_owned(),
            is_try: kind == "try-import",
        });
    }

    let (command_part, options) = line.split_once(char::is_whitespace)?;
    let (command, config) = command_part
        .split_once(':')
        .map(|(command, config)| (command, Some(config.to_owned())))
        .unwrap_or((command_part, None));
    if command.is_empty()
        || !command
            .chars()
            .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
    {
        return None;
    }

    Some(BazelrcEntry::Command {
        command: command.to_owned(),
        options: parse_bazelrc_options(options),
        config,
    })
}

fn parse_single_version_overrides(content: &str) -> Vec<SingleVersionOverride> {
    let mut deps = Vec::new();

    for cap in SINGLE_VERSION_OVERRIDE_BLOCK_RE.captures_iter(content) {
        let args = &cap[1];
        let mut name = String::new();
        let mut version = String::new();
        let mut registry_url = String::new();

        for kv in ATTR_RE.captures_iter(args) {
            let key = &kv[1];
            let val = kv[2].to_owned();
            match key {
                "module_name" => name = val,
                "version" => version = val,
                "registry" => registry_url = val,
                _ => {}
            }
        }

        if name.is_empty() {
            continue;
        }

        let registry_urls = if registry_url.is_empty() {
            Vec::new()
        } else {
            vec![registry_url]
        };

        deps.push(SingleVersionOverride {
            name,
            version,
            registry_urls,
        });
    }

    deps
}

fn attrs_from_args(args: &str) -> std::collections::BTreeMap<String, String> {
    ATTR_RE
        .captures_iter(args)
        .map(|cap| (cap[1].to_owned(), cap[2].to_owned()))
        .collect()
}

fn list_attr(args: &str, attr: &str) -> Vec<String> {
    let pattern = format!(r#"(?s){attr}\s*=\s*\[([^\]]*)\]"#);
    let Ok(regex) = Regex::new(&pattern) else {
        return Vec::new();
    };
    let Some(cap) = regex.captures(args) else {
        return Vec::new();
    };
    STRING_RE
        .captures_iter(&cap[1])
        .map(|item| item[1].to_owned())
        .collect()
}

fn parse_maven_coordinate(raw: &str) -> Option<(String, String)> {
    let parts = raw.split(':').collect::<Vec<_>>();
    if parts.len() < 3 {
        return None;
    }
    let group = parts[0];
    let artifact = parts[1];
    let version = parts.last()?;
    if group.is_empty() || artifact.is_empty() || version.is_empty() {
        None
    } else {
        Some((format!("{group}:{artifact}"), (*version).to_owned()))
    }
}

fn parse_git_repository_deps(
    content: &str,
    regex: &Regex,
    dep_type: &'static str,
) -> Vec<BazelGitRepositoryDep> {
    regex
        .captures_iter(content)
        .filter_map(|cap| {
            let attrs = attrs_from_args(&cap[1]);
            let remote = attrs.get("remote")?;
            Some(BazelGitRepositoryDep {
                dep_name: attrs.get("name")?.clone(),
                package_name: github_package_name(remote),
                current_value: attrs.get("tag").cloned(),
                current_digest: attrs.get("commit").cloned(),
                datasource: "github-tags",
                dep_type,
            })
        })
        .collect()
}

fn rules_img_pull_aliases(content: &str) -> Vec<String> {
    RULES_IMG_PULL_ALIAS_RE
        .captures_iter(content)
        .map(|cap| cap[1].to_owned())
        .collect()
}

fn parse_rules_img_pull_alias(content: &str, alias: &str) -> Vec<BazelRulesImgPullDep> {
    let pattern = format!(r"(?s)(?:^|[^\w.]){}\s*\(([^)]+)\)", regex::escape(alias));
    let Ok(regex) = Regex::new(&pattern) else {
        return Vec::new();
    };

    regex
        .captures_iter(content)
        .filter_map(|cap| {
            let attrs = attrs_from_args(&cap[1]);
            let dep_name = attrs.get("name")?.clone();
            let repository = attrs.get("repository")?.clone();
            let registry = attrs.get("registry").cloned();
            let package_name = registry
                .as_ref()
                .map(|registry| format!("{registry}/{repository}"))
                .unwrap_or(repository);
            let registry_urls = registry
                .map(|registry| vec![format!("https://{registry}")])
                .unwrap_or_default();

            Some(BazelRulesImgPullDep {
                dep_name,
                package_name,
                current_value: attrs.get("tag").cloned(),
                current_digest: attrs.get("digest").cloned(),
                registry_urls,
                datasource: "docker",
                dep_type: "rules_img_pull",
            })
        })
        .collect()
}

fn collect_bazelrc_registry_urls(
    path: &str,
    files: &[(&str, &str)],
    read_files: &mut std::collections::BTreeSet<String>,
    registry_urls: &mut Vec<String>,
) {
    if !read_files.insert(path.to_owned()) {
        return;
    }
    let Some((_, content)) = files.iter().find(|(file_path, _)| *file_path == path) else {
        return;
    };

    for line in content.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(import_path) = bazelrc_import_path(line) {
            if let Some(local_path) = workspace_bazelrc_path(import_path) {
                collect_bazelrc_registry_urls(&local_path, files, read_files, registry_urls);
            }
            continue;
        }

        let Some((command, options)) = line.split_once(char::is_whitespace) else {
            continue;
        };
        if command.contains(':') {
            continue;
        }

        registry_urls.extend(bazelrc_registry_options(options));
    }
}

fn bazelrc_import_path(line: &str) -> Option<&str> {
    let (kind, path) = line.split_once(char::is_whitespace)?;
    if kind == "import" || kind == "try-import" {
        Some(path.trim())
    } else {
        None
    }
}

fn workspace_bazelrc_path(path: &str) -> Option<String> {
    path.strip_prefix("%workspace%/")
        .map(str::to_owned)
        .or_else(|| (!path.starts_with('/')).then(|| path.to_owned()))
}

fn bazelrc_registry_options(options: &str) -> Vec<String> {
    let parts = options.split_whitespace().collect::<Vec<_>>();
    let mut registries = Vec::new();
    let mut index = 0;
    while index < parts.len() {
        let part = parts[index];
        if let Some(value) = part.strip_prefix("--registry=") {
            registries.push(strip_quote_wrappers(value));
        } else if part == "--registry"
            && let Some(value) = parts.get(index + 1)
        {
            registries.push(strip_quote_wrappers(value));
            index += 1;
        }
        index += 1;
    }
    registries
}

fn strip_quote_wrappers(value: &str) -> String {
    value
        .strip_prefix(['"', '\''])
        .unwrap_or(value)
        .strip_suffix(['"', '\''])
        .unwrap_or(value)
        .to_owned()
}

fn apply_registry_alias(image: &str, registry_aliases: &[(&str, &str)]) -> String {
    let Some((registry, rest)) = image.split_once('/') else {
        return image.to_owned();
    };
    registry_aliases
        .iter()
        .find_map(|(from, to)| {
            if *from == registry {
                Some(format!("{to}/{rest}"))
            } else {
                None
            }
        })
        .unwrap_or_else(|| image.to_owned())
}

fn github_package_name(url: &str) -> Option<String> {
    let rest = url
        .strip_prefix("https://github.com/")
        .or_else(|| url.strip_prefix("git@github.com:"))?;
    let rest = rest.strip_suffix(".git").unwrap_or(rest);
    let mut parts = rest.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    if owner.is_empty() || repo.is_empty() {
        None
    } else {
        Some(format!("{owner}/{repo}"))
    }
}

/// Strip `# comment` lines from Starlark content.
fn strip_comments(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            if let Some(pos) = line.find('#') {
                &line[..pos]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse a Starlark boolean string.
///
/// Mirrors `lib/modules/manager/bazel-module/parser/starlark.ts` `asBoolean()`.
pub fn starlark_as_boolean(value: &str) -> Result<bool, String> {
    match value {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(format!("Invalid Starlark boolean string: {value}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bazelrc_option(name: &str, value: Option<&str>) -> BazelrcOption {
        BazelrcOption {
            name: name.to_owned(),
            value: value.map(str::to_owned),
        }
    }

    fn bazelrc_command(
        command: &str,
        options: Vec<BazelrcOption>,
        config: Option<&str>,
    ) -> BazelrcEntry {
        BazelrcEntry::Command {
            command: command.to_owned(),
            options,
            config: config.map(str::to_owned),
        }
    }

    fn bazelrc_import(path: &str, is_try: bool) -> BazelrcEntry {
        BazelrcEntry::Import {
            path: path.to_owned(),
            is_try,
        }
    }

    // Ported: "parse($a)" — bazel-module/bazelrc.spec.ts line 35
    #[test]
    fn bazelrc_option_parse_cases() {
        let cases = [
            (
                "--show_timestamps",
                vec![bazelrc_option("show_timestamps", None)],
            ),
            (
                "--host_jvm_args=-XX:-UseParallelGC",
                vec![bazelrc_option("host_jvm_args", Some("-XX:-UseParallelGC"))],
            ),
            (
                "--host_jvm_args=",
                vec![bazelrc_option("host_jvm_args", Some(""))],
            ),
            ("--jobs 600", vec![bazelrc_option("jobs", Some("600"))]),
        ];

        for (input, expected) in cases {
            assert_eq!(parse_bazelrc_options(input), expected);
        }
    }

    // Ported: "getOption" — bazel-module/bazelrc.spec.ts line 51
    #[test]
    fn bazelrc_command_entry_get_option() {
        let opt0 = bazelrc_option("show_timestamps", None);
        let opt1 = bazelrc_option("keep_going", None);
        let opt2 = bazelrc_option("jobs", Some("600"));
        let cmd_entry = bazelrc_command("build", vec![opt0.clone(), opt1, opt2.clone()], None);

        assert_eq!(cmd_entry.get_option("does_not_exist"), None);
        assert_eq!(cmd_entry.get_option(&opt0.name), Some(&opt0));
        assert_eq!(cmd_entry.get_option(&opt2.name), Some(&opt2));
    }

    // Ported: "parse" — bazel-module/bazelrc.spec.ts line 62
    #[test]
    fn bazelrc_parse_entries() {
        let input = r#"
        # Bob's Bazel option defaults

        startup --host_jvm_args=-XX:-UseParallelGC
        import /home/bobs_project/bazelrc
        build --show_timestamps --keep_going --jobs 600
        build --color=yes
        query --keep_going

        # Definition of --config=memcheck
        build:memcheck --strip=never --test_timeout=3600

        try-import %workspace%/local.bazelrc
      "#;

        assert_eq!(
            parse_bazelrc(input),
            vec![
                bazelrc_command(
                    "startup",
                    vec![bazelrc_option("host_jvm_args", Some("-XX:-UseParallelGC"))],
                    None,
                ),
                bazelrc_import("/home/bobs_project/bazelrc", false),
                bazelrc_command(
                    "build",
                    vec![
                        bazelrc_option("show_timestamps", None),
                        bazelrc_option("keep_going", None),
                        bazelrc_option("jobs", Some("600")),
                    ],
                    None,
                ),
                bazelrc_command("build", vec![bazelrc_option("color", Some("yes"))], None),
                bazelrc_command("query", vec![bazelrc_option("keep_going", None)], None),
                bazelrc_command(
                    "build",
                    vec![
                        bazelrc_option("strip", Some("never")),
                        bazelrc_option("test_timeout", Some("3600")),
                    ],
                    Some("memcheck"),
                ),
                bazelrc_import("%workspace%/local.bazelrc", true),
            ]
        );
    }

    // Ported: "should return original value if no workspace path" — bazel-module/bazelrc.spec.ts line 304
    #[test]
    fn bazelrc_expand_workspace_path_returns_original_without_workspace_path() {
        assert_eq!(
            expand_bazelrc_workspace_path("--some-option", "/workspace", |_| false),
            Some("--some-option".to_owned())
        );
    }

    // Ported: "should expand valid workspace path" — bazel-module/bazelrc.spec.ts line 310
    #[test]
    fn bazelrc_expand_workspace_path_expands_valid_workspace_path() {
        assert_eq!(
            expand_bazelrc_workspace_path("%workspace%/some/path", "/workspace", |path| {
                path == "/workspace" || path == "/workspace/some/path"
            }),
            Some("/workspace/some/path".to_owned())
        );
    }

    // Ported: "should throw error for invalid workspace path" — bazel-module/bazelrc.spec.ts line 320
    #[test]
    fn bazelrc_expand_workspace_path_returns_none_for_invalid_workspace_path() {
        assert_eq!(
            expand_bazelrc_workspace_path("%workspace%/../../outside", "/workspace", |_| false),
            None
        );
    }

    // Ported: "should handle options without values" — bazel-module/bazelrc.spec.ts line 328
    #[test]
    fn bazelrc_sanitize_options_handles_options_without_values() {
        let options = vec![bazelrc_option("build", None)];
        assert_eq!(
            sanitize_bazelrc_options(&options, "/workspace", |_| false),
            options
        );
    }

    // Ported: "should expand valid workspace paths" — bazel-module/bazelrc.spec.ts line 333
    #[test]
    fn bazelrc_sanitize_options_expands_valid_workspace_paths() {
        let options = vec![
            bazelrc_option("build", Some("%workspace%/some/path")),
            bazelrc_option("test", Some("%workspace%/other/path")),
        ];
        let result = sanitize_bazelrc_options(&options, "/workspace", |path| {
            path == "/workspace"
                || path == "/workspace/some/path"
                || path == "/workspace/other/path"
        });
        assert_eq!(
            result,
            vec![
                bazelrc_option("build", Some("/workspace/some/path")),
                bazelrc_option("test", Some("/workspace/other/path")),
            ]
        );
    }

    // Ported: "should throw error for invalid workspace paths" — bazel-module/bazelrc.spec.ts line 352
    #[test]
    fn bazelrc_sanitize_options_drops_invalid_workspace_paths() {
        let options = vec![
            bazelrc_option("build", Some("%workspace%/valid/path")),
            bazelrc_option("test", Some("%workspace%/../../invalid")),
        ];
        assert!(sanitize_bazelrc_options(&options, "/workspace", |_| false).is_empty());
    }

    fn cmd(command: &str, opts: &[(&str, Option<&str>)]) -> BazelrcEntry {
        BazelrcEntry::Command {
            command: command.to_owned(),
            options: opts
                .iter()
                .map(|(n, v)| BazelrcOption {
                    name: (*n).to_owned(),
                    value: v.map(|s| s.to_owned()),
                })
                .collect(),
            config: None,
        }
    }

    // Ported: "when .bazelrc does not exist" — bazel-module/bazelrc.spec.ts line 103
    #[test]
    fn bazelrc_read_bazelrc_not_exist() {
        let files: &[(&str, Option<&str>)] = &[(".bazelrc", None)];
        let valid: &[&str] = &[".bazelrc"];
        let result = read_bazelrc_from_files(".", files, valid).unwrap();
        assert!(result.is_empty());
    }

    // Ported: "when .bazelrc has invalid lines" — bazel-module/bazelrc.spec.ts line 110
    #[test]
    fn bazelrc_read_invalid_lines_ignored() {
        let content =
            "// This is not a valid comment\nbuild --show_timestamps --keep_going --jobs 600";
        let files = &[(".bazelrc", Some(content))];
        let valid = &[".bazelrc"];
        let result = read_bazelrc_from_files(".", files, valid).unwrap();
        assert_eq!(
            result,
            vec![cmd(
                "build",
                &[
                    ("show_timestamps", None),
                    ("keep_going", None),
                    ("jobs", Some("600")),
                ]
            )]
        );
    }

    // Ported: "when .bazelrc has no imports" — bazel-module/bazelrc.spec.ts line 128
    #[test]
    fn bazelrc_read_no_imports() {
        let content = "# This comment should be ignored\nbuild --show_timestamps --keep_going --jobs 600\nbuild --color=yes";
        let files = &[(".bazelrc", Some(content))];
        let valid = &[".bazelrc"];
        let result = read_bazelrc_from_files(".", files, valid).unwrap();
        assert_eq!(
            result,
            vec![
                cmd(
                    "build",
                    &[
                        ("show_timestamps", None),
                        ("keep_going", None),
                        ("jobs", Some("600")),
                    ]
                ),
                cmd("build", &[("color", Some("yes"))]),
            ]
        );
    }

    // Ported: "when .bazelrc has import and try-import, try-import exists" — bazel-module/bazelrc.spec.ts line 148
    #[test]
    fn bazelrc_read_import_and_try_import_both_exist() {
        let files = &[
            (
                ".bazelrc",
                Some("import %workspace%/shared.bazelrc\ntry-import %workspace%/local.bazelrc"),
            ),
            ("shared.bazelrc", Some("build --show_timestamps")),
            ("local.bazelrc", Some("build --color=yes")),
        ];
        let valid = &[".bazelrc", "local.bazelrc", "shared.bazelrc"];
        let result = read_bazelrc_from_files(".", files, valid).unwrap();
        assert_eq!(
            result,
            vec![
                cmd("build", &[("show_timestamps", None)]),
                cmd("build", &[("color", Some("yes"))]),
            ]
        );
    }

    // Ported: "when .bazelrc has import and try-import, try-import does not exist" — bazel-module/bazelrc.spec.ts line 173
    #[test]
    fn bazelrc_read_try_import_not_exist_skipped() {
        let files = &[
            (
                ".bazelrc",
                Some("build --jobs 600\ntry-import %workspace%/local.bazelrc"),
            ),
            ("local.bazelrc", None),
        ];
        let valid = &[".bazelrc"];
        let result = read_bazelrc_from_files(".", files, valid).unwrap();
        assert_eq!(result, vec![cmd("build", &[("jobs", Some("600"))])]);
    }

    // Ported: "when .bazelrc multi-level import" — bazel-module/bazelrc.spec.ts line 188
    #[test]
    fn bazelrc_read_multi_level_import() {
        let files = &[
            (
                ".bazelrc",
                Some("import %workspace%/shared.bazelrc\nbuild --jobs 600"),
            ),
            ("shared.bazelrc", Some("import %workspace%/foo.bazelrc")),
            ("foo.bazelrc", Some("build --show_timestamps")),
        ];
        let valid = &[".bazelrc", "foo.bazelrc", "shared.bazelrc"];
        let result = read_bazelrc_from_files(".", files, valid).unwrap();
        assert_eq!(
            result,
            vec![
                cmd("build", &[("show_timestamps", None)]),
                cmd("build", &[("jobs", Some("600"))]),
            ]
        );
    }

    // Ported: "when bazlerc files recursively import each other" — bazel-module/bazelrc.spec.ts line 213
    #[test]
    fn bazelrc_read_cycle_returns_error() {
        let files = &[
            (
                ".bazelrc",
                Some("import %workspace%/shared.bazelrc\nbuild --jobs 600"),
            ),
            ("shared.bazelrc", Some("import %workspace%/foo.bazelrc")),
            ("foo.bazelrc", Some("import %workspace%/shared.bazelrc")),
        ];
        let valid = &[".bazelrc", "foo.bazelrc", "shared.bazelrc"];
        let result = read_bazelrc_from_files(".", files, valid);
        assert_eq!(
            result,
            Err("Attempted to read a bazelrc multiple times. file: shared.bazelrc".to_owned())
        );
    }

    // Ported: "when .bazelrc refers to a non-local file" — bazel-module/bazelrc.spec.ts line 239
    #[test]
    fn bazelrc_read_non_local_import_skipped() {
        let files = &[(
            ".bazelrc",
            Some("import /non-local.bazelrc\nbuild --jobs 600"),
        )];
        let valid = &[".bazelrc"];
        let result = read_bazelrc_from_files(".", files, valid).unwrap();
        assert_eq!(result, vec![cmd("build", &[("jobs", Some("600"))])]);
    }

    // Ported: "when bazelrc has %workspace% paths in options" — bazel-module/bazelrc.spec.ts line 255
    #[test]
    fn bazelrc_read_workspace_paths_in_options() {
        let workspace_dir = "/tmp/workspace";
        let content = "build --output_base=%workspace%/bazel-out";
        let files = &[("/tmp/workspace/.bazelrc", Some(content))];
        let valid = &["/tmp/workspace/.bazelrc", "/tmp/workspace/bazel-out"];
        let result = read_bazelrc_from_files(workspace_dir, files, valid).unwrap();
        assert_eq!(
            result,
            vec![cmd(
                "build",
                &[("output_base", Some("/tmp/workspace/bazel-out"))]
            )]
        );
    }

    // Ported: "when bazelrc has %workspace% paths in imported files" — bazel-module/bazelrc.spec.ts line 274
    #[test]
    fn bazelrc_read_workspace_paths_in_imported_files() {
        let workspace_dir = "/tmp/workspace";
        let files = &[
            (
                "/tmp/workspace/.bazelrc",
                Some("import %workspace%/shared.bazelrc"),
            ),
            (
                "/tmp/workspace/shared.bazelrc",
                Some(
                    "build --output_base=%workspace%/bazel-out\nbuild --test_output=%workspace%/test-results",
                ),
            ),
        ];
        let valid = &[
            "/tmp/workspace/.bazelrc",
            "/tmp/workspace/shared.bazelrc",
            "/tmp/workspace/bazel-out",
            "/tmp/workspace/test-results",
        ];
        let result = read_bazelrc_from_files(workspace_dir, files, valid).unwrap();
        assert_eq!(
            result,
            vec![
                cmd(
                    "build",
                    &[("output_base", Some("/tmp/workspace/bazel-out"))]
                ),
                cmd(
                    "build",
                    &[("test_output", Some("/tmp/workspace/test-results"))]
                ),
            ]
        );
    }

    // Ported: ".string()" — bazel-module/parser/fragments.spec.ts line 13
    #[test]
    fn fragment_string_constructor() {
        assert_eq!(
            fragment_string("hello"),
            BazelFragment::String {
                value: "hello".to_owned(),
                is_complete: true,
            }
        );
    }

    // Ported: ".boolean()" — bazel-module/parser/fragments.spec.ts line 19
    #[test]
    fn fragment_boolean_constructor() {
        assert_eq!(
            fragment_boolean(true),
            BazelFragment::Boolean {
                value: true,
                is_complete: true,
            }
        );
    }

    // Ported: ".rule()" — bazel-module/parser/fragments.spec.ts line 25
    #[test]
    fn fragment_rule_constructor() {
        let children = BTreeMap::from([("name".to_owned(), fragment_string("bar"))]);
        assert_eq!(
            fragment_rule("foo", children.clone(), true),
            BazelFragment::Rule {
                rule: "foo".to_owned(),
                children,
                is_complete: true,
            }
        );
    }

    // Ported: ".extensionTag()" — bazel-module/parser/fragments.spec.ts line 37
    #[test]
    fn fragment_extension_tag_constructor() {
        let children = BTreeMap::from([("name".to_owned(), fragment_string("bar"))]);
        assert_eq!(
            fragment_extension_tag("ext", "ext_01", "tag", 0, children.clone(), None, true),
            BazelFragment::ExtensionTag {
                extension: "ext".to_owned(),
                raw_extension: "ext_01".to_owned(),
                tag: "tag".to_owned(),
                offset: 0,
                children,
                raw_string: None,
                is_complete: true,
            }
        );
    }

    // Ported: ".preparedExtensionTag()" — bazel-module/parser/fragments.spec.ts line 56
    #[test]
    fn fragment_prepared_extension_tag_constructor() {
        assert_eq!(
            fragment_prepared_extension_tag("ext", "ext_01", 0),
            BazelFragment::PreparedExtensionTag {
                extension: "ext".to_owned(),
                raw_extension: "ext_01".to_owned(),
                offset: 0,
                is_complete: false,
            }
        );
    }

    // Ported: ".attribute()" — bazel-module/parser/fragments.spec.ts line 65
    #[test]
    fn fragment_attribute_constructor() {
        assert_eq!(
            fragment_attribute("name", Some(fragment_string("foo")), true),
            BazelFragment::Attribute {
                name: "name".to_owned(),
                value: Some(Box::new(fragment_string("foo"))),
                is_complete: true,
            }
        );
    }

    // Ported: ".array()" — bazel-module/parser/fragments.spec.ts line 73
    #[test]
    fn fragment_array_constructor() {
        assert_eq!(
            fragment_array(vec![fragment_string("foo")], true),
            BazelFragment::Array {
                items: vec![fragment_string("foo")],
                is_complete: true,
            }
        );
    }

    // Ported: ".isValue($a)" — bazel-module/parser/fragments.spec.ts line 80
    #[test]
    fn fragment_is_value_matches_renovate_value_fragments() {
        let cases = [
            (fragment_string("hello"), true),
            (fragment_boolean(true), true),
            (fragment_array(Vec::new(), false), true),
            (fragment_rule("dummy", BTreeMap::new(), false), false),
            (
                fragment_extension_tag("ext", "ext_01", "tag", 0, BTreeMap::new(), None, false),
                false,
            ),
            (fragment_prepared_extension_tag("ext", "ext_01", 0), false),
        ];

        for (fragment, expected) in cases {
            assert_eq!(fragment_is_value(&fragment), expected);
        }
    }

    // Ported: ".isPrimitive($a)" — bazel-module/parser/fragments.spec.ts line 92
    #[test]
    fn fragment_is_primitive_matches_renovate_primitive_fragments() {
        let cases = [
            (fragment_string("hello"), true),
            (fragment_boolean(true), true),
            (fragment_array(Vec::new(), false), false),
            (fragment_rule("dummy", BTreeMap::new(), false), false),
            (
                fragment_extension_tag("ext", "ext_01", "tag", 0, BTreeMap::new(), None, false),
                false,
            ),
            (fragment_prepared_extension_tag("ext", "ext_01", 0), false),
        ];

        for (fragment, expected) in cases {
            assert_eq!(fragment_is_primitive(&fragment), expected);
        }
    }

    // Ported: "throws if there is no current" — bazel-module/parser/context.spec.ts line 7
    #[test]
    fn bazel_ctx_start_extension_tag_errors_without_current() {
        let mut ctx = BazelCtx::new("");
        assert_eq!(
            ctx.start_extension_tag("install"),
            Err(BazelCtxError::Message("Requested current, but no value."))
        );
    }

    // Ported: "throws if the current is not a prepared extension tag" — bazel-module/parser/context.spec.ts line 13
    #[test]
    fn bazel_ctx_start_extension_tag_errors_for_wrong_current() {
        let mut ctx = BazelCtx::new("");
        ctx.start_rule("foo");
        assert_eq!(
            ctx.start_extension_tag("install"),
            Err(BazelCtxError::Message(
                "Requested current prepared extension tag, but does not exist."
            ))
        );
    }

    // Ported: "throws if the current is not an extension tag" — bazel-module/parser/context.spec.ts line 23
    #[test]
    fn bazel_ctx_end_extension_tag_errors_for_wrong_current() {
        let mut ctx = BazelCtx::new("");
        ctx.start_rule("foo");
        assert_eq!(
            ctx.end_extension_tag(0),
            Err(BazelCtxError::Message(
                "Requested current extension tag, but does not exist."
            ))
        );
    }

    // Ported: "throws on missing current" — bazel-module/parser/context.spec.ts line 30
    #[test]
    fn bazel_ctx_end_rule_errors_without_current() {
        let mut ctx = BazelCtx::new("");
        assert_eq!(
            ctx.end_rule(),
            Err(BazelCtxError::Message("Requested current, but no value."))
        );
    }

    // Ported: "throws on unbalanced endRule" — bazel-module/parser/context.spec.ts line 37
    #[test]
    fn bazel_ctx_end_rule_errors_when_current_is_array() {
        let mut ctx = BazelCtx::new("");
        ctx.start_rule("foo");
        ctx.start_array();
        assert_eq!(
            ctx.end_rule(),
            Err(BazelCtxError::Message(
                "Requested current rule, but does not exist."
            ))
        );
    }

    // Ported: "throws on unbalanced endArray" — bazel-module/parser/context.spec.ts line 44
    #[test]
    fn bazel_ctx_end_array_errors_when_current_is_rule() {
        let mut ctx = BazelCtx::new("");
        ctx.start_array();
        ctx.start_rule("dummy");
        assert_eq!(
            ctx.end_array(),
            Err(BazelCtxError::Message(
                "Requested current array, but does not exist."
            ))
        );
    }

    // Ported: "throws if add an attribute without a parent" — bazel-module/parser/context.spec.ts line 51
    #[test]
    fn bazel_ctx_add_string_to_parentless_attribute_errors() {
        let mut ctx = BazelCtx::new("");
        ctx.start_attribute("name").unwrap();
        let expected_current = fragment_attribute("name", Some(fragment_string("chicken")), true);
        assert_eq!(
            ctx.add_string("chicken"),
            Err(BazelCtxError::Processing(Box::new(
                BazelCtxProcessingError::new(expected_current, None)
            )))
        );
    }

    // Ported: "throws if current use repo rule does not exist" — bazel-module/parser/context.spec.ts line 60
    #[test]
    fn bazel_ctx_end_use_repo_rule_errors_for_wrong_current() {
        let mut ctx = BazelCtx::new("");
        ctx.start_rule("foo");
        assert_eq!(
            ctx.end_use_repo_rule(),
            Err(BazelCtxError::Message(
                "Requested current use repo rule, but does not exist."
            ))
        );
    }

    // Ported: "throws if current repo rule call does not exist" — bazel-module/parser/context.spec.ts line 67
    #[test]
    fn bazel_ctx_end_repo_rule_call_errors_for_wrong_current() {
        let mut ctx = BazelCtx::new("");
        ctx.start_rule("foo");
        assert_eq!(
            ctx.end_repo_rule_call(0),
            Err(BazelCtxError::Message(
                "Requested current repo rule call, but does not exist."
            ))
        );
    }

    // Ported: "creates CtxProcessingError with parent type" — bazel-module/parser/context.spec.ts line 74
    #[test]
    fn bazel_ctx_processing_error_records_current_and_parent_type() {
        let current = fragment_attribute("name", None, false);
        let parent = fragment_rule("parent", BTreeMap::new(), false);
        let error = BazelCtxProcessingError::new(current.clone(), Some(parent.clone()));

        assert_eq!(
            error.message,
            "Invalid context state. current: attribute, parent: rule"
        );
        assert_eq!(error.current, current);
        assert_eq!(error.parent, Some(parent));
    }

    // Ported: "returns bazel_dep and git_override dependencies" — bazel-module/extract.spec.ts line 54
    #[test]
    fn extracts_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_go", version = "0.41.0")
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rules_go");
        assert_eq!(deps[0].current_value, "0.41.0");
        assert!(!deps[0].dev_dependency);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "returns bazel_dep and git_override dependencies" — bazel-module/extract.spec.ts line 54
    #[test]
    fn extracts_dev_dependency() {
        let content = r#"bazel_dep(name = "gazelle", version = "0.32.0", dev_dependency = True)"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].dev_dependency);
    }

    // Ported: "returns bazel_dep and git_override dependencies" — bazel-module/extract.spec.ts line 54
    #[test]
    fn extracts_multiline_dep() {
        let content = r#"
bazel_dep(
    name = "rules_python",
    version = "0.24.0",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rules_python");
        assert_eq!(deps[0].current_value, "0.24.0");
    }

    // Ported: "returns bazel_dep and git_override dependencies" — bazel-module/extract.spec.ts line 54
    #[test]
    fn multiple_deps() {
        let content = r#"
bazel_dep(name = "rules_go", version = "0.41.0")
bazel_dep(name = "gazelle", version = "0.32.0")
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].name, "rules_go");
        assert_eq!(deps[1].name, "gazelle");
    }

    // Ported: "returns bazel_dep with no version and git_override" — bazel-module/extract.spec.ts line 95
    #[test]
    fn dep_without_version_skipped() {
        let content = r#"bazel_dep(name = "rules_go")"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(BazelSkipReason::UnspecifiedVersion)
        );
    }

    // Ported: "returns crate.spec dependencies" — bazel-module/extract.spec.ts line 377
    #[test]
    fn extracts_crate_spec_dependencies() {
        let input = r#"
crate.spec(
    package = "axum",
    version = "0.8.4",
)
crate.spec(
    package = "tokio",
    version = "1.45.1",
    features = [
        "full",
    ],
)
crate.spec(
    package = "custom_crate",
    git = "https://github.com/example/custom_crate.git",
    tag = "v1.0.0",
)
crate.spec(
    package = "local_crate",
    path = "/var/crate",
)
crate.spec(
    package = "no_version_crate",
)
"#;
        let deps = extract_crate_specs(input);
        assert_eq!(deps.len(), 5);
        assert_eq!(deps[0].name, "axum");
        assert_eq!(deps[0].current_value, "0.8.4");
        assert_eq!(deps[0].datasource, "crate");
        assert!(deps[0].nested_version);
        assert_eq!(deps[1].name, "tokio");
        assert_eq!(deps[1].current_value, "1.45.1");
        assert!(deps[1].nested_version);
        assert_eq!(deps[2].name, "custom_crate");
        assert_eq!(deps[2].current_value, "v1.0.0");
        assert_eq!(deps[2].datasource, "github-tags");
        assert_eq!(
            deps[2].package_name.as_deref(),
            Some("example/custom_crate")
        );
        assert_eq!(deps[2].registry_urls, vec!["https://github.com"]);
        assert!(!deps[2].nested_version);
        assert_eq!(deps[3].name, "local_crate");
        assert_eq!(deps[3].skip_reason, Some(BazelSkipReason::PathDependency));
        assert_eq!(deps[4].name, "no_version_crate");
        assert_eq!(
            deps[4].skip_reason,
            Some(BazelSkipReason::InvalidDependencySpecification)
        );
    }

    // Ported: "returns maven.install and maven.artifact dependencies" — bazel-module/extract.spec.ts line 453
    #[test]
    fn extracts_maven_install_and_artifact_dependencies() {
        let input = r#"
maven.artifact(
    artifact = "core.specs.alpha",
    exclusions = ["org.clojure:clojure"],
    group = "org.clojure",
    version = "0.2.56",
)

maven.install(
    artifacts = [
        "junit:junit:4.13.2",
        "com.google.guava:guava:31.1-jre",
    ],
    lock_file = "//:maven_install.json",
    repositories = [
        "https://repo1.maven.org/maven2/",
    ],
    version_conflict_policy = "pinned",
)
"#;
        let deps = extract_maven_deps(input);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[0].current_value, "4.13.2");
        assert_eq!(deps[0].dep_type, "maven_install");
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
        assert_eq!(deps[1].dep_name, "com.google.guava:guava");
        assert_eq!(deps[1].current_value, "31.1-jre");
        assert_eq!(deps[1].dep_type, "maven_install");
        assert_eq!(
            deps[1].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
        assert_eq!(deps[2].dep_name, "org.clojure:core.specs.alpha");
        assert_eq!(deps[2].current_value, "0.2.56");
        assert_eq!(deps[2].dep_type, "maven_install");
        assert_eq!(
            deps[2].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
    }

    // Ported: "returns oci.pull dependencies" — bazel-module/extract.spec.ts line 507
    #[test]
    fn extracts_oci_pull_dependency() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    digest = "sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
    tag = "1.27.1",
)
"#;
        let deps = extract_oci_pull_deps(input, &[]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, "docker");
        assert_eq!(deps[0].dep_type, "oci_pull");
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(deps[0].package_name, "index.docker.io/library/nginx");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.27.1"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720")
        );
    }

    // Ported: "returns oci.pull dependencies without tags" — bazel-module/extract.spec.ts line 544
    #[test]
    fn extracts_oci_pull_dependency_without_tag() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    digest = "sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
)
"#;
        let deps = extract_oci_pull_deps(input, &[]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(deps[0].package_name, "index.docker.io/library/nginx");
        assert_eq!(deps[0].current_value, None);
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720")
        );
    }

    // Ported: "returns oci.pull dependencies with tag only (no digest)" — bazel-module/extract.spec.ts line 578
    #[test]
    fn extracts_oci_pull_dependency_with_tag_only() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
    tag = "1.27.1",
)
"#;
        let deps = extract_oci_pull_deps(input, &[]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(deps[0].package_name, "index.docker.io/library/nginx");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.27.1"));
        assert_eq!(deps[0].current_digest, None);
    }

    // Ported: "returns oci.pull dependencies without tag or digest" — bazel-module/extract.spec.ts line 611
    #[test]
    fn extracts_oci_pull_dependency_without_tag_or_digest() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
)
"#;
        let deps = extract_oci_pull_deps(input, &[]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(deps[0].package_name, "index.docker.io/library/nginx");
        assert_eq!(deps[0].current_value, None);
        assert_eq!(deps[0].current_digest, None);
    }

    // Ported: "returns oci.pull dependencies with registryAliases" — bazel-module/extract.spec.ts line 641
    #[test]
    fn extracts_oci_pull_dependency_with_registry_alias() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    digest = "sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
    tag = "1.27.1",
)
"#;
        let deps = extract_oci_pull_deps(
            input,
            &[("index.docker.io", "my-docker-mirror.registry.com")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(
            deps[0].package_name,
            "my-docker-mirror.registry.com/library/nginx"
        );
        assert_eq!(deps[0].current_value.as_deref(), Some("1.27.1"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720")
        );
    }

    // Ported: "returns oci.pull dependencies with registryAliases with multiple segments" — bazel-module/extract.spec.ts line 682
    #[test]
    fn extracts_oci_pull_dependency_with_multisegment_registry_alias() {
        let input = r#"
oci.pull(
    name = "custom_image",
    digest = "sha256:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    image = "quay.io/myorg/myapp",
    platforms = ["linux/amd64"],
    tag = "v2.0.0",
)
"#;
        let deps = extract_oci_pull_deps(input, &[("quay.io", "my-registry.com/mirror/quay.io")]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "custom_image");
        assert_eq!(
            deps[0].package_name,
            "my-registry.com/mirror/quay.io/myorg/myapp"
        );
        assert_eq!(deps[0].current_value.as_deref(), Some("v2.0.0"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        );
    }

    // Ported: "returns maven.install and bazel_dep dependencies together" — bazel-module/extract.spec.ts line 723
    #[test]
    fn extracts_maven_install_and_bazel_dep_together() {
        let input = r#"
bazel_dep(name = "bazel_jar_jar", version = "0.1.0")

maven = use_extension("@rules_jvm_external//:extensions.bzl", "maven")

maven.install(
    artifacts = [
        "junit:junit:4.13.2",
        "com.google.guava:guava:31.1-jre",
    ],
    lock_file = "//:maven_install.json",
    repositories = [
        "https://repo1.maven.org/maven2/",
    ],
    version_conflict_policy = "pinned",
)
"#;
        let bazel_deps = extract(input);
        assert_eq!(bazel_deps.len(), 1);
        assert_eq!(bazel_deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(bazel_deps[0].name, "bazel_jar_jar");
        assert_eq!(bazel_deps[0].current_value, "0.1.0");
        assert!(bazel_deps[0].skip_reason.is_none());

        let maven_deps = extract_maven_deps(input);
        assert_eq!(maven_deps.len(), 2);
        assert_eq!(maven_deps[0].dep_name, "junit:junit");
        assert_eq!(maven_deps[0].current_value, "4.13.2");
        assert_eq!(maven_deps[0].dep_type, "maven_install");
        assert_eq!(
            maven_deps[0].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
        assert_eq!(maven_deps[1].dep_name, "com.google.guava:guava");
        assert_eq!(maven_deps[1].current_value, "31.1-jre");
        assert_eq!(maven_deps[1].dep_type, "maven_install");
        assert_eq!(
            maven_deps[1].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
    }

    // Ported: "returns git_repository dependencies with digest" — bazel-module/extract.spec.ts line 772
    #[test]
    fn extracts_git_repository_dependency_with_digest() {
        let input = r#"
git_repository(
    name = "rules_foo",
    commit = "850cb49c8649e463b80ef7984e7c744279746170",
    remote = "https://github.com/example/rules_foo.git"
)
"#;
        let deps = extract_git_repository_deps(input);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "git_repository");
        assert_eq!(deps[0].dep_name, "rules_foo");
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].package_name.as_deref(), Some("example/rules_foo"));
        assert_eq!(deps[0].current_value, None);
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("850cb49c8649e463b80ef7984e7c744279746170")
        );
    }

    // Ported: "returns git_repository dependencies with tag" — bazel-module/extract.spec.ts line 796
    #[test]
    fn extracts_git_repository_dependency_with_tag() {
        let input = r#"
git_repository(
    name = "rules_foo",
    tag = "1.2.3",
    remote = "https://github.com/example/rules_foo.git"
)
"#;
        let deps = extract_git_repository_deps(input);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "git_repository");
        assert_eq!(deps[0].dep_name, "rules_foo");
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].package_name.as_deref(), Some("example/rules_foo"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.2.3"));
        assert_eq!(deps[0].current_digest, None);
    }

    // Ported: "returns new_git_repository dependencies" — bazel-module/extract.spec.ts line 820
    #[test]
    fn extracts_new_git_repository_dependency() {
        let input = r#"
new_git_repository(
    name = "rules_foo",
    commit = "850cb49c8649e463b80ef7984e7c744279746170",
    remote = "https://github.com/example/rules_foo.git",
    tag = "1.2.3"
)
"#;
        let deps = extract_git_repository_deps(input);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "new_git_repository");
        assert_eq!(deps[0].dep_name, "rules_foo");
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].package_name.as_deref(), Some("example/rules_foo"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.2.3"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("850cb49c8649e463b80ef7984e7c744279746170")
        );
    }

    // Ported: "handles a real-world MODULE.bazel file (rules_sh)" — bazel-module/extract.spec.ts line 846
    #[test]
    fn extracts_rules_sh_real_world_module_bazel() {
        let input = r#"
module(
    name = "rules_sh",
    version = "0.5.0",
    compatibility_level = 0,
)
bazel_dep(name = "bazel_skylib", version = "1.2.1")
bazel_dep(name = "platforms", version = "0.0.8")
bazel_dep(name = "stardoc", version = "0.6.2", dev_dependency = True, repo_name = "io_bazel_stardoc")
sh_configure = use_extension("//bzlmod:extensions.bzl", "sh_configure")
use_repo(sh_configure, "local_posix_config", "rules_sh_shim_exe")
register_toolchains("@local_posix_config//...")
"#;
        let deps = extract(input);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "bazel_skylib");
        assert_eq!(deps[0].current_value, "1.2.1");
        assert!(!deps[0].dev_dependency);
        assert_eq!(deps[1].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[1].name, "platforms");
        assert_eq!(deps[1].current_value, "0.0.8");
        assert!(!deps[1].dev_dependency);
        assert_eq!(deps[2].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[2].name, "stardoc");
        assert_eq!(deps[2].current_value, "0.6.2");
        assert!(deps[2].dev_dependency);
        assert!(deps.iter().all(|dep| dep.skip_reason.is_none()));
    }

    // Ported: "returns dependencies and custom registry URLs when specified in a bazelrc" — bazel-module/extract.spec.ts line 125
    #[test]
    fn extracts_bazelrc_registry_urls_for_module() {
        let module_bazel = r#"bazel_dep(name = "rules_foo", version = "1.2.3")"#;
        let bazelrc = r#"
# .bazelrc

build --registry=https://example.com/custom_registry.git
build --registry=https://github.com/bazelbuild/bazel-central-registry
build --registry='http://example.com/registry-with-single-quotes.git'
build --registry="http://example.com/registry-with-double-quotes.git"

import %workspace%/shared.bazelrc

build --jobs 600

# This file does not exist.
try-import %workspace%/local.bazelrc

# This file does not exist/is outside of the basePath
try-import /does-not-exist/.bazelrc
"#;
        let shared_bazelrc = r#"
# shared.bazelrc

build --show_timestamps

import %workspace%/foo.bazelrc
"#;
        let foo_bazelrc = r#"
# foo.bazelrc

# This should be ignored, because it is registered for the ci configuration.
build:ci --registry=https://example.com/debug_registry.git

build --color=yes
"#;

        let deps = extract(module_bazel);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");

        let registry_urls = extract_bazelrc_registry_urls(&[
            (".bazelrc", bazelrc),
            ("shared.bazelrc", shared_bazelrc),
            ("foo.bazelrc", foo_bazelrc),
        ]);
        assert_eq!(
            registry_urls,
            vec![
                "https://example.com/custom_registry.git",
                "https://github.com/bazelbuild/bazel-central-registry",
                "http://example.com/registry-with-single-quotes.git",
                "http://example.com/registry-with-double-quotes.git",
            ]
        );
    }

    // Ported: "returns rules_img pull dependencies" — bazel-module/extract.spec.ts line 1005
    #[test]
    fn extracts_rules_img_pull_dependency() {
        let input = r#"
bazel_dep(name = "rules_img", version = "0.1.0")
pull = use_repo_rule("@rules_img//img:pull.bzl", "pull")
pull(
    name = "ubuntu",
    digest = "sha256:1e622c5f9ac0c0144d577702ba5f2cce79fc8e3cf89ec88291739cd4eee3b7b9",
    registry = "index.docker.io",
    repository = "library/ubuntu",
    tag = "24.04",
)
"#;
        let bazel_deps = extract(input);
        assert_eq!(bazel_deps.len(), 1);
        assert_eq!(bazel_deps[0].name, "rules_img");
        assert_eq!(bazel_deps[0].current_value, "0.1.0");

        let image_deps = extract_rules_img_pull_deps(input);
        assert_eq!(image_deps.len(), 1);
        assert_eq!(image_deps[0].datasource, "docker");
        assert_eq!(image_deps[0].dep_type, "rules_img_pull");
        assert_eq!(image_deps[0].dep_name, "ubuntu");
        assert_eq!(image_deps[0].package_name, "index.docker.io/library/ubuntu");
        assert_eq!(image_deps[0].current_value.as_deref(), Some("24.04"));
        assert_eq!(
            image_deps[0].current_digest.as_deref(),
            Some("sha256:1e622c5f9ac0c0144d577702ba5f2cce79fc8e3cf89ec88291739cd4eee3b7b9")
        );
        assert_eq!(image_deps[0].registry_urls, vec!["https://index.docker.io"]);
    }

    // Ported: "returns rules_img pull dependencies with custom registry" — bazel-module/extract.spec.ts line 1051
    #[test]
    fn extracts_rules_img_pull_dependency_with_custom_registry() {
        let input = r#"
pull = use_repo_rule("@rules_img//img:pull.bzl", "pull")
pull(
    name = "my_image",
    registry = "my.registry.com",
    repository = "myorg/myimage",
    tag = "v1.2.3",
)
"#;
        let deps = extract_rules_img_pull_deps(input);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "my_image");
        assert_eq!(deps[0].package_name, "my.registry.com/myorg/myimage");
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.2.3"));
        assert_eq!(deps[0].current_digest, None);
        assert_eq!(deps[0].registry_urls, vec!["https://my.registry.com"]);
    }

    // Ported: "returns rules_img pull dependencies with multiple pulls" — bazel-module/extract.spec.ts line 1086
    #[test]
    fn extracts_multiple_rules_img_pull_dependencies() {
        let input = r#"
pull = use_repo_rule("@rules_img//img:pull.bzl", "pull")
pull(
    name = "ubuntu",
    repository = "library/ubuntu",
    tag = "24.04",
)
pull(
    name = "nginx",
    repository = "library/nginx",
    tag = "1.27.1",
    digest = "sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720",
)
"#;
        let deps = extract_rules_img_pull_deps(input);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "ubuntu");
        assert_eq!(deps[0].package_name, "library/ubuntu");
        assert_eq!(deps[0].current_value.as_deref(), Some("24.04"));
        assert_eq!(deps[0].current_digest, None);
        assert!(deps[0].registry_urls.is_empty());
        assert_eq!(deps[1].dep_name, "nginx");
        assert_eq!(deps[1].package_name, "library/nginx");
        assert_eq!(deps[1].current_value.as_deref(), Some("1.27.1"));
        assert_eq!(
            deps[1].current_digest.as_deref(),
            Some("sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720")
        );
    }

    // Ported: "ignores rules_img pull without required fields" — bazel-module/extract.spec.ts line 1141
    #[test]
    fn ignores_rules_img_pull_without_required_fields() {
        let input = r#"
pull = use_repo_rule("@rules_img//img:pull.bzl", "pull")
# Missing repository
pull(
    name = "missing_repo",
    tag = "1.0.0",
)
# Missing name
pull(
    repository = "library/ubuntu",
    tag = "24.04",
)
"#;
        assert!(extract_rules_img_pull_deps(input).is_empty());
    }

    // Ported: "handles rules_img with renamed variable" — bazel-module/extract.spec.ts line 1161
    #[test]
    fn extracts_rules_img_pull_dependency_with_renamed_variable() {
        let input = r#"
my_pull = use_repo_rule("@rules_img//img:pull.bzl", "pull")
my_pull(
    name = "ubuntu",
    repository = "library/ubuntu",
    tag = "24.04",
)
"#;
        let deps = extract_rules_img_pull_deps(input);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "ubuntu");
        assert_eq!(deps[0].package_name, "library/ubuntu");
        assert_eq!(deps[0].current_value.as_deref(), Some("24.04"));
    }

    // Ported: "ignores non-rules_img repo rules" — bazel-module/extract.spec.ts line 1193
    #[test]
    fn ignores_non_rules_img_repo_rules() {
        let input = r#"
bazel_dep(name = "some_rules", version = "0.1.0")

other_rule = use_repo_rule("@some_rules//some:rule.bzl", "other")

other_rule(
    name = "test",
    value = "something",
)
"#;
        let bazel_deps = extract(input);
        assert_eq!(bazel_deps.len(), 1);
        assert_eq!(bazel_deps[0].name, "some_rules");
        assert_eq!(bazel_deps[0].current_value, "0.1.0");
        assert!(extract_rules_img_pull_deps(input).is_empty());
    }

    // Ported: "handles every method available in MODULE.bazel files" — bazel-module/extract.spec.ts line 887
    #[test]
    fn extracts_every_supported_module_bazel_method() {
        let input = r#"
module(
    name = "module_name",
    version = "1.2.3",
    compatibility_level = 0,
    repo_name = "io_bazel_module_name",
    bazel_compatibility = ["<=6.0.0", ">=8.2.0"],
)
bazel_dep(name = "bazel_skylib", version = "1.2.1")
bazel_dep(name = "platforms", version = "0.0.8")
bazel_dep(name = "rules_img", version = "0.1.5")
bazel_dep(name = "stardoc", version = "0.6.2", dev_dependency = True, repo_name = "io_bazel_stardoc")
pull = use_repo_rule("@rules_img//img:pull.bzl", "pull")
pull(
    name = "ubuntu",
    digest = "sha256:1e622c5f9ac0c0144d577702ba5f2cce79fc8e3cf89ec88291739cd4eee3b7b9",
    registry = "index.docker.io",
    repository = "library/ubuntu",
    tag = "24.04",
)
multiple_version_override(
    module_name = "overriden_module_a",
    versions = ["1.2.3", "1.2.4"],
    registry = "https://example.com/custom_registry",
)
git_override(
    module_name = "overriden_module_c",
    commit = "850cb49c8649e463b80ef7984e7c744279746170",
    remote = "https://github.com/example/overriden_module_b.git",
)
archive_override(
    module_name = "overriden_module_d",
    urls = [
        "https://example.com/archive.tar.gz",
    ],
)
include("//:extra.MODULE.bazel")
sh_configure = use_extension("//bzlmod:extensions.bzl", "sh_configure")
use_repo(sh_configure, "local_posix_config", "rules_sh_shim_exe")
override_repo(
    sh_configure,
    com_github_foo_bar = "overriden_module_a",
)
register_execution_platforms(
    "@overriden_module_a//:some_execution_platform",
    dev_dependency = True,
)
register_toolchains(
    "@overriden_module_a//:some_toolchain",
    dev_dependency = True,
)
single_version_override(
    module_name = "overriden_module_c",
    version = "1.2.5",
    registry = "https://example.com/custom_registry",
    patch_cmds = [],
    patch_strip = 8,
)
my_repo_rule = use_repo_rule("@my_repo//:my_repo.bzl", "my_repo_rule")
my_repo_rule(
    name = "my_custom_repo",
    url = "https://example.com/my_custom_repo.tar.gz",
    sha256 = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
)
"#;
        let bazel_deps = extract(input);
        assert_eq!(bazel_deps.len(), 4);
        assert_eq!(bazel_deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(bazel_deps[0].name, "bazel_skylib");
        assert_eq!(bazel_deps[0].current_value, "1.2.1");
        assert_eq!(bazel_deps[1].name, "platforms");
        assert_eq!(bazel_deps[1].current_value, "0.0.8");
        assert_eq!(bazel_deps[2].name, "rules_img");
        assert_eq!(bazel_deps[2].current_value, "0.1.5");
        assert_eq!(bazel_deps[3].name, "stardoc");
        assert_eq!(bazel_deps[3].current_value, "0.6.2");
        assert!(bazel_deps[3].dev_dependency);

        let image_deps = extract_rules_img_pull_deps(input);
        assert_eq!(image_deps.len(), 1);
        assert_eq!(image_deps[0].dep_name, "ubuntu");
        assert_eq!(image_deps[0].package_name, "index.docker.io/library/ubuntu");
        assert_eq!(image_deps[0].current_value.as_deref(), Some("24.04"));
        assert_eq!(
            image_deps[0].current_digest.as_deref(),
            Some("sha256:1e622c5f9ac0c0144d577702ba5f2cce79fc8e3cf89ec88291739cd4eee3b7b9")
        );
        assert_eq!(image_deps[0].registry_urls, vec!["https://index.docker.io"]);
    }

    // Ported: "returns bazel_dep and archive_override dependencies" — bazel-module/extract.spec.ts line 148
    #[test]
    fn extracts_archive_override_with_bazel_dep_version() {
        let content = r#"
bazel_dep(name = "rules_foo", version = "1.2.3")
archive_override(
  module_name = "rules_foo",
  urls = [
    "https://example.com/archive.tar.gz",
  ],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::FileDependency));
        assert_eq!(deps[1].dep_type, BazelModuleDepType::ArchiveOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(
            deps[1].skip_reason,
            Some(BazelSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "returns bazel_dep with no version and archive_override dependencies" — bazel-module/extract.spec.ts line 179
    #[test]
    fn extracts_archive_override_with_unversioned_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_foo")
archive_override(
  module_name = "rules_foo",
  urls = [
    "https://example.com/archive.tar.gz",
  ],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert!(deps[0].current_value.is_empty());
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::FileDependency));
        assert_eq!(deps[1].dep_type, BazelModuleDepType::ArchiveOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(
            deps[1].skip_reason,
            Some(BazelSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "returns bazel_dep and local_path_override dependencies" — bazel-module/extract.spec.ts line 209
    #[test]
    fn extracts_local_path_override_with_bazel_dep_version() {
        let content = r#"
bazel_dep(name = "rules_foo", version = "1.2.3")
local_path_override(
  module_name = "rules_foo",
  urls = "/path/to/repo",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::LocalDependency));
        assert_eq!(deps[1].dep_type, BazelModuleDepType::LocalPathOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(
            deps[1].skip_reason,
            Some(BazelSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "returns bazel_dep with no version and local_path_override dependencies" — bazel-module/extract.spec.ts line 238
    #[test]
    fn extracts_local_path_override_with_unversioned_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_foo")
local_path_override(
  module_name = "rules_foo",
  urls = "/path/to/repo",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert!(deps[0].current_value.is_empty());
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::LocalDependency));
        assert_eq!(deps[1].dep_type, BazelModuleDepType::LocalPathOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(
            deps[1].skip_reason,
            Some(BazelSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "returns bazel_dep and single_version_override dependencies if a version is specified" — bazel-module/extract.spec.ts line 266
    #[test]
    fn extracts_single_version_override_with_bazel_dep_version() {
        let content = r#"
bazel_dep(name = "rules_foo", version = "1.2.3")
single_version_override(
  module_name = "rules_foo",
  version = "1.2.5",
  registry = "https://example.com/custom_registry",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::IsPinned));
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
        assert_eq!(deps[1].dep_type, BazelModuleDepType::SingleVersionOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(deps[1].current_value, "1.2.5");
        assert_eq!(deps[1].skip_reason, Some(BazelSkipReason::Ignored));
        assert_eq!(
            deps[1].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
    }

    // Ported: "returns bazel_dep with no version and single_version_override dependencies if a version is specified" — bazel-module/extract.spec.ts line 299
    #[test]
    fn extracts_single_version_override_with_unversioned_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_foo")
single_version_override(
  module_name = "rules_foo",
  version = "1.2.3",
  registry = "https://example.com/custom_registry",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert!(deps[0].current_value.is_empty());
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::IsPinned));
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
        assert_eq!(deps[1].dep_type, BazelModuleDepType::SingleVersionOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(deps[1].current_value, "1.2.3");
        assert_eq!(deps[1].skip_reason, Some(BazelSkipReason::Ignored));
    }

    // Ported: "returns bazel_dep dependency if single_version_override does not have a version" — bazel-module/extract.spec.ts line 331
    #[test]
    fn single_version_override_without_version_only_adds_registry_to_versioned_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_foo", version = "1.2.3")
single_version_override(
  module_name = "rules_foo",
  registry = "https://example.com/custom_registry",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert!(deps[0].skip_reason.is_none());
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
    }

    // Ported: "returns bazel_dep with no version dependency if single_version_override does not have a version" — bazel-module/extract.spec.ts line 355
    #[test]
    fn single_version_override_without_version_keeps_unversioned_bazel_dep_skipped() {
        let content = r#"
bazel_dep(name = "rules_foo")
single_version_override(
  module_name = "rules_foo",
  registry = "https://example.com/custom_registry",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert!(deps[0].current_value.is_empty());
        assert_eq!(
            deps[0].skip_reason,
            Some(BazelSkipReason::UnspecifiedVersion)
        );
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
    }

    // Ported: "returns null if file is empty" — bazel-module/extract.spec.ts line 41
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null if fails to parse" — bazel-module/extract.spec.ts line 25
    #[test]
    fn malformed_content_returns_empty() {
        assert!(extract("blahhhhh:foo:@what\n").is_empty());
    }

    // Ported: "returns null if something throws an error" — bazel-module/extract.spec.ts line 33
    #[test]
    fn unexpected_parser_input_returns_empty() {
        // TS test mocks parser.parse to throw; Rust handles all errors via Result.
        // Calling extract() with the same input ("content") must return empty, not panic.
        assert!(extract("content").is_empty());
    }

    // Ported: "returns null if file has unrecognized declarations" — bazel-module/extract.spec.ts line 46
    #[test]
    fn comment_lines_stripped() {
        let content = r#"
# This is a comment
bazel_dep(name = "rules_go", version = "0.41.0")  # inline comment
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rules_go");
    }

    // Ported: "returns null if file has unrecognized declarations" — bazel-module/extract.spec.ts line 46
    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("module(name = \"mymodule\", version = \"1.0.0\")\n").is_empty());
    }

    // Ported: "ignores repo rule calls that are not rules_img" — modules/manager/bazel-module/rules-img.spec.ts line 5
    #[test]
    fn rules_img_ignores_non_rules_img() {
        let frags = vec![
            serde_json::json!({
                "type": "useRepoRule",
                "variableName": "other_rule",
                "bzlFile": "@other_rules//some:rule.bzl",
                "ruleName": "other",
                "isComplete": true
            }),
            serde_json::json!({
                "type": "repoRuleCall",
                "functionName": "other_rule",
                "children": {
                    "name": {"type": "string", "value": "test", "isComplete": true},
                    "value": {"type": "string", "value": "something", "isComplete": true}
                },
                "isComplete": true,
                "offset": 0,
                "rawString": "other_rule(name = \"test\", value = \"something\")"
            }),
        ];
        let result = transform_rules_img_calls(&frags);
        assert!(result.is_empty());
    }

    // Ported: "handles valid rules_img pull call" — modules/manager/bazel-module/rules-img.spec.ts line 32
    #[test]
    fn rules_img_handles_valid_pull_call() {
        let frags = vec![
            serde_json::json!({
                "type": "useRepoRule",
                "variableName": "pull",
                "bzlFile": "@rules_img//img:pull.bzl",
                "ruleName": "pull",
                "isComplete": true
            }),
            serde_json::json!({
                "type": "repoRuleCall",
                "functionName": "pull",
                "children": {
                    "name": {"type": "string", "value": "ubuntu", "isComplete": true},
                    "repository": {"type": "string", "value": "library/ubuntu", "isComplete": true},
                    "tag": {"type": "string", "value": "24.04", "isComplete": true}
                },
                "isComplete": true,
                "offset": 0,
                "rawString": "pull(name = \"ubuntu\", repository = \"library/ubuntu\", tag = \"24.04\")"
            }),
        ];
        let result = transform_rules_img_calls(&frags);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].dep_name, "ubuntu");
        assert_eq!(result[0].package_name, "library/ubuntu");
        assert_eq!(result[0].current_value.as_deref(), Some("24.04"));
        assert_eq!(result[0].datasource, "docker");
        assert_eq!(result[0].dep_type, "rules_img_pull");
    }

    // Ported: "skips repo rule calls without corresponding use_repo_rule" — modules/manager/bazel-module/rules-img.spec.ts line 72
    #[test]
    fn rules_img_skips_unknown_function() {
        let frags = vec![serde_json::json!({
            "type": "repoRuleCall",
            "functionName": "unknown_function",
            "children": {
                "name": {"type": "string", "value": "test", "isComplete": true}
            },
            "isComplete": true,
            "offset": 0,
            "rawString": "unknown_function(name = \"test\")"
        })];
        let result = transform_rules_img_calls(&frags);
        assert!(result.is_empty());
    }

    // Ported: "skips malformed repo rule calls" — modules/manager/bazel-module/rules-img.spec.ts line 91
    #[test]
    fn rules_img_skips_malformed_call() {
        let frags = vec![
            serde_json::json!({
                "type": "useRepoRule",
                "variableName": "pull",
                "bzlFile": "@rules_img//img:pull.bzl",
                "ruleName": "pull",
                "isComplete": true
            }),
            serde_json::json!({
                "type": "repoRuleCall",
                "functionName": "pull",
                "children": {
                    "tag": {"type": "string", "value": "24.04", "isComplete": true}
                },
                "isComplete": true,
                "offset": 0,
                "rawString": "pull(tag = \"24.04\")"
            }),
        ];
        let result = transform_rules_img_calls(&frags);
        assert!(result.is_empty());
    }

    // Ported: ".asBoolean($a)" (it.each True/False) — modules/manager/bazel-module/parser/starlark.spec.ts line 4
    #[test]
    fn starlark_boolean_parsing() {
        assert_eq!(starlark_as_boolean("True"), Ok(true));
        assert_eq!(starlark_as_boolean("False"), Ok(false));
    }

    // Ported: "asBoolean" (throws) — modules/manager/bazel-module/parser/starlark.spec.ts line 12
    #[test]
    fn starlark_boolean_invalid_throws() {
        let result = starlark_as_boolean("bad");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Invalid Starlark boolean string: bad")
        );
    }

    #[test]
    fn bazel_ctx_prepare_extension_tag() {
        let mut ctx = BazelCtx::new("");
        ctx.prepare_extension_tag("my_ext", "raw_ext", 0);
        assert_eq!(ctx.stack.len(), 1);
    }

    // ── bazel-module/parser/index.spec.ts ───────────────────────────────────

    // Ported: "returns empty string if invalid content" — bazel-module/parser/index.spec.ts line 7
    #[test]
    fn parse_module_bazel_invalid_content_returns_empty() {
        let input = "// This is invalid\na + 1\n<<<<<<<";
        let res = parse_module_bazel(input);
        assert!(res.is_empty());
    }

    // Ported: "finds simple bazel_dep" — bazel-module/parser/index.spec.ts line 17
    #[test]
    fn parse_module_bazel_finds_simple_bazel_dep() {
        let input = r#"bazel_dep(name = "rules_foo", version = "1.2.3")
bazel_dep(name = "rules_bar", version = "1.0.0", dev_dependency = True)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 2);
        assert!(matches!(&res[0], BazelFragment::Rule { rule, .. } if rule == "bazel_dep"));
        assert!(matches!(&res[1], BazelFragment::Rule { rule, .. } if rule == "bazel_dep"));
    }

    // Ported: "finds the git_override" — bazel-module/parser/index.spec.ts line 44
    #[test]
    fn parse_module_bazel_finds_git_override() {
        let input = r#"bazel_dep(name = "rules_foo", version = "1.2.3")
git_override(
  module_name = "rules_foo",
  remote = "https://github.com/example/rules_foo.git",
  commit = "6a2c2e22849b3e6b33d5ea9aa72222d4803a986a",
)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 2);
        assert!(matches!(&res[0], BazelFragment::Rule { rule, .. } if rule == "bazel_dep"));
        assert!(matches!(&res[1], BazelFragment::Rule { rule, .. } if rule == "git_override"));
    }

    // Ported: "finds archive_override" — bazel-module/parser/index.spec.ts line 85
    #[test]
    fn parse_module_bazel_finds_archive_override() {
        let input = r#"bazel_dep(name = "rules_foo", version = "1.2.3")
archive_override(
  module_name = "rules_foo",
  urls = [
    "https://example.com/archive.tar.gz",
  ],
)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 2);
        assert!(matches!(&res[0], BazelFragment::Rule { rule, .. } if rule == "bazel_dep"));
        assert!(matches!(&res[1], BazelFragment::Rule { rule, .. } if rule == "archive_override"));
    }

    // Ported: "finds local_path_override" — bazel-module/parser/index.spec.ts line 119
    #[test]
    fn parse_module_bazel_finds_local_path_override() {
        let input = r#"bazel_dep(name = "rules_foo", version = "1.2.3")
local_path_override(
  module_name = "rules_foo",
  path = "/path/to/repo",
)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 2);
        assert!(matches!(&res[0], BazelFragment::Rule { rule, .. } if rule == "bazel_dep"));
        assert!(
            matches!(&res[1], BazelFragment::Rule { rule, .. } if rule == "local_path_override")
        );
    }

    // Ported: "finds single_version_override" — bazel-module/parser/index.spec.ts line 148
    #[test]
    fn parse_module_bazel_finds_single_version_override() {
        let input = r#"bazel_dep(name = "rules_foo", version = "1.2.3")
single_version_override(
  module_name = "rules_foo",
  version = "1.2.3",
  registry = "https://example.com/custom_registry",
)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 2);
        assert!(matches!(&res[0], BazelFragment::Rule { rule, .. } if rule == "bazel_dep"));
        assert!(
            matches!(&res[1], BazelFragment::Rule { rule, .. } if rule == "single_version_override")
        );
    }

    // Ported: "finds maven.artifact" — bazel-module/parser/index.spec.ts line 179
    #[test]
    fn parse_module_bazel_finds_maven_artifact() {
        let input = r#"maven.artifact(
    artifact = "core.specs.alpha",
    exclusions = ["org.clojure:clojure"],
    group = "org.clojure",
    version = "0.2.56",
)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 1);
        assert!(
            matches!(&res[0], BazelFragment::ExtensionTag { extension, tag, .. } if extension == "maven" && tag == "artifact")
        );
    }

    // Ported: "finds maven.install and maven.artifact" — bazel-module/parser/index.spec.ts line 248
    #[test]
    fn parse_module_bazel_finds_maven_install_and_artifact() {
        let input = r#"maven.install(
    artifacts = [
        "junit:junit:4.13.2",
        "com.google.guava:guava:31.1-jre",
    ],
    repositories = [
        "https://repo1.maven.org/maven2/"
    ]
)

maven.artifact(
    artifact = "core.specs.alpha",
    group = "org.clojure",
    version = "0.2.56",
)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 2);
        assert!(
            matches!(&res[0], BazelFragment::ExtensionTag { extension, tag, .. } if extension == "maven" && tag == "install")
        );
        assert!(
            matches!(&res[1], BazelFragment::ExtensionTag { extension, tag, .. } if extension == "maven" && tag == "artifact")
        );
    }

    // Ported: "finds oci.pull" — bazel-module/parser/index.spec.ts line 335
    #[test]
    fn parse_module_bazel_finds_oci_pull() {
        let input = r#"oci.pull(
  name = "nginx_image",
  digest = "sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720",
  image = "index.docker.io/library/nginx",
  platforms = ["linux/amd64"],
  tag = "1.27.1",
)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 1);
        assert!(
            matches!(&res[0], BazelFragment::ExtensionTag { extension, tag, .. } if extension == "oci" && tag == "pull")
        );
    }

    // Ported: "finds the git_repository" — bazel-module/parser/index.spec.ts line 376
    #[test]
    fn parse_module_bazel_finds_git_repository() {
        let input = r#"git_repository(
  name = "rules_foo",
  remote = "https://github.com/example/rules_foo.git",
  commit = "6a2c2e22849b3e6b33d5ea9aa72222d4803a986a",
)"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 1);
        assert!(matches!(&res[0], BazelFragment::Rule { rule, .. } if rule == "git_repository"));
    }

    // Ported: "finds use_repo_rule and repo rule call" — bazel-module/parser/index.spec.ts line 408
    #[test]
    fn parse_module_bazel_finds_use_repo_rule_and_repo_rule_call() {
        let input = r#"pull = use_repo_rule("@rules_img//img:pull.bzl", "pull")
pull(name = "nginx", tag = "1.27.1")"#;
        let res = parse_module_bazel(input);
        assert_eq!(res.len(), 2);
        assert!(
            matches!(&res[0], BazelFragment::UseRepoRule { variable_name, .. } if variable_name == "pull")
        );
        assert!(
            matches!(&res[1], BazelFragment::RepoRuleCall { function_name, .. } if function_name == "pull")
        );
    }

    // Ported: "ignores use_repo_rule with insufficient args" — bazel-module/parser/index.spec.ts line 420
    #[test]
    fn parse_module_bazel_ignores_use_repo_rule_with_insufficient_args() {
        let input = r#"pull = use_repo_rule("only_one_arg")"#;
        let res = parse_module_bazel(input);
        assert!(res.is_empty());
    }
}

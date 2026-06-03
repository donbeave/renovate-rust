//! Bazel rule parser — produces Fragment AST with byte offsets.
//!
//! Mirrors `lib/modules/manager/bazel/parser.ts`.
//!
//! Handles:
//! - Plain rules: `rule_name(key = "value", ...)`
//! - Maybe-wrapped: `maybe(rule_name, key = "value", ...)`
//! - String values and arrays of strings

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Fragment types
// ---------------------------------------------------------------------------

/// A parsed fragment of a Bazel rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fragment {
    String(StringFragment),
    Array(ArrayFragment),
    Record(RecordFragment),
}

impl Fragment {
    pub fn value(&self) -> &str {
        match self {
            Fragment::String(f) => &f.value,
            Fragment::Array(f) => &f.value,
            Fragment::Record(f) => &f.value,
        }
    }

    pub fn offset(&self) -> usize {
        match self {
            Fragment::String(f) => f.offset,
            Fragment::Array(f) => f.offset,
            Fragment::Record(f) => f.offset,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringFragment {
    /// Inner string content (without quotes).
    pub value: String,
    /// Byte offset of inner content (after opening quote).
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayFragment {
    /// Source text of the array (including brackets).
    pub value: String,
    /// Byte offset of the opening `[`.
    pub offset: usize,
    pub children: Vec<Fragment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordFragment {
    /// Full source text of the rule call.
    pub value: String,
    /// Byte offset of the first character of the rule.
    pub offset: usize,
    pub children: HashMap<String, Fragment>,
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    fn remaining(&self) -> &'a str {
        &self.src[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            let ws = self
                .remaining()
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();
            if ws > 0 {
                let byte_len: usize = self
                    .remaining()
                    .chars()
                    .take(ws)
                    .map(|c| c.len_utf8())
                    .sum();
                self.pos += byte_len;
                continue;
            }
            // Skip # line comments
            if self.remaining().starts_with('#') {
                let end = self
                    .remaining()
                    .find('\n')
                    .unwrap_or(self.remaining().len());
                self.pos += end;
                continue;
            }
            break;
        }
    }

    fn consume_char(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    /// Try to consume an identifier (letters, digits, underscores).
    fn consume_ident(&mut self) -> Option<String> {
        let s = self.remaining();
        if !s.starts_with(|c: char| c.is_alphabetic() || c == '_') {
            return None;
        }
        let len = s
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '.')
            .map(|c| c.len_utf8())
            .sum();
        let ident = s[..len].to_owned();
        self.pos += len;
        Some(ident)
    }

    /// Parse a quoted string, returning (inner_content, offset_of_inner).
    fn parse_string(&mut self) -> Option<StringFragment> {
        let quote = self.peek()?;
        if quote != '"' && quote != '\'' {
            return None;
        }
        let quote_offset = self.pos;
        self.pos += 1; // consume opening quote
        let inner_offset = self.pos;
        let mut value = String::new();
        loop {
            match self.consume_char()? {
                c if c == quote => break,
                '\\' => {
                    let escaped = self.consume_char()?;
                    value.push('\\');
                    value.push(escaped);
                }
                c => value.push(c),
            }
        }
        let _ = quote_offset;
        Some(StringFragment {
            value,
            offset: inner_offset,
        })
    }

    /// Try to parse a function call like `maven.artifact(...)` as a sub-RecordFragment.
    /// Returns None and does NOT advance pos if it's not a function call.
    fn try_parse_function_call(&mut self) -> Option<RecordFragment> {
        let call_start = self.pos;
        // Parse dotted identifier: `maven.artifact` or `http_archive`
        let s = self.remaining();
        if !s.starts_with(|c: char| c.is_alphabetic() || c == '_') {
            return None;
        }
        let len: usize = s
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '.')
            .map(|c| c.len_utf8())
            .sum();
        let func_name = s[..len].to_owned();
        self.pos += len;
        self.skip_whitespace_and_comments();
        if self.peek() != Some('(') {
            // Not a call – restore position
            self.pos = call_start;
            return None;
        }
        self.pos += 1; // consume '('

        let mut children: HashMap<String, Fragment> = HashMap::new();
        children.insert(
            "_function".to_owned(),
            Fragment::String(StringFragment {
                value: func_name,
                offset: call_start,
            }),
        );

        let mut arg_index: usize = 0;
        loop {
            self.skip_whitespace_and_comments();
            match self.peek() {
                Some(')') => {
                    self.pos += 1;
                    break;
                }
                None => break,
                Some('"') | Some('\'') => {
                    // Positional string arg
                    if let Some(s) = self.parse_string() {
                        children.insert(arg_index.to_string(), Fragment::String(s));
                        arg_index += 1;
                    }
                    self.skip_whitespace_and_comments();
                    if self.peek() == Some(',') {
                        self.pos += 1;
                    }
                }
                _ => {
                    // Try keyword arg: ident = "..."
                    let saved = self.pos;
                    if let Some(key) = self.consume_ident() {
                        self.skip_whitespace_and_comments();
                        if self.peek() == Some('=') {
                            self.pos += 1;
                            self.skip_whitespace_and_comments();
                            if (self.peek() == Some('"') || self.peek() == Some('\''))
                                && let Some(s) = self.parse_string()
                            {
                                children.insert(key, Fragment::String(s));
                                self.skip_whitespace_and_comments();
                                if self.peek() == Some(',') {
                                    self.pos += 1;
                                }
                                continue;
                            }
                        }
                    }
                    // Not a parseable kwarg — skip to next comma or ')'
                    self.pos = saved;
                    let mut depth = 0i32;
                    loop {
                        match self.peek() {
                            None => break,
                            Some('(') | Some('[') => {
                                depth += 1;
                                self.pos += 1;
                            }
                            Some(')') | Some(']') => {
                                if depth == 0 {
                                    break;
                                }
                                depth -= 1;
                                self.pos += 1;
                            }
                            Some(',') if depth == 0 => {
                                self.pos += 1;
                                break;
                            }
                            Some(c) => {
                                self.pos += c.len_utf8();
                            }
                        }
                    }
                }
            }
        }

        let value = self.src[call_start..self.pos].to_owned();
        Some(RecordFragment {
            value,
            offset: call_start,
            children,
        })
    }

    /// Parse a `[...]` array of strings and/or function calls.
    fn parse_array(&mut self) -> Option<ArrayFragment> {
        if self.peek() != Some('[') {
            return None;
        }
        let start = self.pos;
        self.pos += 1; // consume `[`
        let mut children = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            match self.peek() {
                Some(']') => {
                    self.pos += 1;
                    break;
                }
                Some('"') | Some('\'') => {
                    if let Some(s) = self.parse_string() {
                        children.push(Fragment::String(s));
                    }
                    self.skip_whitespace_and_comments();
                    if self.peek() == Some(',') {
                        self.pos += 1;
                    }
                }
                Some(c) if c.is_alphabetic() || c == '_' => {
                    // Try to parse as function call
                    if let Some(record) = self.try_parse_function_call() {
                        children.push(Fragment::Record(record));
                        self.skip_whitespace_and_comments();
                        if self.peek() == Some(',') {
                            self.pos += 1;
                        }
                    } else {
                        // Skip non-parseable token
                        let mut depth = 0i32;
                        loop {
                            match self.peek() {
                                None => break,
                                Some('[') | Some('(') => {
                                    depth += 1;
                                    self.pos += 1;
                                }
                                Some(']') | Some(')') => {
                                    if depth == 0 {
                                        break;
                                    }
                                    depth -= 1;
                                    self.pos += 1;
                                }
                                Some(',') if depth == 0 => {
                                    self.pos += 1;
                                    break;
                                }
                                Some(c) => {
                                    self.pos += c.len_utf8();
                                }
                            }
                        }
                    }
                }
                _ => {
                    // Skip non-string non-call elements
                    let mut depth = 0i32;
                    loop {
                        match self.peek() {
                            None => break,
                            Some('[') | Some('(') => {
                                depth += 1;
                                self.pos += 1;
                            }
                            Some(']') | Some(')') => {
                                if depth == 0 {
                                    break;
                                }
                                depth -= 1;
                                self.pos += 1;
                            }
                            Some(',') if depth == 0 => {
                                self.pos += 1;
                                break;
                            }
                            Some(c) => {
                                self.pos += c.len_utf8();
                            }
                        }
                    }
                }
            }
        }
        let value = self.src[start..self.pos].to_owned();
        Some(ArrayFragment {
            value,
            offset: start,
            children,
        })
    }

    /// Parse key = value pair, returning (key, Fragment).
    fn parse_kwarg(&mut self) -> Option<(String, Fragment)> {
        self.skip_whitespace_and_comments();
        let key = self.consume_ident()?;
        self.skip_whitespace_and_comments();
        if self.peek() != Some('=') {
            return None;
        }
        self.pos += 1; // consume '='
        self.skip_whitespace_and_comments();
        let frag = if self.peek() == Some('[') {
            Fragment::Array(self.parse_array()?)
        } else if self.peek() == Some('"') || self.peek() == Some('\'') {
            Fragment::String(self.parse_string()?)
        } else {
            // Skip non-string, non-array values
            let start = self.pos;
            let mut depth = 0i32;
            loop {
                match self.peek() {
                    None => break,
                    Some(',') | Some(')') if depth == 0 => break,
                    Some('(') | Some('[') => {
                        depth += 1;
                        self.pos += 1;
                    }
                    Some(')') | Some(']') => {
                        depth -= 1;
                        self.pos += 1;
                    }
                    Some(c) => {
                        self.pos += c.len_utf8();
                    }
                }
            }
            Fragment::String(StringFragment {
                value: self.src[start..self.pos].trim().to_owned(),
                offset: start,
            })
        };
        Some((key, frag))
    }

    /// Parse a Bazel rule call (or maybe() wrapper).
    fn parse_rule(&mut self) -> Option<RecordFragment> {
        self.skip_whitespace_and_comments();
        let rule_start = self.pos;
        let func_name = self.consume_ident()?;
        self.skip_whitespace_and_comments();
        if self.peek() != Some('(') {
            return None;
        }
        self.pos += 1; // consume '('

        let mut children: HashMap<String, Fragment> = HashMap::new();
        let actual_rule_name;

        if func_name == "maybe" {
            // maybe(actual_rule, key = val, ...)
            self.skip_whitespace_and_comments();
            let inner_rule_start = self.pos;
            let rule = self.consume_ident()?;
            let inner_rule_end = self.pos;
            children.insert(
                "rule".to_owned(),
                Fragment::String(StringFragment {
                    value: rule,
                    offset: inner_rule_start,
                }),
            );
            // Check if there's an offset delta - the rule name value might include prefix
            let _ = inner_rule_end;
            let _ = rule;
            // Skip comma after rule name
            self.skip_whitespace_and_comments();
            if self.peek() == Some(',') {
                self.pos += 1;
            }
        } else {
            // Normal rule: record the rule name as a child
            actual_rule_name = func_name;
            children.insert(
                "rule".to_owned(),
                Fragment::String(StringFragment {
                    value: actual_rule_name,
                    offset: rule_start,
                }),
            );
        }

        // Parse keyword arguments
        loop {
            self.skip_whitespace_and_comments();
            if self.peek() == Some(')') {
                self.pos += 1;
                break;
            }
            if self.peek().is_none() {
                break;
            }
            if let Some((key, frag)) = self.parse_kwarg() {
                children.insert(key, frag);
            }
            self.skip_whitespace_and_comments();
            if self.peek() == Some(',') {
                self.pos += 1;
            }
        }

        let value = self.src[rule_start..self.pos].to_owned();
        Some(RecordFragment {
            value,
            offset: rule_start,
            children,
        })
    }

    /// Parse all rules from source.
    fn parse_all(&mut self) -> Vec<RecordFragment> {
        let mut results = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            if self.pos >= self.src.len() {
                break;
            }
            if let Some(rule) = self.parse_rule() {
                results.push(rule);
            } else {
                // Skip one char and try again
                if let Some(c) = self.peek() {
                    self.pos += c.len_utf8();
                }
            }
        }
        results
    }
}

/// Parse a Bazel source string into an array of record fragments.
///
/// Mirrors `parse()` from `lib/modules/manager/bazel/parser.ts`.
pub fn parse(input: &str) -> Option<Vec<RecordFragment>> {
    let mut parser = Parser::new(input);
    let results = parser.parse_all();
    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

// ---------------------------------------------------------------------------
// updateCode — lib/modules/manager/bazel/common.ts
// ---------------------------------------------------------------------------

/// Navigation path into a parsed rule tree.
#[derive(Debug, Clone)]
pub enum PathKey {
    Index(usize),
    Key(String),
}

/// Update the value at a path in a parsed Bazel source string.
///
/// Mirrors `updateCode()` from `lib/modules/manager/bazel/common.ts`.
pub fn update_code(input: &str, path: &[PathKey], replacement: &str) -> String {
    let Some(rules) = parse(input) else {
        return input.to_owned();
    };

    let Some(PathKey::Index(rule_idx)) = path.first() else {
        return input.to_owned();
    };
    let rule_idx = *rule_idx;

    let Some(rule) = rules.get(rule_idx) else {
        return input.to_owned();
    };

    if path.len() == 1 {
        // Replace entire rule
        let left = &input[..rule.offset];
        let right = &input[rule.offset + rule.value.len()..];
        return format!("{}{}{}", left, replacement, right);
    }

    // Navigate into the rule
    let Some(PathKey::Key(key)) = path.get(1) else {
        return input.to_owned();
    };

    let Some(child) = rule.children.get(key.as_str()) else {
        return input.to_owned();
    };

    if path.len() == 2 {
        // Replace a string or array value
        let (offset, len) = match child {
            Fragment::String(s) => (s.offset, s.value.len()),
            Fragment::Array(a) => (a.offset, a.value.len()),
            Fragment::Record(r) => (r.offset, r.value.len()),
        };
        let left = &input[..offset];
        let right = &input[offset + len..];
        return format!("{}{}{}", left, replacement, right);
    }

    // Navigate into array
    let Some(PathKey::Index(arr_idx)) = path.get(2) else {
        return input.to_owned();
    };
    let arr_idx = *arr_idx;

    match child {
        Fragment::Array(arr) => {
            let Some(elem) = arr.children.get(arr_idx) else {
                return input.to_owned();
            };
            let (offset, len) = match elem {
                Fragment::String(s) => (s.offset, s.value.len()),
                _ => return input.to_owned(),
            };
            let left = &input[..offset];
            let right = &input[offset + len..];
            format!("{}{}{}", left, replacement, right)
        }
        _ => input.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns input for invalid" — lib/modules/manager/bazel/common.spec.ts line 5
    #[test]
    fn update_code_returns_input_for_invalid() {
        let input = "!@#";
        assert_eq!(update_code(input, &[PathKey::Index(0)], "foobar"), input);
    }

    // Ported: "replaces whole rule" — lib/modules/manager/bazel/common.spec.ts line 11
    #[test]
    fn update_code_replaces_whole_rule() {
        let input = r#"git_repository(name = "foo")"#;
        let output = update_code(input, &[PathKey::Index(0)], "abcde");
        assert_eq!(output, "abcde");
    }

    // Ported: "replaces rule key" — lib/modules/manager/bazel/common.spec.ts line 17
    #[test]
    fn update_code_replaces_rule_key() {
        let input = r#"git_repository(name = "foo")"#;
        let output = update_code(
            input,
            &[PathKey::Index(0), PathKey::Key("name".to_owned())],
            "bar",
        );
        assert_eq!(output, r#"git_repository(name = "bar")"#);
    }

    // Ported: "returns input on wrong index" — lib/modules/manager/bazel/common.spec.ts line 23
    #[test]
    fn update_code_wrong_index() {
        let input = r#"git_repository(name = "foo")"#;
        let output = update_code(
            input,
            &[PathKey::Index(1), PathKey::Key("name".to_owned())],
            "bar",
        );
        assert_eq!(output, input);
    }

    // Ported: "returns input on wrong key" — lib/modules/manager/bazel/common.spec.ts line 29
    #[test]
    fn update_code_wrong_key() {
        let input = r#"git_repository(name = "foo")"#;
        let output = update_code(
            input,
            &[PathKey::Index(0), PathKey::Key("foobar".to_owned())],
            "bar",
        );
        assert_eq!(output, input);
    }

    // Ported: "replaces array values" — lib/modules/manager/bazel/common.spec.ts line 35
    #[test]
    fn update_code_replaces_array_value() {
        let input = r#"git_repository(name = "foo", deps = ["bar", "baz", "qux"])"#;
        let output = update_code(
            input,
            &[
                PathKey::Index(0),
                PathKey::Key("deps".to_owned()),
                PathKey::Index(1),
            ],
            "BAZ",
        );
        assert_eq!(
            output,
            r#"git_repository(name = "foo", deps = ["bar", "BAZ", "qux"])"#
        );
    }

    // Ported: "updates using function" — lib/modules/manager/bazel/common.spec.ts line 43
    // Note: Rust doesn't support function values the same way; tested as uppercase replacement
    #[test]
    fn update_code_updater_function_equivalent() {
        let input = r#"git_repository(name = "foo")"#;
        // Simulate: (x) => x.toUpperCase() for "foo" → "FOO"
        let rules = parse(input).unwrap();
        let frag = &rules[0].children["name"];
        let current = frag.value();
        let replacement = current.to_uppercase();
        let output = update_code(
            input,
            &[PathKey::Index(0), PathKey::Key("name".to_owned())],
            &replacement,
        );
        assert_eq!(output, r#"git_repository(name = "FOO")"#);
    }
}

// Ported: "parses rules input" — lib/modules/manager/bazel/parser.spec.ts line 6
#[test]
fn parse_rules_input_basic() {
    let input = "go_repository(name = \"foo\")\nmaybe(go_repository, name = \"bar\", deps = [\"baz\", \"qux\"])\n_go_repository(name = \"quux\")\nmaybe(_go_repository, name = \"corge\", deps = [\"grault\", \"garply\"])\n";
    let res = parse(input).unwrap();
    assert_eq!(res.len(), 4);

    // Rule 1: go_repository
    assert_eq!(res[0].offset, 0);
    assert_eq!(res[0].value, "go_repository(name = \"foo\")");
    assert_eq!(res[0].children["name"].value(), "foo");
    assert_eq!(res[0].children["name"].offset(), 22);

    // Rule 2: maybe(go_repository, ...)
    assert_eq!(res[1].offset, 28);
    assert_eq!(res[1].children["rule"].value(), "go_repository");
    assert_eq!(res[1].children["rule"].offset(), 34);
    assert_eq!(res[1].children["name"].value(), "bar");
    assert_eq!(res[1].children["name"].offset(), 57);
    // deps array
    let deps = &res[1].children["deps"];
    match deps {
        Fragment::Array(a) => {
            assert_eq!(a.offset, 70);
            assert_eq!(a.children[0].value(), "baz");
            assert_eq!(a.children[0].offset(), 72);
            assert_eq!(a.children[1].value(), "qux");
            assert_eq!(a.children[1].offset(), 79);
        }
        _ => panic!("expected array"),
    }

    // Rule 3: _go_repository
    assert_eq!(res[2].children["name"].value(), "quux");

    // Rule 4: maybe(_go_repository, ...)
    assert_eq!(res[3].children["rule"].value(), "_go_repository");
}

// Ported: "parses multiple archives" — lib/modules/manager/bazel/parser.spec.ts line 81
#[test]
fn parse_multiple_archives() {
    let input = concat!(
        "http_archive(\n",
        "    name = \"aspect_rules_js\",\n",
        "    sha256 = \"db9f446752fe4100320cf8487e8fd476b9af0adf6b99b601bcfd70b289bb0598\",\n",
        "    strip_prefix = \"rules_js-1.1.2\",\n",
        "    url = \"https://github.com/aspect-build/rules_js/archive/refs/tags/v1.1.2.tar.gz\",\n",
        ")\n",
        "http_archive(\n",
        "  name = \"rules_nodejs\",\n",
        "  sha256 = \"5aef09ed3279aa01d5c928e3beb248f9ad32dde6aafe6373a8c994c3ce643064\",\n",
        "  urls = [\"https://github.com/bazelbuild/rules_nodejs/releases/download/5.5.3/rules_nodejs-core-5.5.3.tar.gz\"],\n",
        ")\n",
    );
    let res = parse(input).unwrap();
    assert_eq!(res.len(), 2);

    assert_eq!(res[0].children["rule"].value(), "http_archive");
    assert_eq!(res[0].children["name"].value(), "aspect_rules_js");
    assert_eq!(
        res[0].children["sha256"].value(),
        "db9f446752fe4100320cf8487e8fd476b9af0adf6b99b601bcfd70b289bb0598"
    );
    assert_eq!(res[0].children["strip_prefix"].value(), "rules_js-1.1.2");
    assert_eq!(
        res[0].children["url"].value(),
        "https://github.com/aspect-build/rules_js/archive/refs/tags/v1.1.2.tar.gz"
    );

    assert_eq!(res[1].children["rule"].value(), "http_archive");
    assert_eq!(res[1].children["name"].value(), "rules_nodejs");
    assert_eq!(
        res[1].children["sha256"].value(),
        "5aef09ed3279aa01d5c928e3beb248f9ad32dde6aafe6373a8c994c3ce643064"
    );
    match &res[1].children["urls"] {
        Fragment::Array(a) => {
            assert_eq!(a.children.len(), 1);
            assert_eq!(
                a.children[0].value(),
                "https://github.com/bazelbuild/rules_nodejs/releases/download/5.5.3/rules_nodejs-core-5.5.3.tar.gz"
            );
        }
        _ => panic!("expected array for urls"),
    }
}

// Ported: "parses http_archive" — lib/modules/manager/bazel/parser.spec.ts line 156
#[test]
fn parse_http_archive() {
    let input = concat!(
        "http_archive(\n",
        "  name = \"rules_nodejs\",\n",
        "  sha256 = \"5aef09ed3279aa01d5c928e3beb248f9ad32dde6aafe6373a8c994c3ce643064\",\n",
        "  url = \"https://github.com/bazelbuild/rules_nodejs/releases/download/5.5.3/rules_nodejs-core-5.5.3.tar.gz\",\n",
        ")\n",
    );
    let res = parse(input).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].children["rule"].value(), "http_archive");
    assert_eq!(res[0].children["name"].value(), "rules_nodejs");
    assert_eq!(
        res[0].children["sha256"].value(),
        "5aef09ed3279aa01d5c928e3beb248f9ad32dde6aafe6373a8c994c3ce643064"
    );
    assert_eq!(
        res[0].children["url"].value(),
        "https://github.com/bazelbuild/rules_nodejs/releases/download/5.5.3/rules_nodejs-core-5.5.3.tar.gz"
    );
}

// Ported: "parses http_archive with prefixes and multiple urls" — lib/modules/manager/bazel/parser.spec.ts line 195
#[test]
fn parse_http_archive_multiple_urls() {
    let input = concat!(
        "http_archive(\n",
        "  name = \"bazel_toolchains\",\n",
        "  sha256 = \"4b1468b254a572dbe134cc1fd7c6eab1618a72acd339749ea343bd8f55c3b7eb\",\n",
        "  strip_prefix = \"bazel-toolchains-d665ccfa3e9c90fa789671bf4ef5f7c19c5715c4\",\n",
        "  urls = [\n",
        "      \"https://mirror.bazel.build/github.com/bazelbuild/bazel-toolchains/archive/d665ccfa3e9c90fa789671bf4ef5f7c19c5715c4.tar.gz\",\n",
        "      \"https://github.com/bazelbuild/bazel-toolchains/archive/d665ccfa3e9c90fa789671bf4ef5f7c19c5715c4.tar.gz\",\n",
        "  ],\n",
        ")\n",
    );
    let res = parse(input).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].children["rule"].value(), "http_archive");
    assert_eq!(res[0].children["name"].value(), "bazel_toolchains");
    assert_eq!(
        res[0].children["sha256"].value(),
        "4b1468b254a572dbe134cc1fd7c6eab1618a72acd339749ea343bd8f55c3b7eb"
    );
    assert_eq!(
        res[0].children["strip_prefix"].value(),
        "bazel-toolchains-d665ccfa3e9c90fa789671bf4ef5f7c19c5715c4"
    );
    match &res[0].children["urls"] {
        Fragment::Array(a) => {
            assert_eq!(a.children.len(), 2);
            assert_eq!(
                a.children[0].value(),
                "https://mirror.bazel.build/github.com/bazelbuild/bazel-toolchains/archive/d665ccfa3e9c90fa789671bf4ef5f7c19c5715c4.tar.gz"
            );
            assert_eq!(
                a.children[1].value(),
                "https://github.com/bazelbuild/bazel-toolchains/archive/d665ccfa3e9c90fa789671bf4ef5f7c19c5715c4.tar.gz"
            );
        }
        _ => panic!("expected array for urls"),
    }
}

// Ported: "parses Maven" — lib/modules/manager/bazel/parser.spec.ts line 254
#[test]
fn parse_maven() {
    let input = concat!(
        "maven_install(\n",
        "  artifacts = [\n",
        "    \"com.example1:foo:1.1.1\",\n",
        "    maven.artifact(\n",
        "      group = \"com.example2\",\n",
        "      artifact = \"bar\",\n",
        "      version = \"2.2.2\",\n",
        "    ),\n",
        "    maven.artifact(\n",
        "      \"com.example3\",\n",
        "      \"baz\",\n",
        "      \"3.3.3\",\n",
        "      neverlink = True\n",
        "    )\n",
        "  ],\n",
        "  repositories = [\n",
        "    \"https://example1.com/maven2\",\n",
        "    \"https://example2.com/maven2\",\n",
        "  ]\n",
        ")\n",
    );
    let res = parse(input).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].children["rule"].value(), "maven_install");

    match &res[0].children["artifacts"] {
        Fragment::Array(arr) => {
            assert_eq!(arr.children.len(), 3);
            // First: plain string
            assert_eq!(arr.children[0].value(), "com.example1:foo:1.1.1");
            // Second: maven.artifact kwargs
            match &arr.children[1] {
                Fragment::Record(r) => {
                    assert_eq!(r.children["_function"].value(), "maven.artifact");
                    assert_eq!(r.children["group"].value(), "com.example2");
                    assert_eq!(r.children["artifact"].value(), "bar");
                    assert_eq!(r.children["version"].value(), "2.2.2");
                }
                _ => panic!("expected record for maven.artifact kwargs"),
            }
            // Third: maven.artifact positional
            match &arr.children[2] {
                Fragment::Record(r) => {
                    assert_eq!(r.children["_function"].value(), "maven.artifact");
                    assert_eq!(r.children["0"].value(), "com.example3");
                    assert_eq!(r.children["1"].value(), "baz");
                    assert_eq!(r.children["2"].value(), "3.3.3");
                }
                _ => panic!("expected record for maven.artifact positional"),
            }
        }
        _ => panic!("expected array for artifacts"),
    }

    match &res[0].children["repositories"] {
        Fragment::Array(arr) => {
            assert_eq!(arr.children.len(), 2);
            assert_eq!(arr.children[0].value(), "https://example1.com/maven2");
            assert_eq!(arr.children[1].value(), "https://example2.com/maven2");
        }
        _ => panic!("expected array for repositories"),
    }
}

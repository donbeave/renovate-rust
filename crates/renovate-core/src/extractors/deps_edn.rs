//! Clojure `deps.edn` dependency extractor.
//!
//! Parses the `:mvn/version` map entries from Clojure Tools Deps (`deps.edn`)
//! and Babashka (`bb.edn`) files.
//!
//! Renovate reference:
//! - `lib/modules/manager/deps-edn/extract.ts`
//! - Pattern: `/(^|/)(?:deps|bb)\.edn$/`
//! - Datasource: Clojure (Maven Central + Clojars)
//!
//! ## Supported forms
//!
//! ```edn
//! {:deps {org.clojure/clojure {:mvn/version "1.11.1"}
//!         ring {:mvn/version "1.9.6"}}}
//! ```
//!
//! ## Skip reasons
//!
//! - `:git/url` deps — uses git rev, not a Maven version
//! - `:local/root` deps — local path
//! - `:deps/root` deps — local root

use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

/// A single extracted deps.edn Maven dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepsEdnDep {
    /// Maven `group:artifact` coordinates (e.g. `"org.clojure:clojure"`).
    pub dep_name: String,
    /// Version string (e.g. `"1.11.1"`).
    pub current_value: String,
}

// ── Compiled regex ────────────────────────────────────────────────────────────

/// Matches `:mvn/version "x.y.z"`.
static MVN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#":mvn/version\s+"([^"]+)""#).unwrap());

/// Expand `group/artifact` or bare `group` to `group:artifact`.
fn expand_name(s: &str) -> String {
    if let Some((g, a)) = s.split_once('/') {
        format!("{g}:{a}")
    } else {
        format!("{s}:{s}")
    }
}

/// Extract Maven deps from a `deps.edn` or `bb.edn` file.
///
/// Strategy: scan line by line. For each line that contains `:mvn/version`,
/// look backwards (same line or previous tracked line) for the most recent
/// EDN dep symbol followed by ` {`.
pub fn extract(content: &str) -> Vec<DepsEdnDep> {
    let mut out = Vec::new();
    let mut pending: Option<String> = None; // dep name waiting for version

    for raw in content.lines() {
        // Strip line comments.
        let line = {
            let mut in_str = false;
            let mut end = raw.len();
            let mut prev = ' ';
            for (i, ch) in raw.char_indices() {
                if ch == '"' && prev != '\\' {
                    in_str = !in_str;
                }
                if ch == ';' && !in_str {
                    end = i;
                    break;
                }
                prev = ch;
            }
            &raw[..end]
        };

        // Skip git/local deps — they may span multiple lines so reset pending.
        if line.contains(":git/") || line.contains(":local/") || line.contains(":deps/root") {
            pending = None;
            continue;
        }

        let has_mvn = MVN.is_match(line);

        // Find the last `dep-symbol {` pattern on this line.
        // We look for the rightmost `sym {` where `sym` is a valid dep symbol
        // (starts with a letter, contains alphanumeric/dot/hyphen/slash).
        let dep_on_line = find_last_dep_sym(line);

        if has_mvn {
            let version = MVN.captures(line).map(|c| c[1].to_owned());
            // Use dep on this same line if present; otherwise use pending from previous line.
            let name = dep_on_line.or_else(|| pending.take());
            if let (Some(n), Some(v)) = (name, version) {
                out.push(DepsEdnDep {
                    dep_name: expand_name(&n),
                    current_value: v,
                });
            }
            pending = None;
        } else if let Some(sym) = dep_on_line {
            // This line has a dep symbol but no :mvn/version — version is on next line.
            pending = Some(sym);
        } else {
            // No dep symbol on this line — keep pending only if it's from previous line
            // (don't clear it mid-block).
        }
    }
    out
}

/// Find the last `dep-symbol {` occurrence on a line, returning the symbol.
///
/// Returns `None` if no valid dep-symbol-space-brace pattern is found.
/// Filters out EDN keywords (`:deps`, `:aliases`, etc.).
fn find_last_dep_sym(line: &str) -> Option<String> {
    let mut last: Option<String> = None;
    // Scan for every symbol followed by whitespace and `{`.
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        // Start of a potential symbol: letter.
        if ch.is_ascii_alphabetic() {
            let start = i;
            // Consume the symbol: letters, digits, `.`, `-`, `_`, `/`.
            while i < chars.len()
                && (chars[i].is_alphanumeric()
                    || chars[i] == '.'
                    || chars[i] == '-'
                    || chars[i] == '_'
                    || chars[i] == '/')
            {
                i += 1;
            }
            let sym: String = chars[start..i].iter().collect();
            // Skip if the symbol is a keyword word (like `mvn`, `git`, `local`).
            // We allow anything that is a valid Clojure dep symbol.
            // Next must be whitespace then `{`.
            let rest: String = chars[i..].iter().collect();
            if rest.trim_start().starts_with('{') {
                last = Some(sym);
            }
        } else {
            i += 1;
        }
    }
    last
}

// ── EDN parser (mirrors lib/modules/manager/deps-edn/parser.ts) ──────────────

/// Parsed result of a deps.edn file.
#[derive(Debug)]
pub struct ParsedEdnResult {
    /// The parsed EDN data as a JSON-like value (always an Object at root).
    pub data: Value,
    /// Metadata: each entry is (parsed_object_or_array, replace_string).
    pub metadata: Vec<(Value, String)>,
}

impl ParsedEdnResult {
    /// Look up the replace string for a given value (by equality).
    pub fn get_metadata(&self, value: &Value) -> Option<&str> {
        self.metadata
            .iter()
            .find(|(v, _)| v == value)
            .map(|(_, s)| s.as_str())
    }
}

/// EDN token used by the parser state machine.
#[derive(Debug)]
enum EdnToken {
    LeftBrace(usize),   // { with byte offset
    RightBrace(usize),  // } with byte offset + len
    LeftSquare(usize),
    RightSquare(usize),
    LeftParen(usize),
    RightParen(usize),
    Atom(String),       // keyword/symbol/number/string content
}

fn tokenize_edn(input: &str) -> Vec<EdnToken> {
    let bytes = input.as_bytes();
    let n = bytes.len();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < n {
        match bytes[i] {
            b' ' | b'\t' | b'\r' | b'\n' | b',' => i += 1,
            b';' => {
                while i < n && bytes[i] != b'\n' { i += 1; }
            }
            b'{' => { tokens.push(EdnToken::LeftBrace(i)); i += 1; }
            b'}' => { tokens.push(EdnToken::RightBrace(i)); i += 1; }
            b'[' => { tokens.push(EdnToken::LeftSquare(i)); i += 1; }
            b']' => { tokens.push(EdnToken::RightSquare(i)); i += 1; }
            b'(' => { tokens.push(EdnToken::LeftParen(i)); i += 1; }
            b')' => { tokens.push(EdnToken::RightParen(i)); i += 1; }
            b'"' => {
                // Triple-quote string
                if input[i..].starts_with("\"\"\"") {
                    i += 3;
                    let start = i;
                    while i + 2 < n && &input[i..i+3] != "\"\"\"" { i += 1; }
                    let content = input[start..i].to_owned();
                    if i + 2 < n { i += 3; }
                    tokens.push(EdnToken::Atom(content));
                } else {
                    i += 1; // skip opening quote
                    let mut content = String::new();
                    while i < n && bytes[i] != b'"' {
                        if bytes[i] == b'\\' && i + 1 < n {
                            content.push(bytes[i + 1] as char);
                            i += 2;
                        } else {
                            content.push(bytes[i] as char);
                            i += 1;
                        }
                    }
                    if i < n { i += 1; } // skip closing quote
                    tokens.push(EdnToken::Atom(content));
                }
            }
            b':' => {
                // Keyword: strip leading ':'
                i += 1;
                let start = i;
                while i < n && is_edn_sym_char(bytes[i]) { i += 1; }
                tokens.push(EdnToken::Atom(input[start..i].to_owned()));
            }
            c if c.is_ascii_alphabetic() || c == b'_' || c == b'+' || c == b'!' || c == b'\'' || c == b'?' || c == b'<' || c == b'>' || c == b'=' || c == b'.' || c == b'-' || c == b'*' => {
                let start = i;
                while i < n && is_edn_sym_char(bytes[i]) { i += 1; }
                tokens.push(EdnToken::Atom(input[start..i].to_owned()));
            }
            c if c.is_ascii_digit() => {
                let start = i;
                // consume number including e/E+- for exponent, . for float
                while i < n && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.' || bytes[i] == b'+' || bytes[i] == b'-') {
                    // Don't let '-' start a new token unless it's part of exponent
                    if bytes[i] == b'+' || bytes[i] == b'-' {
                        if i > start && (bytes[i-1] == b'e' || bytes[i-1] == b'E') {
                            i += 1;
                        } else {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                tokens.push(EdnToken::Atom(input[start..i].to_owned()));
            }
            _ => i += 1,
        }
    }
    tokens
}

fn is_edn_sym_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_' | b'/' | b'*' | b'+' | b'!' | b'\'' | b'?' | b'<' | b'>' | b'=')
}

/// Parse deps.edn content into a structured value.
/// Returns `None` if the root is not a map.
pub fn parse_deps_edn_file(content: &str) -> Option<ParsedEdnResult> {
    let tokens = tokenize_edn(content);
    let mut metadata: Vec<(Value, String)> = Vec::new();

    enum State {
        Root { data: Option<Value> },
        Record { start: usize, data: serde_json::Map<String, Value>, current_key: Option<String>, skip_key: bool },
        Array  { start: usize, data: Vec<Value> },
    }

    let mut stack: Vec<State> = Vec::new();
    let mut state = State::Root { data: None };

    // Helper: pop state and integrate child value
    let pop_with = |stack: &mut Vec<State>, state: &mut State, child: Value, start: usize, end: usize, content: &str, metadata: &mut Vec<(Value, String)>| {
        // Record metadata for container values
        if child.is_object() || child.is_array() {
            let replace_string = content[start..end].to_owned();
            metadata.push((child.clone(), replace_string));
        }
        let parent = stack.pop().unwrap();
        match parent {
            State::Root { .. } => { *state = State::Root { data: Some(child) }; }
            State::Record { start: ps, data: mut pdata, current_key, skip_key } => {
                if skip_key {
                    *state = State::Record { start: ps, data: pdata, current_key: None, skip_key: false };
                } else if let Some(key) = current_key {
                    pdata.insert(key, child);
                    *state = State::Record { start: ps, data: pdata, current_key: None, skip_key: false };
                } else {
                    // child is the key slot but it's not an atom key - skip next value
                    *state = State::Record { start: ps, data: pdata, current_key: None, skip_key: true };
                }
            }
            State::Array { start: ps, mut data } => {
                data.push(child);
                *state = State::Array { start: ps, data };
            }
        }
    };

    let mut tok_idx = 0;
    while tok_idx < tokens.len() {
        let token = &tokens[tok_idx];
        tok_idx += 1;

        match token {
            EdnToken::LeftBrace(offset) => {
                let old = std::mem::replace(&mut state, State::Record { start: *offset, data: serde_json::Map::new(), current_key: None, skip_key: false });
                stack.push(old);
            }
            EdnToken::LeftSquare(offset) | EdnToken::LeftParen(offset) => {
                let old = std::mem::replace(&mut state, State::Array { start: *offset, data: Vec::new() });
                stack.push(old);
            }
            EdnToken::RightBrace(offset) | EdnToken::RightSquare(offset) | EdnToken::RightParen(offset) => {
                let end = offset + 1;
                let (child_val, start) = match &state {
                    State::Record { data, start, .. } => (Value::Object(data.clone()), *start),
                    State::Array  { data, start, .. } => (Value::Array(data.clone()), *start),
                    State::Root { data } => {
                        // Mismatched closer - ignore
                        let _ = data;
                        continue;
                    }
                };
                if stack.is_empty() { break; }
                pop_with(&mut stack, &mut state, child_val, start, end, content, &mut metadata);
            }
            EdnToken::Atom(val) => {
                match &mut state {
                    State::Root { data } => { *data = Some(Value::String(val.clone())); }
                    State::Array { data, .. } => { data.push(Value::String(val.clone())); }
                    State::Record { data, current_key, skip_key, .. } => {
                        if *skip_key {
                            *current_key = None;
                            *skip_key = false;
                        } else if let Some(key) = current_key.take() {
                            data.insert(key, Value::String(val.clone()));
                        } else {
                            *current_key = Some(val.clone());
                        }
                    }
                }
            }
        }
    }

    // Flush unclosed containers
    while !stack.is_empty() {
        let (child_val, start, end) = match &state {
            State::Record { data, start, .. } => (Value::Object(data.clone()), *start, content.len()),
            State::Array  { data, start, .. } => (Value::Array(data.clone()), *start, content.len()),
            State::Root { data } => { let _ = data; break; }
        };
        pop_with(&mut stack, &mut state, child_val, start, end, content, &mut metadata);
    }

    match state {
        State::Root { data: Some(Value::Object(map)) } => Some(ParsedEdnResult {
            data: Value::Object(map),
            metadata,
        }),
        State::Record { data, .. } => Some(ParsedEdnResult {
            data: Value::Object(data),
            metadata,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
{:deps {org.clojure/clojure {:mvn/version "1.11.1"}
        ring/ring-core {:mvn/version "1.9.6"}
        compojure {:mvn/version "1.6.3"}
        nrepl/nrepl {:git/url "https://github.com/nrepl/nrepl"
                     :git/sha "abc123"}
        local-lib {:local/root "../local-lib"}}

 :aliases
 {:dev {:extra-deps {ring/ring-mock {:mvn/version "0.4.0"}}}}}
"#;

    // Ported: "extractPackageFile" — deps-edn/extract.spec.ts line 10
    #[test]
    fn extracts_deps() {
        let deps = extract(SAMPLE);
        let clojure = deps
            .iter()
            .find(|d| d.dep_name == "org.clojure:clojure")
            .unwrap();
        assert_eq!(clojure.current_value, "1.11.1");

        let ring = deps
            .iter()
            .find(|d| d.dep_name == "ring:ring-core")
            .unwrap();
        assert_eq!(ring.current_value, "1.9.6");

        // Bare `compojure` → `compojure:compojure`
        let compojure = deps
            .iter()
            .find(|d| d.dep_name == "compojure:compojure")
            .unwrap();
        assert_eq!(compojure.current_value, "1.6.3");
    }

    // Ported: "extractPackageFile" — deps-edn/extract.spec.ts line 10
    #[test]
    fn skips_git_deps() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.dep_name == "nrepl:nrepl"));
    }

    // Ported: "extractPackageFile" — deps-edn/extract.spec.ts line 10
    #[test]
    fn skips_local_deps() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.dep_name == "local-lib:local-lib"));
    }

    // Ported: "extractPackageFile" — deps-edn/extract.spec.ts line 10
    #[test]
    fn extracts_alias_deps() {
        let deps = extract(SAMPLE);
        let mock = deps.iter().find(|d| d.dep_name == "ring:ring-mock");
        assert!(mock.is_some());
        assert_eq!(mock.unwrap().current_value, "0.4.0");
    }

    // Ported: "returns null for empty" — lib/modules/manager/woodpecker/extract.spec.ts line 8
    #[test]
    fn empty_returns_empty() {
        assert!(extract("{}").is_empty());
    }

    // Ported: "returns null for invalid file" — deps-edn/extract.spec.ts line 6
    #[test]
    fn invalid_edn_returns_empty() {
        assert!(extract("123").is_empty());
    }
}

// Ported: "'$input' parses to $output" — modules/manager/deps-edn/parser.spec.ts line 7
#[test]
fn edn_parse_inputs_to_outputs() {
    let cases: &[(&str, Option<serde_json::Value>)] = &[
        ("", None),
        (":foo", None),
        ("foo", None),
        ("1", None),
        ("1.5", None),
        ("1e1", None),
        ("1e-1", None),
        ("[]", None),
        ("}", None),
        ("{}", Some(serde_json::json!({}))),
        ("{", Some(serde_json::json!({}))),
        ("{:foo :foo}", Some(serde_json::json!({"foo": "foo"}))),
        ("{:foo foo}", Some(serde_json::json!({"foo": "foo"}))),
        ("{:foo 1}", Some(serde_json::json!({"foo": "1"}))),
        ("{:foo 1.5}", Some(serde_json::json!({"foo": "1.5"}))),
        ("{:foo 1e1}", Some(serde_json::json!({"foo": "1e1"}))),
        ("{:foo 1e-1}", Some(serde_json::json!({"foo": "1e-1"}))),
        ("{:foo {}}", Some(serde_json::json!({"foo": {}}))),
        ("{{} :foo}", Some(serde_json::json!({}))),
        ("{{} {}}", Some(serde_json::json!({}))),
        ("{:foo :bar}", Some(serde_json::json!({"foo": "bar"}))),
        ("{:foo 1 :bar 2}", Some(serde_json::json!({"foo": "1", "bar": "2"}))),
        ("{:foo {:bar 2} :baz}", Some(serde_json::json!({"foo": {"bar": "2"}}))),
        ("{:foo [:bar :baz]}", Some(serde_json::json!({"foo": ["bar", "baz"]}))),
        ("{:foo {:bar :baz}}", Some(serde_json::json!({"foo": {"bar": "baz"}}))),
        ("{:foo [{:bar :baz}]}", Some(serde_json::json!({"foo": [{"bar": "baz"}]}))),
    ];
    for (input, expected) in cases {
        let result = parse_deps_edn_file(input);
        match expected {
            None => assert!(result.is_none(), "expected None for {input:?}, got Some"),
            Some(expected_val) => {
                let r = result.unwrap_or_else(|| panic!("expected Some for {input:?}"));
                assert_eq!(&r.data, expected_val, "parse({input:?})");
            }
        }
    }
}

// Ported: "extracts file" — modules/manager/deps-edn/parser.spec.ts line 41
#[test]
fn edn_extracts_file() {
    let content = include_str!("../../../../tests/fixtures/deps_edn/deps.edn");
    let result = parse_deps_edn_file(content).expect("should parse");
    // Check the specific dep
    assert_eq!(result.data["deps"]["persistent-sorted-set"]["mvn/version"], "0.1.2");
    // Check metadata
    let dep = &result.data["deps"]["persistent-sorted-set"];
    let meta = result.get_metadata(dep).expect("should have metadata");
    assert_eq!(meta, "{:mvn/version,\"0.1.2\"}");
}

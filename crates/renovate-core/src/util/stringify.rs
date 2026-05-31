use serde_json::Value;

pub fn safe_stringify(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let pairs: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, safe_stringify(v)))
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(safe_stringify).collect();
            format!("[{}]", items.join(", "))
        }
        Value::String(s) => serde_json::to_string(s).unwrap_or_default(),
        other => other.to_string(),
    }
}

pub fn safe_stringify_pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| safe_stringify(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn safe_stringify_string() {
        assert_eq!(safe_stringify(&json!("hello")), "\"hello\"");
    }

    #[test]
    fn safe_stringify_number() {
        assert_eq!(safe_stringify(&json!(42)), "42");
    }

    #[test]
    fn safe_stringify_bool() {
        assert_eq!(safe_stringify(&json!(true)), "true");
    }

    #[test]
    fn safe_stringify_null() {
        assert_eq!(safe_stringify(&Value::Null), "null");
    }

    #[test]
    fn safe_stringify_array() {
        assert_eq!(safe_stringify(&json!([1, 2, 3])), "[1, 2, 3]");
    }

    #[test]
    fn safe_stringify_object() {
        let result = safe_stringify(&json!({"a": 1}));
        assert!(result.contains("\"a\""));
        assert!(result.contains("1"));
    }

    #[test]
    fn safe_stringify_pretty_produces_multiline() {
        let result = safe_stringify_pretty(&json!({"a": 1, "b": 2}));
        assert!(result.contains('\n'));
    }

    #[test]
    fn safe_stringify_empty_object() {
        assert_eq!(safe_stringify(&json!({})), "{}");
    }

    #[test]
    fn safe_stringify_empty_array() {
        assert_eq!(safe_stringify(&json!([])), "[]");
    }
}

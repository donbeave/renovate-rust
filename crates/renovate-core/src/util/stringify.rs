use serde_json::Value;

/// @parity lib/util/stringify.ts full
/// Stringify utilities — lib/util/stringify.ts
///
/// `quickStringify` keeps object key insertion order.
/// `safeStringify` sorts object keys deterministically for stable output.
pub fn quick_stringify(value: &Value) -> String {
    stringify_with_options(value, false)
}

/// Deterministically stringify an arbitrary JSON value by sorting object keys.
pub fn safe_stringify(value: &Value) -> String {
    stringify_with_options(value, true)
}

pub fn safe_stringify_pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| safe_stringify(value))
}

fn stringify_with_options(value: &Value, deterministic: bool) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => serde_json::to_string(v).unwrap_or_else(|_| "\"\"".to_owned()),
        Value::Array(arr) => {
            let inner = arr
                .iter()
                .map(|item| stringify_with_options(item, deterministic))
                .collect::<Vec<String>>()
                .join(",");
            format!("[{}]", inner)
        }
        Value::Object(map) => {
            let mut pairs: Vec<_> = map.iter().collect();
            if deterministic {
                pairs.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(b_key));
            }

            let inner = pairs
                .iter()
                .map(|(key, value)| {
                    let key_json =
                        serde_json::to_string(*key).unwrap_or_else(|_| "\"\"".to_owned());
                    let value_json = stringify_with_options(value, deterministic);
                    format!("{key_json}:{value_json}")
                })
                .collect::<Vec<String>>()
                .join(",");
            format!("{{{inner}}}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Ported: `safeStringify` deterministic and `quickStringify` non-deterministic behavior
    #[test]
    fn test_stringify() {
        let unordered = json!({"z": 1, "a": 2, "m": {"x": 1, "y": 2}});
        let quick_ordered = json!({"z": 1, "a": 2, "m": {"y": 2, "x": 1}});
        let array = json!([1, 2, true, false, null]);

        assert_eq!(quick_stringify(&json!(42)), "42");
        assert_eq!(quick_stringify(&json!("hello")), "\"hello\"");
        assert_eq!(safe_stringify(&Value::Null), "null");
        assert_eq!(quick_stringify(&json!(false)), "false");

        assert_eq!(
            safe_stringify(&unordered),
            r#"{"a":2,"m":{"x":1,"y":2},"z":1}"#
        );
        assert_eq!(
            quick_stringify(&quick_ordered),
            r#"{"z":1,"a":2,"m":{"y":2,"x":1}}"#
        );
        assert_eq!(safe_stringify(&array), "[1,2,true,false,null]");
        assert_eq!(quick_stringify(&array), "[1,2,true,false,null]");
    }
}

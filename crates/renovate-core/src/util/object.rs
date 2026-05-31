use serde_json::Value;

pub fn deep_merge(base: &Value, override_val: &Value) -> Value {
    match (base, override_val) {
        (Value::Object(base_map), Value::Object(over_map)) => {
            let mut result = base_map.clone();
            for (key, val) in over_map {
                let merged = match result.get(key) {
                    Some(existing) => deep_merge(existing, val),
                    None => val.clone(),
                };
                result.insert(key.clone(), merged);
            }
            Value::Object(result)
        }
        (_, over) => over.clone(),
    }
}

pub fn flatten_object(obj: &Value, prefix: &str) -> Vec<(String, Value)> {
    let mut result = Vec::new();
    match obj {
        Value::Object(map) => {
            for (key, val) in map {
                let new_key = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                match val {
                    Value::Object(_) => result.extend(flatten_object(val, &new_key)),
                    _ => result.push((new_key, val.clone())),
                }
            }
        }
        _ => {
            if !prefix.is_empty() {
                result.push((prefix.to_owned(), obj.clone()));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deep_merge_objects() {
        let base = json!({"a": 1, "b": 2});
        let over = json!({"b": 3, "c": 4});
        let result = deep_merge(&base, &over);
        assert_eq!(result, json!({"a": 1, "b": 3, "c": 4}));
    }

    #[test]
    fn deep_merge_nested() {
        let base = json!({"a": {"x": 1, "y": 2}});
        let over = json!({"a": {"y": 3, "z": 4}});
        let result = deep_merge(&base, &over);
        assert_eq!(result, json!({"a": {"x": 1, "y": 3, "z": 4}}));
    }

    #[test]
    fn deep_merge_override_wins() {
        let base = json!(1);
        let over = json!(2);
        assert_eq!(deep_merge(&base, &over), json!(2));
    }

    #[test]
    fn deep_merge_base_null() {
        let base = Value::Null;
        let over = json!({"a": 1});
        assert_eq!(deep_merge(&base, &over), json!({"a": 1}));
    }

    #[test]
    fn deep_merge_override_null() {
        let base = json!({"a": 1});
        let over = Value::Null;
        assert_eq!(deep_merge(&base, &over), Value::Null);
    }

    #[test]
    fn flatten_simple() {
        let obj = json!({"a": 1, "b": 2});
        let flat = flatten_object(&obj, "");
        assert_eq!(flat.len(), 2);
        assert!(flat.contains(&("a".to_owned(), json!(1))));
        assert!(flat.contains(&("b".to_owned(), json!(2))));
    }

    #[test]
    fn flatten_nested() {
        let obj = json!({"a": {"b": 1, "c": {"d": 2}}});
        let flat = flatten_object(&obj, "");
        assert_eq!(flat.len(), 2);
        assert!(flat.contains(&("a.b".to_owned(), json!(1))));
        assert!(flat.contains(&("a.c.d".to_owned(), json!(2))));
    }

    #[test]
    fn flatten_empty_object() {
        let obj = json!({});
        let flat = flatten_object(&obj, "");
        assert!(flat.is_empty());
    }
}

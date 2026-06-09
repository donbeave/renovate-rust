//! Jsonata expression evaluator stub (for parity with TS custom manager support).
//! Full implementation is out of scope for current slices; this allows compilation
//! and basic type checking.

use serde_json::Value;
use std::collections::BTreeMap as Map;

#[derive(Debug, Clone)]
pub struct JsonataExpression {
    #[expect(
        dead_code,
        reason = "retained for API surface / future evaluation in port"
    )]
    expression: String,
}

#[derive(Debug, thiserror::Error)]
pub enum JsonataError {
    #[error("{0}")]
    Msg(String),
}

pub fn get_expression(expression: &str) -> Result<JsonataExpression, JsonataError> {
    Ok(JsonataExpression {
        expression: expression.to_owned(),
    })
}

impl JsonataExpression {
    pub fn evaluate(
        &self,
        _data: &Value,
        _bindings: Option<&Map<String, Value>>,
    ) -> Result<Value, JsonataError> {
        // Stub always returns Null; real eval would parse and run the expr.
        Ok(Value::Null)
    }
}

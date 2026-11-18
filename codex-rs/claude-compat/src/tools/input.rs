//! Reading a Claude tool's JSON input as Claude Code's schemas do: unknown parameters are
//! refused, numbers and booleans may come as strings (`"5"`, `"true"`), and every problem
//! is listed in Claude Code's `InputValidationError` text.

use serde_json::Map;
use serde_json::Value;

pub struct Input<'a> {
    tool: &'a str,
    object: Map<String, Value>,
    issues: Vec<String>,
}

impl<'a> Input<'a> {
    /// Starts reading `value`, the input of `tool`, whose parameters are `known`.
    pub fn new(tool: &'a str, value: Value, known: &[&str]) -> Self {
        let mut issues = Vec::new();
        let object = match value {
            Value::Object(object) => object,
            other => {
                issues.push(format!(
                    "The input is expected as `object` but provided as `{}`",
                    type_name(&other)
                ));
                Map::new()
            }
        };
        for name in object.keys() {
            if !known.contains(&name.as_str()) {
                issues.push(format!("An unexpected parameter `{name}` was provided"));
            }
        }
        Self {
            tool,
            object,
            issues,
        }
    }

    fn mismatch(&mut self, name: &str, expected: &str, value: &Value) {
        self.issues.push(format!(
            "The parameter `{name}` type is expected as `{expected}` but provided as `{}`",
            type_name(value)
        ));
    }

    pub fn string(&mut self, name: &str) -> Option<String> {
        match self.object.get(name)? {
            Value::String(text) => Some(text.clone()),
            other => {
                let other = other.clone();
                self.mismatch(name, "string", &other);
                None
            }
        }
    }

    pub fn required_string(&mut self, name: &str) -> String {
        if !self.object.contains_key(name) {
            self.issues
                .push(format!("The required parameter `{name}` is missing"));
            return String::new();
        }
        self.string(name).unwrap_or_default()
    }

    /// A count (`-A`, `head_limit`, ...): a number or a numeric string, not negative. A
    /// fraction is rounded down.
    pub fn count(&mut self, name: &str) -> Option<usize> {
        let value = self.object.get(name)?.clone();
        let number = match &value {
            Value::Number(number) => number.as_f64(),
            Value::String(text) => text.trim().parse::<f64>().ok(),
            _ => None,
        };
        match number {
            Some(number) if number.is_finite() && number >= 0.0 => Some(number as usize),
            Some(_) => {
                self.issues.push(format!(
                    "The parameter `{name}` is expected as a number not below 0"
                ));
                None
            }
            None => {
                self.mismatch(name, "number", &value);
                None
            }
        }
    }

    /// A boolean, or the string `"true"` or `"false"`.
    pub fn boolean(&mut self, name: &str) -> Option<bool> {
        match self.object.get(name)? {
            Value::Bool(value) => Some(*value),
            Value::String(text) if text == "true" => Some(true),
            Value::String(text) if text == "false" => Some(false),
            other => {
                let other = other.clone();
                self.mismatch(name, "boolean", &other);
                None
            }
        }
    }

    /// Ends reading: `Err` with Claude Code's message when anything was wrong.
    pub fn finish(self) -> Result<(), String> {
        if self.issues.is_empty() {
            return Ok(());
        }
        let plural = if self.issues.len() > 1 {
            "issues"
        } else {
            "issue"
        };
        Err(format!(
            "InputValidationError: {} failed due to the following {plural}:\n{}",
            self.tool,
            self.issues.join("\n")
        ))
    }
}

fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

use std::collections::HashMap;

static TEMPLATE_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"\{\{\{([^}]+)\}\}\}|\{\{([^}]+)\}\}").expect("valid template regex")
});

pub fn compile(template: &str, context: &HashMap<&str, &str>) -> String {
    TEMPLATE_RE
        .replace_all(template, |caps: &regex::Captures<'_>| {
            let key = caps
                .get(1)
                .or_else(|| caps.get(2))
                .map(|m| m.as_str().trim())
                .unwrap_or("");
            context.get(key).copied().unwrap_or("")
        })
        .into_owned()
}

pub fn compile_with_env(
    template: &str,
    context: &HashMap<&str, &str>,
    env: &HashMap<String, String>,
) -> String {
    TEMPLATE_RE
        .replace_all(template, |caps: &regex::Captures<'_>| {
            let key = caps
                .get(1)
                .or_else(|| caps.get(2))
                .map(|m| m.as_str().trim())
                .unwrap_or("");
            context
                .get(key)
                .copied()
                .or_else(|| env.get(key).map(|s| s.as_str()))
                .unwrap_or("")
        })
        .into_owned()
}

pub fn validate_template(template: &str) -> Result<(), String> {
    let mut depth = 0i32;
    let mut i = 0;
    let bytes = template.as_bytes();
    while i < bytes.len() {
        if i + 2 < bytes.len() && &bytes[i..i + 3] == b"{{{" {
            depth += 1;
            i += 3;
        } else if i + 2 < bytes.len() && &bytes[i..i + 3] == b"}}}" {
            depth -= 1;
            if depth < 0 {
                return Err("Unmatched closing }}}}".to_owned());
            }
            i += 3;
        } else if i + 1 < bytes.len() && &bytes[i..i + 2] == b"{{" {
            depth += 1;
            i += 2;
        } else if i + 1 < bytes.len() && &bytes[i..i + 2] == b"}}" {
            depth -= 1;
            if depth < 0 {
                return Err("Unmatched closing }}".to_owned());
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    if depth != 0 {
        return Err("Unmatched opening {{".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_simple_substitution() {
        let mut ctx = HashMap::new();
        ctx.insert("name", "world");
        assert_eq!(compile("Hello {{name}}!", &ctx), "Hello world!");
    }

    #[test]
    fn compile_multiple_vars() {
        let mut ctx = HashMap::new();
        ctx.insert("a", "1");
        ctx.insert("b", "2");
        assert_eq!(compile("{{a}}-{{b}}", &ctx), "1-2");
    }

    #[test]
    fn compile_missing_var_produces_empty() {
        let ctx = HashMap::new();
        assert_eq!(compile("Hello {{name}}!", &ctx), "Hello !");
    }

    #[test]
    fn compile_triple_brace() {
        let mut ctx = HashMap::new();
        ctx.insert("val", "test");
        assert_eq!(compile("{{{val}}}", &ctx), "test");
    }

    #[test]
    fn compile_trims_whitespace() {
        let mut ctx = HashMap::new();
        ctx.insert("name", "test");
        assert_eq!(compile("{{ name }}", &ctx), "test");
    }

    #[test]
    fn compile_no_placeholders() {
        let ctx = HashMap::new();
        assert_eq!(compile("no templates here", &ctx), "no templates here");
    }

    #[test]
    fn compile_empty_template() {
        let ctx = HashMap::new();
        assert_eq!(compile("", &ctx), "");
    }

    #[test]
    fn compile_with_env_falls_back_to_env() {
        let ctx = HashMap::new();
        let mut env = HashMap::new();
        env.insert("HOME".to_owned(), "/home/user".to_owned());
        assert_eq!(compile_with_env("{{HOME}}", &ctx, &env), "/home/user");
    }

    #[test]
    fn compile_with_env_prefers_context() {
        let mut ctx = HashMap::new();
        ctx.insert("KEY", "from_ctx");
        let mut env = HashMap::new();
        env.insert("KEY".to_owned(), "from_env".to_owned());
        assert_eq!(compile_with_env("{{KEY}}", &ctx, &env), "from_ctx");
    }

    #[test]
    fn validate_template_valid() {
        assert!(validate_template("Hello {{name}}!").is_ok());
    }

    #[test]
    fn validate_template_valid_triple() {
        assert!(validate_template("{{{val}}}").is_ok());
    }

    #[test]
    fn validate_template_no_placeholders() {
        assert!(validate_template("plain text").is_ok());
    }

    #[test]
    fn validate_template_unmatched_opening() {
        assert!(validate_template("{{name").is_err());
    }

    #[test]
    fn validate_template_unmatched_closing() {
        assert!(validate_template("name}}").is_err());
    }
}

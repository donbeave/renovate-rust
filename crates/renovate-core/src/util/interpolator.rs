use std::collections::HashMap;

static INTERPOLATE_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"\{\{([^}]+)\}\}").expect("valid interpolation regex")
});

pub fn interpolate(template: &str, context: &HashMap<&str, &str>) -> String {
    INTERPOLATE_RE
        .replace_all(template, |caps: &regex::Captures<'_>| {
            let key = caps.get(1).unwrap().as_str().trim();
            context.get(key).copied().unwrap_or("")
        })
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_simple() {
        let mut ctx = HashMap::new();
        ctx.insert("name", "world");
        assert_eq!(interpolate("Hello {{name}}!", &ctx), "Hello world!");
    }

    #[test]
    fn interpolate_multiple() {
        let mut ctx = HashMap::new();
        ctx.insert("a", "1");
        ctx.insert("b", "2");
        assert_eq!(interpolate("{{a}}-{{b}}", &ctx), "1-2");
    }

    #[test]
    fn interpolate_missing() {
        let ctx = HashMap::new();
        assert_eq!(interpolate("{{missing}}", &ctx), "");
    }

    #[test]
    fn interpolate_no_vars() {
        let ctx = HashMap::new();
        assert_eq!(interpolate("plain text", &ctx), "plain text");
    }

    #[test]
    fn interpolate_empty_template() {
        let ctx = HashMap::new();
        assert_eq!(interpolate("", &ctx), "");
    }

    #[test]
    fn interpolate_whitespace_in_key() {
        let mut ctx = HashMap::new();
        ctx.insert("name", "test");
        assert_eq!(interpolate("{{ name }}", &ctx), "test");
    }
}

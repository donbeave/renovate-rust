pub fn parse_number(input: &str) -> Option<f64> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<f64>().ok()
}

pub fn is_positive_number(value: Option<f64>) -> bool {
    value.is_some_and(|v| v > 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_integer() {
        assert_eq!(parse_number("42"), Some(42.0));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn parse_number_float() {
        assert_eq!(parse_number("3.14"), Some(3.14));
    }

    #[test]
    fn parse_number_negative() {
        assert_eq!(parse_number("-5"), Some(-5.0));
    }

    #[test]
    fn parse_number_zero() {
        assert_eq!(parse_number("0"), Some(0.0));
    }

    #[test]
    fn parse_number_whitespace() {
        assert_eq!(parse_number("  42  "), Some(42.0));
    }

    #[test]
    fn parse_number_empty() {
        assert_eq!(parse_number(""), None);
    }

    #[test]
    fn parse_number_invalid() {
        assert_eq!(parse_number("abc"), None);
    }

    #[test]
    fn parse_number_nan() {
        assert!(parse_number("NaN").is_none() || parse_number("NaN").unwrap().is_nan());
    }

    #[test]
    fn is_positive_number_some_positive() {
        assert!(is_positive_number(Some(1.0)));
    }

    #[test]
    fn is_positive_number_some_zero() {
        assert!(!is_positive_number(Some(0.0)));
    }

    #[test]
    fn is_positive_number_some_negative() {
        assert!(!is_positive_number(Some(-1.0)));
    }

    #[test]
    fn is_positive_number_none() {
        assert!(!is_positive_number(None));
    }
}

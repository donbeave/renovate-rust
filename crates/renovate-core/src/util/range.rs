pub fn get_endpoints(range_str: &str) -> Vec<(i64, i64)> {
    let mut endpoints = Vec::new();
    for part in range_str.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(dash_pos) = trimmed.find('-') {
            if dash_pos == 0 {
                if let Ok(n) = trimmed.parse::<i64>() {
                    endpoints.push((n, n));
                }
            } else {
                let start: i64 = trimmed[..dash_pos].trim().parse().unwrap_or(0);
                let end: i64 = trimmed[dash_pos + 1..].trim().parse().unwrap_or(0);
                endpoints.push((start, end));
            }
        } else if let Ok(n) = trimmed.parse::<i64>() {
            endpoints.push((n, n));
        }
    }
    endpoints
}

pub fn expand_range(range_str: &str) -> Vec<i64> {
    let endpoints = get_endpoints(range_str);
    let mut result = Vec::new();
    for (start, end) in &endpoints {
        let (lo, hi) = if start <= end {
            (*start, *end)
        } else {
            (*end, *start)
        };
        for n in lo..=hi {
            result.push(n);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_endpoints_single() {
        assert_eq!(get_endpoints("5"), vec![(5, 5)]);
    }

    #[test]
    fn get_endpoints_range() {
        assert_eq!(get_endpoints("1-5"), vec![(1, 5)]);
    }

    #[test]
    fn get_endpoints_mixed() {
        assert_eq!(get_endpoints("1-5,7,9-11"), vec![(1, 5), (7, 7), (9, 11)]);
    }

    #[test]
    fn get_endpoints_empty() {
        assert!(get_endpoints("").is_empty());
    }

    #[test]
    fn expand_range_simple() {
        assert_eq!(expand_range("1-5"), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn expand_range_single() {
        assert_eq!(expand_range("7"), vec![7]);
    }

    #[test]
    fn expand_range_mixed() {
        assert_eq!(expand_range("1-3,5,7-8"), vec![1, 2, 3, 5, 7, 8]);
    }

    #[test]
    fn expand_range_empty() {
        assert!(expand_range("").is_empty());
    }

    #[test]
    fn expand_range_reverse() {
        assert_eq!(expand_range("5-3"), vec![3, 4, 5]);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mean {
    count: u64,
    sum: f64,
}

impl Mean {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
    }

    pub fn calculate(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        Some(self.sum / self.count as f64)
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.sum = 0.0;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Variance {
    count: u64,
    mean: f64,
    m2: f64,
}

impl Variance {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, value: f64) {
        self.count += 1;
        let delta = value - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = value - self.mean;
        self.m2 += delta * delta2;
    }

    pub fn variance(&self) -> Option<f64> {
        if self.count < 2 {
            return None;
        }
        Some(self.m2 / (self.count - 1) as f64)
    }

    pub fn population_variance(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        Some(self.m2 / self.count as f64)
    }

    pub fn stddev(&self) -> Option<f64> {
        self.variance().map(f64::sqrt)
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.mean = 0.0;
        self.m2 = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_empty() {
        let m = Mean::new();
        assert_eq!(m.calculate(), None);
    }

    #[test]
    fn mean_single_value() {
        let mut m = Mean::new();
        m.update(5.0);
        assert_eq!(m.calculate(), Some(5.0));
    }

    #[test]
    fn mean_multiple_values() {
        let mut m = Mean::new();
        m.update(2.0);
        m.update(4.0);
        m.update(6.0);
        assert_eq!(m.calculate(), Some(4.0));
    }

    #[test]
    fn mean_count() {
        let mut m = Mean::new();
        assert_eq!(m.count(), 0);
        m.update(1.0);
        m.update(2.0);
        assert_eq!(m.count(), 2);
    }

    #[test]
    fn mean_reset() {
        let mut m = Mean::new();
        m.update(1.0);
        m.reset();
        assert_eq!(m.calculate(), None);
        assert_eq!(m.count(), 0);
    }

    #[test]
    fn variance_empty() {
        let v = Variance::new();
        assert_eq!(v.variance(), None);
    }

    #[test]
    fn variance_single_value() {
        let mut v = Variance::new();
        v.update(5.0);
        assert_eq!(v.variance(), None);
    }

    #[test]
    fn variance_two_values() {
        let mut v = Variance::new();
        v.update(2.0);
        v.update(4.0);
        let var = v.variance().unwrap();
        assert!((var - 2.0).abs() < 1e-10);
    }

    #[test]
    fn variance_mean() {
        let mut v = Variance::new();
        v.update(2.0);
        v.update(4.0);
        v.update(6.0);
        assert!((v.mean() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn stddev_calculates() {
        let mut v = Variance::new();
        v.update(2.0);
        v.update(4.0);
        let sd = v.stddev().unwrap();
        assert!((sd - 1.4142135623730951).abs() < 1e-10);
    }

    #[test]
    fn variance_reset() {
        let mut v = Variance::new();
        v.update(1.0);
        v.reset();
        assert_eq!(v.count(), 0);
        assert_eq!(v.variance(), None);
    }
}

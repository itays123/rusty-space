//! A module to use linear dependence

#[derive(Debug)]
pub enum Ratio {
    /// When we try to devide by zero
    Invalid,
    /// When both values are zero
    Zeros,
    /// A real number
    Real(f64)
}

impl Ratio {

    /// Compute the ratio between two scalars
    pub fn compute(x: f64, y: f64) -> Ratio {
        if x == 0.0 && y == 0.0 {
            Self::Zeros
        } else if x == 0.0 || y == 0.0 {
            Self::Invalid
        } else {
            Self::Real(x / y)
        }
    }

    pub fn is_valid(self) -> bool {
        match self {
            Self::Invalid => false,
            _ => true
        }
    }
}

/// Compare two ratios. 
/// A zero ratio is equal to all, an invalid ratio is equal to nothing
impl PartialEq for Ratio {

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Invalid, _) | (_, Self::Invalid) => false,
            (Self::Zeros, _) | (_, Self::Zeros) => true,
            (Self::Real(x), Self::Real(y)) => x == y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid() {
        assert!(!Ratio::compute(0.0, 3.0).is_valid());
        assert!(!Ratio::compute(1.0, 0.0).is_valid());
    }

    #[test]
    fn zeros() {
        assert_eq!(Ratio::compute(0.0, 0.0), Ratio::compute(1.0, 2.0));
    }

    #[test]
    fn real() {
        assert_eq!(Ratio::compute(1.0, 2.0), Ratio::compute(2.0, 4.0));
        assert_ne!(Ratio::compute(1.0, 2.0), Ratio::compute(2.0, 3.0));
    }
}
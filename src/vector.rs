use std::ops::{Add, Sub, Mul};

mod lindep;
use lindep::Ratio;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector(pub f64, pub f64, pub f64);

impl Vector {

    /// The mathematical length of a vector
    pub fn length(&self) -> f64 {
        ((*self) * (*self)).sqrt()
    }

    /// Check if two vectors are linearly dependent
    pub fn is_lindep(&self, other: &Vector) -> bool {
        let Vector(u1, u2, u3) = *self;
        let Vector(v1, v2, v3) = *other;
        
        let ratio1 = Ratio::compute(u1, v1);
        let ratio2 = Ratio::compute(u2, v2);
        let ratio3 = Ratio::compute(u3, v3);

        ratio1 == ratio2 && ratio1 == ratio3 && ratio2 == ratio3

    }

    /// Compute the angle between two vectors, in radians
    pub fn angle_between(u: &Vector, v: &Vector) -> f64 {
        (((*u) * (*v)) / (u.length() * v.length())).acos()
    }

    /// Compute a vector that is perpendicular to two given vectors
    pub fn vectoric_product(u: &Vector, v: &Vector) -> Vector {
        let Vector(u1, u2, u3) = *u;
        let Vector(v1, v2, v3) = *v;
        Vector(u2 * v3 - u3 * v2, u3 * v1 - u1 * v3, u1 * v2 - u2 * v1)
    }
}

impl Add for Vector {
    type Output = Self;

    /// Vectoric addition of two vectors in space
    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vector {
    type Output = Self;

    /// Vectoric subtraction of two vectors in space
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for Vector {
    type Output = f64;

    /// Scalaric product of two vectors
    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

/// multiply a vector by a scalar
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let Vector(x, y, z) = rhs;
        Vector(self * x, self * y, self * z)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn sum_works() {
        assert_eq!(Vector(0.0, 0.0, 1.0) + Vector(0.0, 1.0, 0.0), Vector(0.0, 1.0, 1.0));
        assert_eq!(Vector(0.0, 0.0, 1.0) + Vector(0.0, 1.0, 1.0), Vector(0.0, 1.0, 2.0));
    }

    #[test]
    fn subtract_works() {
        assert_eq!(Vector(0.0, 0.0, 1.0) - Vector(0.0, 1.0, 0.0), Vector(0.0, -1.0, 1.0)); 
        assert_eq!(Vector(0.0, 0.0, 1.0) - Vector(0.0, 1.0, 1.0), Vector(0.0, -1.0, 0.0));
    }

    #[test]
    fn scalaric_product_works() {
        assert_eq!(Vector(0.0, 0.0, 1.0) * Vector(0.0, 1.0, 0.0), 0.0);
        assert_eq!(Vector(0.0, 0.0, 1.0) * Vector(0.0, 1.0, 1.0), 1.0);
    }

    #[test]
    fn multiply_by_scalar() {
        assert_eq!(2.0 * Vector(1.0, 2.0, 3.0), Vector(2.0, 4.0, 6.0));
    }

    #[test]
    fn length_works() {
        assert_eq!(Vector(0.0, 0.0, 1.0).length(), 1.0);
        assert_eq!(Vector(2.0, 2.0, 1.0).length(), 3.0);
    }

    #[test]
    fn dependency_works() {
        assert!(Vector(0.0, 0.0, 1.0).is_lindep(&Vector(0.0, 0.0, 2.0)));
        assert!(!Vector(0.0, 0.0, 1.0).is_lindep(&Vector(0.0, 1.0, 2.0)));
        assert!(Vector(2.0, 2.0, 1.0).is_lindep(&Vector(4.0, 4.0, 2.0)));
    }

    #[test]
    fn angle_works() {
        const EPSILON: f64 = 0.00001;
        assert_eq!(Vector::angle_between(&Vector(0.0, 0.0, 1.0), &Vector(0.0, 1.0, 0.0)), PI / 2.0);
        assert!((Vector::angle_between(&Vector(0.0, 0.0, 1.0), &Vector(0.0, 1.0, 1.0)) - PI / 4.0).abs() < EPSILON);
    }

    #[test]
    fn vectoric_product_works() {
        assert!(Vector::vectoric_product(&Vector(0.0, 0.0, 1.0), &Vector(0.0, 1.0, 0.0)).is_lindep(&Vector(1.0, 0.0, 0.0)));
    }
}
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector(f64, f64, f64);

impl Vector {

    /// The mathematical length of a vector
    pub fn length(self) -> f64 {
        (self * self).sqrt()
    }

    /// Check if two vectors are linearly dependent
    pub fn is_lindep(self, other: Vector) -> bool {
        let ratio = self.0 / other.0;
        self.1 / other.1 == ratio && self.2 / other.2 == ratio
    }

    /// Compute the angle between two vectors, in radians
    pub fn angle_between(u: Vector, v: Vector) -> f64 {
        ((u * v) / (u.length() * v.length())).acos()
    }

    /// Compute a vector that is perpendicular to two given vectors
    pub fn vectoric_product(u: Vector, v: Vector) -> Vector {
        let Vector(u1, u2, u3) = u;
        let Vector(v1, v2, v3) = v;
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

#[cfg(test)]
mod vector_test {
    use super::*;

    /// Test the sum of two vectors
    #[test]
    fn sum_test() {
        assert_eq!(Vector(0_f64, 0_f64, 1_f64) + Vector(0_f64, 1_f64, 0_f64), Vector(0_f64, 1_f64, 1_f64));
        assert_eq!(Vector(0_f64, 0_f64, 1_f64) + Vector(0_f64, 1_f64, 1_f64), Vector(0_f64, 1_f64, 2_f64));
    }
}
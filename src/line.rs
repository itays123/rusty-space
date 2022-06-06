//! A line module

use crate::vector::Vector;
use crate::equation::EquationSolution;

mod relations;
pub struct Line {
    pub point: Vector,
    pub direction: Vector    
}

impl Line {
    
    pub fn new(point: Vector, direction: Vector) -> Line {
        Line { point, direction }
    }

    pub fn is_on_line(&self, other_point: &Vector) -> bool {
        self.distance_from_point(other_point) == 0.0
    }

    fn distance_from_point(&self, other_point: &Vector) -> f64 {
        // Find a point p1 = p + tu where op1 * direction = 0
        // for every dimension of p1:
        // xp1 = xp + tu1
        // xp1 - xo = xp - xo + tu1
        // therefore op1 = (xp - xo + tu1, yp - yo + tu2, zp - zo + tu3)
        // and op1 * direction = u1(xp - xo + tu1) + u2(yp - yo + tu2) + u3(zp - zo + tu3) = 0
        // simplify: (u1^2 + u2^2 + u3^2)*t + u1(xp - xo) + u2(yp-yo) + u3(zp - zo) = 0
        // (u^2) * t + u * (p - o) = 0
        let coefficient = self.direction * self.direction;
        let sum = self.direction * (self.point - *other_point);

        let solution = EquationSolution::compute(coefficient, sum);
        if let EquationSolution::Real(t) = solution {
            let differece = self.point - *other_point + t * self.direction;
            differece.length()
        }
        else {
            // mathematically not possible
            panic!("This should never happen")
        }

    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.direction == other.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_works() {
        let line = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let distance = line.distance_from_point(&Vector(0.0, 1.0, 0.0));
        assert_eq!(distance, 1.0);
        let distance = line.distance_from_point(&Vector(0.0, 0.0, 1.0));
        assert_eq!(distance, 1.0);
        let distance = line.distance_from_point(&Vector(0.0, 0.0, 0.0));
        assert_eq!(distance, 0.0);
    }
}
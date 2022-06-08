//! A line module

use std::f64::consts::PI;
use crate::vector::Vector;
use crate::equation::EquationSolution;

use self::relations::LineRelations;

pub mod relations;

#[derive(Debug)]
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

    /// find intersection of two lines, if existing and single
    pub fn intersection(line1: &Line, line2: &Line) -> Option<Vector> {
        // given {p1, u1} = line1 and { p2, u2 } = line2
        // find a point such that p1 + tu1 = p2 + su2
        // for dimension x: xp1 + t * xu1 = xp2 + s * xu2
        // simplify: t * xu1 - s * xu2 = xp2 - xp1;
        // therefore, for the entire vector: tu1 - su2 = p2 - p1;
        let Vector(constx, consty, constz) = line2.point - line1.point;
        let Vector(coefficient_tx, coefficient_ty, coefficient_tz) = line1.direction;
        let Vector(coefficient_sx, coefficient_sy, coefficient_sz) = line2.direction;

        // solve for t and s
        let eq1 = (coefficient_tx, -coefficient_sx, constx);
        let eq2 = (coefficient_ty, -coefficient_sy, consty);
        let eq3 = (coefficient_tz, -coefficient_sz, constz);

        let result: Option<(f64, f64)>;
        if eq1 == (0.0, 0.0, 0.0) { // plane containing the lines is vertical to the x axis 
            result = EquationSolution::compute_multiple(eq3, eq2)
        } else if eq2 == (0.0, 0.0, 0.0) { // plane containing the lines is vertical to the y axis 
            result = EquationSolution::compute_multiple(eq1, eq3);
        } else {
            result = EquationSolution::compute_multiple(eq1, eq2);
        }

        if let Some((t, s)) = result {
            // check for z
            let intersection_t = line1.point + t * line1.direction;
            let intersection_s = line2.point + s * line2.direction;
            if intersection_t == intersection_s { // same z value, lines intersect
                Some(intersection_t)
            } else { // lines don't intersect
                None
            }
        } else {
            None
        }
    }

    /// Find the angle (0 < x < PI/2) between two lines, in radians
    pub fn angle_between(line1: &Line, line2: &Line) -> f64 {
        let angle = Vector::angle_between(&line1.direction, &line2.direction);
        // angle between lines must be between 0 and 90 degrees
        if angle > PI / 2.0 {
            PI - angle
        } else {
            angle
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        LineRelations::of(self, other) == LineRelations::Unite
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

    #[test]
    fn intersection_works() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 0.0, 0.0), Vector(0.0, 1.0, 0.0)); // the y axis
        assert_eq!(Line::intersection(&line1, &line2).unwrap(), Vector(0.0, 0.0, 0.0))
    }

    #[test]
    fn angle_works() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 0.0, 0.0), Vector(-1.0, 1.0, 0.0));
        assert_eq!(Line::angle_between(&line1, &line2), PI / 4.0)
    }
}
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

    pub fn is_on_line(self, other_point: Vector) -> bool {
        let Vector(x, y, z) = other_point;
        // compute t where (x, y, z) = point + t * direction
        // meaning, x = xp + t * u1 => u1 * t = x - xp
        let Vector(xp, yp, zp) = self.point;
        let Vector(u1, u2, u3) = self.direction;

        let sol_x = EquationSolution::compute(u1, x - xp);
        let sol_y = EquationSolution::compute(u2, y - yp);
        let sol_z = EquationSolution::compute(u3, z - zp);

        sol_x == sol_y && sol_x == sol_z && sol_y == sol_z
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.direction == other.direction
    }
}
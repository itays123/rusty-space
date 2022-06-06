//! Represents plains in a 3d space

use std::f64::consts::PI;

use crate::{vector::Vector, line::Line};

pub struct Plain {
    plumb: Vector,
    constant_d: f64
}

impl Plain {
    /// Generates a plain from an origin and two directions
    /// # Panics:
    /// - If the two directions are linearly dependent, and cannot form a plane
    pub fn new(origin: &Vector, dir1: &Vector, dir2: &Vector) -> Plain {
        if dir1.is_lindep(dir2) {
            panic!("Two linearly dependent vectors cannot form a plane")
        }
        let plumb = Vector::vectoric_product(dir1, dir2);
        // we went plumb * origin + d = 0, therefore d = -1 * plumb + origin
        let constant_d = -1.0 * (plumb * (*origin));
        Plain { plumb, constant_d }
    }

    fn compute(&self, point: &Vector) -> f64 {
        self.plumb * (*point) + self.constant_d
    }

    // Calculate distance between a given point and this plain
    pub fn distance_from(&self, point: &Vector) -> f64 {
        self.compute(point) / self.plumb.length()
    }

    // Check if the plain contains a given point
    pub fn contains_point(&self, point: &Vector) -> bool {
        self.compute(point) == 0.0
    }

    // Check if a plain contains a given line
    pub fn contains_line(&self, line: &Line) -> bool {
        // Point is on line, and the direction of the line is vertical to the plumb
        self.contains_point(&line.point) && (line.direction * self.plumb == 0.0) 
    }

    // Compute the angle between the plain and a given vector
    pub fn angle_with_vector(&self, vector: &Vector) -> f64 {
        (PI / 2.0) - Vector::angle_between(&self.plumb, vector)
    }

    // Compute the angle (0 <= x <= PI / 2) between the plain and a given line
    pub fn angle_with_line(&self, line:&Line) -> f64 {
        let angle = self.angle_with_vector(&line.direction);
        // angle between lines must be between 0 and 90 degrees
        angle.abs()
    }
}
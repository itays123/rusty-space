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

    /// Generates a plain from intersecting lines
    /// # Panics:
    /// - If the two lines provided are not intersecting and cannot form a plain
    pub fn from_intersecting_lines(line1: &Line, line2: &Line) -> Plain {
        if let Some(intersection) = Line::intersection(line1, line2) {
            Plain::new(&intersection, &line1.direction, &line2.direction)
        } else {
            panic!("Lines don't intersect");
        }
    }

    /// Generates a plain from three points
    /// # Panics:
    /// - If the three points provided form a line and not a plain
    pub fn from_three_points(point1: &Vector, point2: &Vector, point3: &Vector) -> Plain {
        let dir1 = *point2 - *point1;
        let dir2 = *point3 - *point1;
        Plain::new(point1, &dir1, &dir2)
    }

    fn compute(&self, point: &Vector) -> f64 {
        self.plumb * (*point) + self.constant_d
    }

    // Calculate distance between a given point and this plain
    pub fn distance_from(&self, point: &Vector) -> f64 {
        self.compute(point).abs() / self.plumb.length()
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

    // Compute the angle (0 <= x <= PI/2) between two plains
    pub fn angle_between(plain1: &Plain, plain2: &Plain) -> f64 {
        let angle = Vector::angle_between(&plain1.plumb, &plain2.plumb);
        if angle > PI / 2.0 {
            PI - angle
        } else {
            angle
        }
    }
}
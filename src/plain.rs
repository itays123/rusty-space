//! Represents plains in a 3d space

pub mod line_relations;
pub mod relations;

use std::f64::consts::PI;

use crate::{vector::Vector, line::{Line, relations::LineRelations}, equation::EquationSolution};

use self::line_relations::PlainLineRelations;

#[derive(PartialEq, Debug)]
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

    /// Generates a plain from intersecting or parallel lines
    /// # Panics:
    /// - If the two lines provided are not intersecting and cannot form a plain
    pub fn from_two_lines(line1: &Line, line2: &Line) -> Plain {
        match LineRelations::of(line1, line2) {
            LineRelations::Parallel(_) => {
                let dir2 = line2.point - line1.point;
                Plain::new(&line1.point, &line1.direction, &dir2)
            },
            LineRelations::Intersect(intersection, _) => {
                Plain::new(&intersection, &line1.direction, &line2.direction)
            },
            LineRelations::Unite => panic!("Lines unite and form infinite shared planes!"),
            LineRelations::Foreign(_, _) => panic!("Forign lines have no common plane!")
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

    /// Calculate distance between a given point and this plain
    pub fn distance_from(&self, point: &Vector) -> f64 {
        self.compute(point).abs() / self.plumb.length()
    }

    /// Check if the plain contains a given point
    pub fn contains_point(&self, point: &Vector) -> bool {
        self.compute(point) == 0.0
    }

    /// Check if a plain contains a given line
    pub fn contains_line(&self, line: &Line) -> bool {
        // Point is on line, and the direction of the line is vertical to the plumb
        self.contains_point(&line.point) && (line.direction * self.plumb == 0.0) 
    }

    /// Compute the angle between the plain and a given vector
    pub fn angle_with_vector(&self, vector: &Vector) -> f64 {
        (PI / 2.0) - Vector::angle_between(&self.plumb, vector)
    }

    /// Compute the angle (0 <= x <= PI / 2) between the plain and a given line
    pub fn angle_with_line(&self, line:&Line) -> f64 {
        let angle = self.angle_with_vector(&line.direction);
        // angle between lines must be between 0 and 90 degrees
        angle.abs()
    }

    /// Compute the angle (0 <= x <= PI/2) between two plains
    pub fn angle_between(plain1: &Plain, plain2: &Plain) -> f64 {
        let angle = Vector::angle_between(&plain1.plumb, &plain2.plumb);
        if angle > PI / 2.0 {
            PI - angle
        } else {
            angle
        }
    }

    /// Compute the relation between a plane and a line
    pub fn relation_with_line(&self, line: &Line) -> PlainLineRelations {
        // Find a point on the line, p1 = p + tu, such that compute(p1) == 0
        // Simplify: plumb * p1 + d = 0
        // plumb * (p + tu) + d = 0
        // t * (plumb * u) + d + plumb * p = 0
        let coefficient = self.plumb * line.direction;
        let constant = self.plumb * line.point + self.constant_d;
        match EquationSolution::compute(coefficient, constant) {
            EquationSolution::Real(t) => {
                let intersection = line.point + t * line.direction;
                let angle = self.angle_with_line(line);
                PlainLineRelations::Intersect(intersection, angle)
            },
            EquationSolution::None => {
                // No intersection -> Parallel
                let distance = self.distance_from(&line.point);
                PlainLineRelations::Parallel(distance)
            }
            EquationSolution::Undefined => PlainLineRelations::Containing
        }
    }

    /// Compute a constant distance between plains.
    /// # Panics:
    /// - If the two plains provided intersect, they don't have a constant distance between them.
    pub fn distance_between(plain1: &Plain, plain2: &Plain) -> f64 {
        if !plain1.plumb.is_lindep(&plain2.plumb) {
            panic!("The two planes must be parallel or uniting to calculate the distance between them")
        }

        let (d1, plumb1) = (plain1.constant_d, &plain1.plumb);
        let (d2, plumb2) = (plain2.constant_d, &plain2.plumb);
        let plumb_ratio = (*plumb1 / *plumb2).unwrap();
        let d2 = d2 * plumb_ratio;
        let difference = (d1 - d2).abs();
        difference / plumb1.length()
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_plain() {
        let plain = Plain::new(&Vector(0.0,0.0,0.0), &Vector(1.0, 0.0, 0.0), &Vector(0.0, 1.0, 0.0)); // z=0
        assert_eq!(plain.plumb, Vector(0.0, 0.0, 1.0));
        assert_eq!(plain.constant_d, 0.0);
    }

    #[test]
    #[should_panic]
    fn creates_new_plain_lindep_dirs() {
        Plain::new(&Vector(0.0,0.0,0.0), &Vector(1.0, 0.0, 0.0), &Vector(2.0, 0.0, 0.0));
    }

    #[test]
    fn creates_new_from_intersection() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 0.0, 0.0), Vector(0.0, 1.0, 0.0)); // the y axis
        let plain = Plain::from_two_lines(&line1, &line2);
        assert_eq!(plain.plumb, Vector(0.0, 0.0, 1.0));
        assert_eq!(plain.constant_d, 0.0);
    }

    #[test]
    fn creates_new_from_parallel() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 1.0, 0.0), Vector(1.0, 0.0, 0.0)); // the y axis
        let plain = Plain::from_two_lines(&line1, &line2);
        assert_eq!(plain.plumb, Vector(0.0, 0.0, 1.0));
        assert_eq!(plain.constant_d, 0.0);
    }

    #[test]
    #[should_panic]
    fn creates_new_plain_foreign_lines() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 0.0, 1.0), Vector(0.0, 1.0, 0.0)); // the y axis
        Plain::from_two_lines(&line1, &line2);
    }

    #[test]
    fn creates_new_from_3_points() {
        let plain = Plain::from_three_points(&Vector(0.0,0.0,0.0), &Vector(1.0, 0.0, 0.0), &Vector(0.0, 1.0, 0.0)); // z=0
        assert_eq!(plain.plumb, Vector(0.0, 0.0, 1.0));
        assert_eq!(plain.constant_d, 0.0);
    }

    #[test]
    #[should_panic]
    fn creates_new_from_3_points_same_line() {
        Plain::from_three_points(&Vector(0.0,0.0,0.0), &Vector(1.0, 0.0, 0.0), &Vector(3.0, 0.0, 0.0)); // z=0
    }

    #[test]
    fn contains_point() {
        let origin = Vector(0.0,0.0,0.0);
        let plain = Plain::from_three_points(&origin, &Vector(1.0, 0.0, 0.0), &Vector(0.0, 1.0, 0.0)); // z=0
        assert!(plain.contains_point(&origin));
        assert!(!plain.contains_point(&Vector(0.0, 0.0, 1.0)))
    }

    #[test]
    fn contains_line() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 0.0, 0.0), Vector(0.0, 1.0, 0.0)); // the y axis
        let line3 = Line::new(Vector(0.0, 0.0, 0.0), Vector(0.0, 0.0, 1.0)); // the z axis
        let plain = Plain::from_two_lines(&line1, &line2);
        assert!(plain.contains_line(&line1));
        assert!(plain.contains_line(&line2));
        assert!(!plain.contains_line(&line3));
    }

    #[test]
    fn distance_from_points() {
        let origin = Vector(0.0,0.0,0.0);
        let plain = Plain::from_three_points(&origin, &Vector(1.0, 0.0, 0.0), &Vector(0.0, 1.0, 0.0)); // z=0
        assert_eq!(plain.distance_from(&origin), 0.0);
        assert_eq!(plain.distance_from(&Vector(0.0, 0.0, 1.0)), 1.0);
    }

    #[test]
    fn distance_between_plains() {
        let plain1 = Plain::from_three_points(&Vector(0.0,0.0,0.0), &Vector(1.0, 0.0, 0.0), &Vector(0.0, 1.0, 0.0)); // z=0
        let plain2 = Plain::from_three_points(& Vector(0.0,0.0,1.0), &Vector(1.0, 0.0, 1.0), &Vector(1.0, 1.0, 1.0)); // z=1
        assert_eq!(Plain::distance_between(&plain1, &plain2), 1.0);
    }

    #[test]
    fn angles_with_vector_and_line() {
        const EPSILON: f64 = 0.0001;
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 1.0, 0.0), Vector(1.0, 0.0, 0.0)); // the y axis
        let plain = Plain::from_two_lines(&line1, &line2);
        let line3 = Line::new(Vector(0.0, 0.0, 0.0), Vector(-1.0, 0.0, 1.0));
        assert!(plain.angle_with_vector(&line3.direction) - PI / 4.0 < EPSILON);
        assert!(plain.angle_with_line(&line3) - PI / 4.0 < EPSILON);
    }

    #[test]
    fn angle_between_plains() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 0.0, 0.0), Vector(0.0, 1.0, 0.0)); // the y axis
        let line3 = Line::new(Vector(0.0, 0.0, 0.0), Vector(0.0, 0.0, 1.0)); // the z axis
        let plain1 = Plain::from_two_lines(&line1, &line2);
        let plain2 = Plain::from_two_lines(&line1, &line3);
        assert_eq!(Plain::angle_between(&plain1, &plain2), PI / 2.0)
    }
}
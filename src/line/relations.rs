//! A module to describe a relation between two lines
use crate::vector::Vector;
use crate::line::Line;

pub enum LineRelations {
    /// The two lines share the exact same points
    Unite,
    /// The two lines have a constant distance between them
    Parallel(f64),
    /// The two lines share a point and have an angle between them
    Colliding(Vector, f64),
    /// The two lines have no common plane. They have an angle and a distance
    Intersect(f64, f64)
}

impl LineRelations {
    pub fn of(line1: Line, line2: Line) -> LineRelations {
        if line1.direction.is_lindep(line2.direction) {
            // lines either unite or parallel
            if line1.is_on_line(line2.point) { Self::Unite }
            else { Self::Parallel(0.0) }
        }
        else {
            let angle = Vector::angle_between(line1.direction, line2.direction);
            // find a point that is on both lines
            Self::Intersect(0.0, angle)
        }
    }
}
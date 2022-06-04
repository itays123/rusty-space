//! A module to describe a relation between two lines
use crate::vector::Vector;
use crate::line::Line;

#[derive(Debug, PartialEq)]
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
    pub fn of(line1: &Line, line2: &Line) -> LineRelations {
        if line1.direction.is_lindep(line2.direction) {
            // lines either unite or parallel
            let distance = line1.distance_from_point(line2.point);
            if distance == 0.0 { Self::Unite } else { Self::Parallel(distance) }
        }
        else {
            let angle = Vector::angle_between(line1.direction, line2.direction);
            // find a point that is on both lines
            Self::Intersect(0.0, angle)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_line() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0));
        let line2 = Line::new(Vector(0.0, 0.0, 0.0), Vector(2.0, 0.0, 0.0));
        assert_eq!(LineRelations::of(&line1, &line2), LineRelations::Unite)
    }

    #[test]
    fn parallel_lines() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0));
        let line2 = Line::new(Vector(0.0, 1.0, 0.0), Vector(1.0, 0.0, 0.0));
        assert_eq!(LineRelations::of(&line1, &line2), LineRelations::Parallel(1.0))
    }
}
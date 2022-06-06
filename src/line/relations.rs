//! A module to describe a relation between two lines

use crate::plain::Plain;
use crate::vector::Vector;
use crate::line::Line;

#[derive(Debug, PartialEq)]
pub enum LineRelations {
    /// The two lines share the exact same points
    Unite,
    /// The two lines have a constant distance between them
    Parallel(f64),
    /// The two lines share a point and have an angle between them
    Intersect(Vector, f64),
    /// The two lines have no common plane. They have an angle and a distance
    Foreign(f64, f64)
}

impl LineRelations {
    /// find the relation between two lines
    pub fn of(line1: &Line, line2: &Line) -> LineRelations {
        if line1.direction.is_lindep(&line2.direction) {
            // lines either unite or parallel
            let distance = line1.distance_from_point(&line2.point);
            if distance == 0.0 { Self::Unite } else { Self::Parallel(distance) }
        }
        else {
            // lines either collide or intersect
            let angle = Line::angle_between(line1, line2);

            if let Some(intersection) = Line::intersection(line1, line2) {
                // found a point that is on both lines
                Self::Intersect(intersection, angle)
            } else {
                // lines are foreign. Calculate the distance between them
                // Create a plain with the origin of the first line and the directions of the two
                let common_plain = Plain::new(&line1.point, &line1.direction, &line2.direction);
                let distance = common_plain.distance_from(&line2.point);
                Self::Foreign(distance, angle)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

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

    #[test]
    fn intersecting_lines() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)); // the x axis
        let line2 = Line::new(Vector(0.0, 0.0, 0.0), Vector(-1.0, 1.0, 0.0));
        assert_eq!(LineRelations::of(&line1, &line2), LineRelations::Intersect(Vector(0.0, 0.0, 0.0), PI / 4.0));
    }

    #[test]
    fn foreign_lines() {
        let line1 = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0));
        let line2 = Line::new(Vector(0.0, 1.0, 0.0), Vector(0.0, 0.0, 1.0));
        assert_eq!(LineRelations::of(&line1, &line2), LineRelations::Foreign(1.0, PI / 2.0))
    }
}
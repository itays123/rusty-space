//! Represents a relation between two planes
use crate::{line::Line, vector::Vector};

use super::Plain;

#[derive(Debug, PartialEq)]
pub enum PlainRelations {
    /// The two planes share the exact same points
    Unite,
    /// The two planes have a constant distance between them
    Parallel(f64),
    /// The two planes share an intersection line and have an angle between them
    Intersect(Line, f64),
}

impl PlainRelations {
    pub fn of(plain1: &Plain, plain2: &Plain) -> Self {
        if !plain1.plumb.is_lindep(&plain2.plumb) {
            // planes intersect.
            let angle = Plain::angle_between(plain1, plain2);
            let intersection = Line::new(Vector(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0));
            return Self::Intersect(intersection, angle);
        }

        if plain1.constant_d == plain2.constant_d { Self::Unite } else { Self::Parallel(Plain::distance_between(plain1, plain2)) }
    }
}
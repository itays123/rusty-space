//! Calculate intersection lines

use crate::{vector::Vector, math::dependence::{SingleScalarDependence, Dimension}, line::Line};

use super::Plain;

pub fn intersection(p1: &Plain, p2: &Plain) -> Line {
    let Vector(a, b, c) = p1.plumb;
    let Vector(m, n, k) = p2.plumb;
    // we have two equations - ax + by + cz + d1 = 0, mx + ny + kz + d2 = 0
    let dep1 = SingleScalarDependence::compute_from((a, b, c, p1.constant_d), (m, n, k, p2.constant_d));
    // could conflict!
    let dep2 = dep1.substitute_in(a, b, c, p1.constant_d)
        .unwrap_or_else(|| dep1.substitute_in(m, n, k, p2.constant_d).unwrap()); 

    let point1 = SingleScalarDependence::put_multiple(&dep1, &dep2, 0.0);
    let point2 = SingleScalarDependence::put_multiple(&dep1, &dep2, 1.0);
    Line::from_two_points(point1, &point2)
}
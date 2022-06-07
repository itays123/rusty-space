use crate::vector::Vector;

#[derive(PartialEq, Debug)]
pub enum PlainLineRelations {
    /// Plane contains the line
    Containing,
    /// Line has one intersection with the plane, at a certain angle
    Intersect(Vector, f64),
    /// Line is parallel to the plain in a given distance
    Parallel(f64)
}
//! Custom utilities to calculate intersection lines of planes

use std::collections::HashMap;

use crate::vector::Vector;

/// Represents a dimension: Either x, y, or z
#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
pub enum Dimension {
    X,
    Y,
    Z,
    /// No dimension, used by the zero_dims function
    None
}

use self::Dimension::{X, Y, Z};

/// Compute zero dims easily for match expressions that don't allow floats
fn zero_dims(x_coefficient: f64, y_coefficient: f64, z_coefficient: f64) -> (Dimension, Dimension, Dimension) {
    let dim1 = if x_coefficient == 0.0 { X } else { Dimension::None };
    let dim2 = if y_coefficient == 0.0 { Y } else { Dimension::None };
    let dim3 = if z_coefficient == 0.0 { Z } else { Dimension::None };
    (dim1, dim2, dim3)
}

/// Represents a dependence such as y = x + 3
/// This dependence will be represented as { target: Y, source: X, source_scalar: 1.0, constant: 3.0 }
/// The dependence z = 4 will be represented as { target: Z, source: None, source_scalar: 0.0, constant: 4.0 }
#[derive(PartialEq, Debug)]
pub struct SingleScalarDependence {
    pub target: Dimension,
    pub source: Dimension,
    source_scalar: f64,
    constant: f64
}

impl SingleScalarDependence {
    /// Private constructor
    fn new(target: Dimension, source: Dimension, source_scalar: f64, constant: f64) -> Self {
        Self { target, source, source_scalar, constant }
    }

    /// Private constructor: scalar only, i.e: z = 4
    fn scalar_only(target: Dimension, coefficient: f64, constant: f64) -> Self {
        // ax + d = 0 => x = -d/a
        Self::new(target, Dimension::None, 1.0, -constant / coefficient)
    }

    /// Private constructor: single dependence, i.e: y = x - 3
    fn from_coefficients(target: Dimension, source: Dimension, target_coefficient: f64, source_coefficient: f64, constant: f64) -> Self {
        // by + cz + d = 0
        // cz = -by - d
        // z = -b/c*y -d/c
        let source_scalar = -source_coefficient / target_coefficient;
        let constant = -constant / target_coefficient;
        Self::new(target, source, source_scalar, constant)
    }


    /// Compute a new SingleScalarDependency.
    /// Returns a result of self, and fails if can't form a dependency from the values provided
    /// # Panics:
    /// - All 3 coefficients are 0
    pub fn compute(x_coefficient: f64, y_coefficient: f64, z_coefficient: f64, constant: f64) -> Result<Self, ()> {
        match zero_dims(x_coefficient, y_coefficient, z_coefficient) {
            (X, Y, Z) => panic!("All 3 coefficients are 0"),
            (X, Y, _) => Ok(Self::scalar_only(Z, z_coefficient, constant)),
            (X, _, Z) => Ok(Self::scalar_only(Y, y_coefficient, constant)),
            (_, Y, Z) => Ok(Self::scalar_only(X, x_coefficient, constant)),
            (X, _, _) => Ok(Self::from_coefficients(Z, Y, z_coefficient, y_coefficient, constant)),
            (_, Y, _) => Ok(Self::from_coefficients(Z, X, z_coefficient, x_coefficient, constant)),
            (_, _, Z) => Ok(Self::from_coefficients(Y, X, y_coefficient, x_coefficient, constant)),
            (_, _, _) => Err(()) // cannot form a single dependence when don't have a single 
        }
    }

    ///  Compute another single dependency from a different equation in the form of ax + by + cz + d = 0
    /// # None-returns
    ///  - If equation passed conflicts with the current dependency, for instance, z=3 and z=2
    /// # Panics:
    /// - If all 3 coefficients are 0
    pub fn substitute_in(&self, x_coefficient: f64, y_coefficient: f64, z_coefficient: f64, constant: f64)-> Option<Self> {
        // try to compute an equation from scratch without the need for this one
        let result = SingleScalarDependence::compute(x_coefficient, y_coefficient, z_coefficient, constant)
            .unwrap_or_else(|_| {
            self.subsitute(x_coefficient, y_coefficient, z_coefficient, constant)
        });

        if result.source == self.source && result.target == self.target {
            return None;
        }

        Some(result)

    }

    /// Calculate the substitute source coefficient as specified below
    fn calc_substituted_values(&self, source_coefficient: f64, target_coefficient: f64, constant: f64) -> (f64, f64) {
        let substituted_source_coefficient = source_coefficient + self.source_scalar * target_coefficient;
        let substituted_scalar = constant + self.constant * target_coefficient;
        (substituted_source_coefficient, substituted_scalar)
    }

    /// Substitute this dependency in the equation given.
    fn subsitute(&self, x_coefficient: f64, y_coefficient: f64, z_coefficient: f64, constant: f64) -> Self {
        // Assume source=x, target=z: z = mx + n therefore for the equation ax + by + cx + d = 0
        // ax + by + c(mx + n) + d = 0
        // (a + mc)x + by + cn + d = 0
        match (self.source, self.target) {
            (X, Z) => {
                let (source_coefficient, constant) = self.calc_substituted_values(x_coefficient, z_coefficient, constant);
                Self::from_coefficients(Y, X, y_coefficient, source_coefficient, constant)
            },
            (X, Y) => {
                let (source_coefficient, constant) = self.calc_substituted_values(x_coefficient, y_coefficient, constant);
                Self::from_coefficients(Z, X, z_coefficient, source_coefficient, constant)
            },
            (Y, Z) => {
                let (source_coefficient, constant) = self.calc_substituted_values(y_coefficient, z_coefficient, constant);
                Self::from_coefficients(X, Y, x_coefficient, source_coefficient, constant)
            },
            // Special case: current dependence is source=scalar
            // Assume target = z, therefore equation is z=n, and substituted eqation is ax + by + cn + d = 0
            (Dimension::None, X) => {
                let (_, constant) = self.calc_substituted_values(1.0, x_coefficient, constant);
                Self::from_coefficients(Z, Y, z_coefficient, y_coefficient, constant)
            },
            (Dimension::None, Y) => {
                let (_, constant) = self.calc_substituted_values(1.0, y_coefficient, constant);
                Self::from_coefficients(Z, X, z_coefficient, y_coefficient, constant)
            },
            (Dimension::None, Z) => {
                let (_, constant) = self.calc_substituted_values(1.0, z_coefficient, constant);
                Self::from_coefficients(Y, X, y_coefficient, x_coefficient, constant)
            }
            (_, _) => panic!("These are the only options provided! This should never panic!")
        }
    }

    /// Generate a new dependency from two equations in the form of ax + by + cz + d = 0
    /// # Panics:
    /// - Equations conflict / merge
    pub fn compute_from(eq1: (f64, f64, f64, f64), eq2: (f64, f64, f64, f64)) -> Self {
        if Vector(eq1.0, eq1.1, eq1.2).is_lindep(&Vector(eq2.0, eq2.1, eq2.2)) {
            // either 0 or infinite solutions
            panic!("Equations cannot form a single dependency");
        }

        Self::compute(eq1.0, eq1.1, eq1.2, eq1.3)
            .unwrap_or_else(|_| { Self::compute(eq2.0, eq2.1, eq2.2, eq2.3)
            .unwrap_or_else(|_| { 
                Self::compute_from_full_equations(eq1, eq2)
             }) })
    }

    /// Generate a new dependency from equations where all coefficients are not 0
    fn compute_from_full_equations(eq1: (f64, f64, f64, f64), eq2: (f64, f64, f64, f64)) -> Self {
        // first equation: ax + by + cz + d1 = 0
        let (a, b, c, d1) = eq1;
        // second equation: mx + ny + kz + d2 = 0
        let (m, n, k, d2) = eq2; // multiply by a/m
        let multiplier = a / m;
        let (n, k, d2) = (n * multiplier, k * multiplier, d2 * multiplier);
        // equation difference:
        // (b-n')y + (c-k')z + d1 - d2' = 0
        let (y_coefficient, z_coefficient, constant) = (b - n, c-k, d1-d2);
        Self::from_coefficients(Z, Y, z_coefficient, y_coefficient, constant)

    }

    /// Put the specified value as the value of the source, and compute the result
    pub fn put(&self, value: f64) -> f64 {
        if self.source == Dimension::None {
            self.constant
        }
        else {
            self.source_scalar * value + self.constant
        }
    }

    /// Put the values of two dependencies into a point
    /// # Panics:
    /// - The two dependencies have a different source
    /// - The two dependencies have the same target
    pub fn put_multiple(dep1: &Self, dep2: &Self, value: f64) -> Vector {
        let source = match (dep1.source, dep2.source) {
            (Dimension::None, dim) | (dim, Dimension::None) => dim,
            (dim1, dim2) if dim1 == dim2 => dim1,
            (_, _) => panic!("The two dependencies have different sources")
        };

        let (value1, value2) = (dep1.put(value), dep2.put(value));
        let mut value_map: HashMap<Dimension, f64> = HashMap::new();
        value_map.insert(source, value);
        value_map.insert(dep1.source, value1);
        value_map.insert(dep2.source, value2);

        if value_map.len() != 3 {
            panic!("Could not assemble a point");
        }

        let x = value_map.get(&X).unwrap_or_else(|| value_map.get(&Dimension::None).unwrap());
        let y = value_map.get(&Y).unwrap_or_else(|| value_map.get(&Dimension::None).unwrap());
        let z = value_map.get(&Z).unwrap_or_else(|| value_map.get(&Dimension::None).unwrap());

        Vector(*x, *y, *z)

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_equations_two_zeros() {
        // -x + 3 = 0 => x = 3
        assert_eq!(SingleScalarDependence::compute(-1.0, 0.0, 0.0, 3.0).unwrap(), SingleScalarDependence::new(X, Dimension::None, 1.0, 3.0));
        assert_eq!(SingleScalarDependence::compute(0.0, -1.0, 0.0, 3.0).unwrap(), SingleScalarDependence::new(Y, Dimension::None, 1.0, 3.0));
        assert_eq!(SingleScalarDependence::compute(0.0, 0.0, -1.0, 3.0).unwrap(), SingleScalarDependence::new(Z, Dimension::None, 1.0, 3.0));
    }

    #[test]
    fn linear_dependence() {
        // -x + y + 3 = 0 => y = x - 3
        assert_eq!(SingleScalarDependence::compute(-1.0, 1.0, 0.0, 3.0).unwrap(), SingleScalarDependence::new(Y, X, 1.0, -3.0));
        // -x + z + 3 = 0
        assert_eq!(SingleScalarDependence::compute(-1.0, 0.0, 1.0, 3.0).unwrap(), SingleScalarDependence::new(Z, X, 1.0, -3.0));
        // -y + z + 3 = 0
        assert_eq!(SingleScalarDependence::compute(0.0, -1.0, 1.0, 3.0).unwrap(), SingleScalarDependence::new(Z, Y, 1.0, -3.0));
    }

    #[test]
    fn two_linear_dependencies() {
        // y = x - 3
        let eq = SingleScalarDependence::new(Y, X, 1.0, -3.0);
        // z = 2x + 8
        let result_expected = SingleScalarDependence::new(Z, X, 2.0, 8.0);
        assert_eq!(eq.substitute_in(2.0, 0.0, -1.0, 8.0).unwrap(), result_expected)
    }

    #[test]
    fn substitute_dependencies() {
        // y = x - 3
        let eq = SingleScalarDependence::new(Y, X, 1.0, -3.0);
        // z = 2x + 8
        // Matching equation:
        // x + y - z = x + (x-3) - (2x+8) = 2x - 3 - 2x - 8 = -11
        // x + y - z + 11 = 0
        let result_expected = SingleScalarDependence::new(Z, X, 2.0, 8.0);
        assert_eq!(eq.substitute_in(1.0, 1.0, -1.0, 11.0).unwrap(), result_expected)
    }

    #[test]
    fn full_substitution() {
        // z = y - 3
        let expected_dependence1 = SingleScalarDependence::new(Z, Y, 1.0, -3.0);
        // x = 2y + 8
        let expected_dependence2 = SingleScalarDependence::new(X, Y, 2.0, 8.0);
        // Matching equation:
        // x - y - z = 2y + 8 -y - (y-3) = 2y + 8 - y - y + 3 = 11
        // x - y - z - 11 = 0
        // And another one:
        // 2x - 3y - z = 4y + 16 - 3y - y + 3 = 19
        // 2x - 3y - z - 19 = 0
        let eq1 = (1.0, -1.0, -1.0, -11.0);
        let eq2 = (2.0, -3.0, -1.0, -19.0);
        let first_dependence = SingleScalarDependence::compute_from(eq1, eq2);
        assert_eq!(first_dependence, expected_dependence1);
        let second_dependence = first_dependence.substitute_in(eq2.0, eq2.1, eq2.2, eq2.3).unwrap();
        assert_eq!(second_dependence, expected_dependence2);
    }
}
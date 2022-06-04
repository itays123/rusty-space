//! A module for solving linear equations

pub enum EquationSolution {
    /// No solution (0x = -b, b != 0)
    None,
    /// Undefined solution (0x = 0)
    Undefined,
    /// A Real solution
    Real(f64)
}

impl EquationSolution {
    /// Solve the equation `ax + b = 0`
    pub fn compute(a: f64, b: f64) -> Self {
        if a == 0.0 {
            if b == 0.0 { Self::Undefined } else { Self::None }
        } else {
            Self::Real(-1.0 * b / a)
        }
    }
}

impl PartialEq<EquationSolution> for EquationSolution {
    
    fn eq(&self, other: &EquationSolution) -> bool { 
        match (self, other) {
            (Self::None, _) | (_, Self::None) => false,
            (Self::Undefined, _) | (_, Self::Undefined) => true,
            (Self::Real(a), Self::Real(b)) => a == b 
        }
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_solution() {
        match EquationSolution::compute(0.0, 5.0) {
            EquationSolution::None => (),
            _ => panic!("Should not find solution")
        }
    }

    
    #[test]
    fn undefined_solution() {
        match EquationSolution::compute(0.0, 0.0) {
            EquationSolution::Undefined => (),
            _ => panic!("Should not find solution")
        }
    }

    
    #[test]
    fn real_solution() {
        match EquationSolution::compute(1.0, 2.0) {
            EquationSolution::Real(a) => assert_eq!(a, -2.0),
            _ => panic!("Should find solution - 2.0")
        }
    }
}
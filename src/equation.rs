//! A module for solving linear equations

use crate::vector::Vector;

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

    /// compute the x solution to an equation system, given the y solution
    fn compute_other_solution(eq: (f64, f64, f64), y: f64) -> Option<(f64, f64)> {
        // we have a solution for y, compute solution for x
        if let EquationSolution::Real(x) = EquationSolution::compute(eq.0, eq.1 * y + eq.2) {
            Some((x, y))
        } else {
            None
        }
    }

    /// solve the equation system: `ax + by + c = 0`, `mx + ny + k = 0`
    /// return an optional tuple of solutions, if found.
    pub fn compute_multiple(eq1: (f64, f64, f64), eq2: (f64, f64, f64)) -> Option<(f64, f64)> {
        // given ax + by + c = 0,
        //       mx + ny + k = 0  - Multiply this by a/m
        // we now have:
        // ax + by + c = 0
        // ax + (a*n/m)y + (a*k/m) = 0
        // this equation is solveable for b
        // special case: a = 0 or m = 0, solve for y immediately
        let y_coefficient: f64;
        let constant: f64;
        let mut eq_compute_later = eq1;
        if eq1.0 == 0.0 {
            y_coefficient = eq1.1;
            eq_compute_later = eq2;
            constant = eq1.2;
        } else if eq2.0 == 0.0 {
            y_coefficient = eq2.1;
            constant = eq2.2;
        } else {
            // subtract two equations
            let multiplier = eq1.0 / eq2.0;
            let eq1 = Vector(eq1.0, eq1.1, eq1.2);
            let eq2 = multiplier * Vector(eq2.0, eq2.1, eq2.2);
            let difference = eq1 - eq2;
            y_coefficient = difference.1;
            constant = difference.2;
        }
        
        if let EquationSolution::Real(y) = EquationSolution::compute(y_coefficient, constant) {
            // we have a solution for y, compute solution for x
            EquationSolution::compute_other_solution(eq_compute_later, y)
        } else {
            None
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

    #[test]
    fn parallel_equations() {
        let eq1 = (1.0, 2.0, 0.0);
        let eq2 = (1.0, 2.0, 3.0);
        if let Some((_, _)) = EquationSolution::compute_multiple(eq1, eq2) {
            panic!("Equation should have no solutions")
        }
    }

    #[test]
    fn same_equations() {
        let eq1 = (1.0, 2.0, 3.0);
        let eq2 = (1.0, 2.0, 3.0);
        if let Some((_, _)) = EquationSolution::compute_multiple(eq1, eq2) {
            panic!("Equation should have no solutions")
        }
    }

    #[test]
    fn single_solution_equations() {
        let eq1 = (1.0, 1.0, -2.0);
        let eq2 = (1.0, 2.0, -3.0);
        assert_eq!(EquationSolution::compute_multiple(eq1, eq2).unwrap(), (1.0, 1.0))
    }

    #[test]
    fn y_solution_immediate() {
        let eq1 = (0.0, 1.0, -2.0); // y=2
        let eq2 = (1.0, -1.0, 1.0);// x - 2 + 1 = 0 => x = 1
        assert_eq!(EquationSolution::compute_multiple(eq1, eq2).unwrap(), (1.0, 2.0))
    }
}
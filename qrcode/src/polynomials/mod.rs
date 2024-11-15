use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul, Sub};

/// structure that represents an indetermiante as coefficient*x^degree
#[derive(Clone, Copy, PartialEq, Debug)]
struct Indeterminate {
    /// factor in front pof the indeterminate
    coefficient: i16,
    /// the 'to the power of' factor
    degree: u16,
}

impl Indeterminate {
    fn new(coefficient: i16, degree: u16) -> Indeterminate {
        Indeterminate {
            coefficient,
            degree,
        }
    }
}

impl Display for Indeterminate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // write the individual indeterminate
        match self.degree {
            0 => write!(f, "{}", self.coefficient)?,
            1 => write!(f, "{}x", self.coefficient)?,
            _ => write!(f, "{}x^{}", self.coefficient, self.degree)?,
        }
        write!(f, "{}", "")
    }
}

impl Mul for Indeterminate {
    type Output = Indeterminate;
    fn mul(self, rhs: Self) -> Self::Output {
        Indeterminate::new(self.coefficient * rhs.coefficient, self.degree * rhs.degree)
    }
}

/// combines the indeterminates together into a polynomial for more
/// advanced maths operations
#[derive(Clone, PartialEq, Debug)]
struct Polynomial {
    function: Vec<Indeterminate>,
}

impl Polynomial {
    fn new(function: Vec<Indeterminate>) -> Polynomial {
        Polynomial { function }
    }

    /// method to reduce
    fn reduce(&mut self) -> Polynomial {
        // vector to collect all degrees
        let mut degrees: Vec<u16> = vec![];
        for x in self.function.iter() {
            // save local degree
            let current_degree: u16 = x.degree;
            // check if current degree is already in degree vector
            // if not: put it in
            let mut is_in_already: bool = false;
            for y in degrees.iter() {
                if current_degree == *y {
                    is_in_already = true;
                }
            }
            if !is_in_already {
                degrees.push(current_degree);
            }
        }
        // take all x'es with the same degrees add their coefficients together
        // and save them in a new vector
        let mut result: Vec<Indeterminate> = vec![];
        for degree in degrees.iter() {
            let mut degree_coefficient: i16 = 0;
            for z in self.function.iter() {
                if *degree == z.degree {
                    degree_coefficient += z.coefficient;
                }
            }
            result.push(Indeterminate::new(degree_coefficient, *degree));
        }
        self.function = result.clone();
        Polynomial { function: result }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut is_first_element: bool = true;
        for indeterminate in self.function.iter() {
            // print the + in front of x
            // - get printed automatically
            if indeterminate.coefficient >= 0 && !is_first_element {
                write!(f, "{}", "+")?;
            }
            // write the individual indeterminate
            match indeterminate.degree {
                0 => write!(f, "{}", indeterminate.coefficient)?,
                1 => write!(f, "{}x", indeterminate.coefficient)?,
                _ => write!(f, "{}x^{}", indeterminate.coefficient, indeterminate.degree)?,
            }
            is_first_element = false;
        }
        // end
        write!(f, "{}", "")
    }
}

impl Add for Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Self) -> Self::Output {
        // get current values
        let mut help_vector: Vec<Indeterminate> = self.function;
        // add indeterminates from right hand side rhs
        // adding them to the polynomial
        for indeterminate in rhs.function.iter() {
            help_vector.push(*indeterminate);
        }
        // return the result
        // don't forget to .reduce(), which is the "adding" part of this function
        Polynomial {
            function: help_vector,
        }
        .reduce()
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: Self) -> Self::Output {
        // same as add, but the coefficient gets negated
        // before it is added, thus subtracting it
        // get current values
        let mut help_vector: Vec<Indeterminate> = self.function;
        // add indeterminates from right hand side rhs
        // adding them to the polynomial
        for indeterminate in rhs.function.iter() {
            let mut intermediate_help: Indeterminate = *indeterminate;
            intermediate_help.coefficient = indeterminate.coefficient * -1;
            help_vector.push(intermediate_help);
        }
        // return the result
        // don't forget to .reduce(), which is the "adding" part of this function
        Polynomial {
            function: help_vector,
        }
        .reduce()
    }
}

#[test]
pub fn polynomials_math() {
    // simple test polynomial
    let test_pol: Polynomial = Polynomial::new(vec![
        Indeterminate::new(1, 2),
        Indeterminate::new(2, 1),
        Indeterminate::new(-4, 0),
    ]);
    println!("{}", test_pol);
    let reduce_test1: Polynomial =
        Polynomial::new(vec![Indeterminate::new(-3, 4), Indeterminate::new(5, 4)]);
    println!(
        "{} simplified to {}",
        reduce_test1,
        reduce_test1.clone().reduce()
    );
    assert_eq!(
        reduce_test1.clone().reduce(),
        Polynomial::new(vec![Indeterminate::new(2, 4)])
    );
    let add1: Polynomial =
        Polynomial::new(vec![Indeterminate::new(4, 3), Indeterminate::new(2, 2)]);
    let add2: Polynomial = Polynomial::new(vec![Indeterminate::new(-2, 3)]);
    let result_add: Polynomial = add1.clone() + add2.clone();
    println!("{} plus {} equals {}", add1, add2, result_add);
    assert_eq!(
        result_add,
        Polynomial::new(vec![Indeterminate::new(2, 3), Indeterminate::new(2, 2)])
    );
    println!(
        "{} minus {} equals {}",
        add1,
        add2,
        add1.clone() - add2.clone()
    );
    assert_eq!(
        add1.clone() - add2.clone(),
        Polynomial::new(vec![Indeterminate::new(6, 3), Indeterminate::new(2, 2)])
    );
    let inde1: Indeterminate = Indeterminate::new(3, 2);
    let inde2: Indeterminate = Indeterminate::new(-1, 6);
    let result2: Indeterminate = inde1.clone() * inde2.clone();
    println!("{} multiplied with {} equals {} ", inde1, inde2, result2);
    assert_eq!(result2, Indeterminate::new(-3, 12));
}

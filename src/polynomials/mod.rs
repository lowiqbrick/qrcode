use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul};
use std::vec;

/// structure that represents an indetermiante as coefficient*x^degree
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Indeterminate {
    /// factor in front pof the indeterminate
    coefficient: u8,
    /// the 'to the power of' factor
    degree: i16,
}

impl Indeterminate {
    pub fn new(coefficient: u8, degree: i16) -> Indeterminate {
        Indeterminate {
            coefficient,
            degree,
        }
    }

    pub fn get_coefficient(&self) -> u8 {
        self.coefficient
    }

    pub fn set_coefficient(&mut self, new_coefficient: u8) {
        self.coefficient = new_coefficient;
    }

    pub fn get_degree(&self) -> i16 {
        self.degree
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
        write!(f, "")
    }
}

impl Mul for Indeterminate {
    type Output = Indeterminate;
    fn mul(self, rhs: Self) -> Self::Output {
        Indeterminate::new(
            ((self.coefficient as i16 * rhs.coefficient as i16) & 0x00_00_FF_FF_u16 as i16) as u8,
            self.degree + rhs.degree,
        )
    }
}

/// combines the indeterminates together into a polynomial for more
/// advanced maths operations
#[derive(Clone, PartialEq, Debug)]
pub struct Polynomial {
    function: Vec<Indeterminate>,
}

impl Polynomial {
    pub fn new(function: Vec<Indeterminate>) -> Polynomial {
        let mut mutable_function: Vec<Indeterminate> = function;
        // sort the polynomial
        mutable_function.sort_by_key(|indet_struct| indet_struct.degree);
        // create new reduced polynomial
        Polynomial {
            function: mutable_function,
        }
        .reduce()
    }

    /// method to reduce/make the polynomial look good
    /// (indeterminates get sorted from highest power degree to lowest)
    ///
    ///  # Bug
    ///
    /// This method updates the struct it is called on correctly and SHOULD return
    /// the exact value the struct was changed to, but doesn't. The function removes
    /// indeterminates with a coefficient that is zero, but returns the
    /// version where the zero coefficients are still present. Changing this
    /// causes an assertion error later on.
    pub fn reduce(&mut self) -> Polynomial {
        // vector to collect all degrees
        let mut degrees: Vec<i16> = vec![];
        for x in self.function.iter() {
            // save local degree
            let current_degree: i16 = x.degree;
            // check if current degree is already in degree vector
            // if not: put it in
            let mut is_in_already: bool = false;
            for y in degrees.iter() {
                if current_degree == (*y) {
                    is_in_already = true;
                }
            }
            if !is_in_already {
                degrees.push(current_degree);
            }
        }
        // sort the elements from tallest degree to smallest
        degrees.sort();
        degrees.reverse();
        // take all x'es with the same degrees add their coefficients together
        // and save them in a new vector
        let mut result: Vec<Indeterminate> = vec![];
        for degree in degrees.iter() {
            let mut degree_coefficient: u8 = 0;
            for z in self.function.iter() {
                if *degree == z.degree {
                    degree_coefficient = ((degree_coefficient as i16 + z.coefficient as i16)
                        & 0x00_00_FF_FF_u16 as i16) as u8;
                }
            }
            result.push(Indeterminate::new(degree_coefficient, *degree));
        }
        // delete terms that have a coefficient equal to zero
        let mut catch_zero_coefficients: Vec<Indeterminate> = vec![];
        for indeter in result.iter() {
            if indeter.coefficient == 0 {
                continue;
            } else {
                catch_zero_coefficients.push(*indeter);
            }
        }
        self.function = catch_zero_coefficients.clone();
        Polynomial { function: result }
    }

    pub fn push(&mut self, element: Indeterminate) {
        self.function.push(element);
    }

    pub fn get_function(&self) -> Vec<Indeterminate> {
        self.function.clone()
    }

    pub fn get_function_mut(&mut self) -> &mut Vec<Indeterminate> {
        &mut self.function
    }

    pub fn remove_highest_degree(&self) -> Polynomial {
        assert!(!self.function.is_empty());
        let mut work_polynomial = self.clone();
        // get highest degree
        let mut highest_degree = 0;
        for indeterminate in work_polynomial.function.iter() {
            if indeterminate.degree > highest_degree {
                highest_degree = indeterminate.degree;
            }
        }
        // remove the highest degree
        for indeterminate in work_polynomial.function.iter_mut() {
            if indeterminate.degree == highest_degree {
                indeterminate.coefficient = 0;
            }
        }
        // delete the coefficient that is zero
        work_polynomial.reduce();
        work_polynomial
    }

    pub fn galois_mul_x1(&self, mod_fx: Polynomial, m: u8) -> Polynomial {
        let x1 = Polynomial::new(vec![Indeterminate::new(1, 1)]);
        let mut result = self.clone() * x1;
        let mut is_overflowing = false;
        // remove
        for indeterminate in result.get_function_mut().iter_mut() {
            if indeterminate.get_degree() == m.into() {
                indeterminate.coefficient = 0;
                is_overflowing = true;
            }
        }
        result.reduce();
        if is_overflowing {
            result = result + mod_fx;
        }
        result.reduce();
        for indeterminate in result.get_function_mut().iter_mut() {
            if indeterminate.coefficient == 2 {
                indeterminate.coefficient = 0;
            }
        }
        result.reduce();
        result
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut is_first_element: bool = true;
        for indeterminate in self.function.iter() {
            // print the + in front of x
            // - get printed automatically
            if !is_first_element {
                write!(f, "+")?;
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
        write!(f, "")
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

impl IntoIterator for Polynomial {
    type Item = Indeterminate;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.function.into_iter()
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Self) -> Self::Output {
        // make shure that something is in the vector
        assert!(!rhs.function.is_empty());
        // collect all results
        let mut polynomial_collector: Vec<Polynomial> = vec![];
        // multiply an indeterminate with an polynomial
        for indeterminate_left in self.clone() {
            let mut help_result: Polynomial = Polynomial::new(vec![]);
            // multiply the current indeterminate
            for indeterminate_right in rhs.clone() {
                help_result
                    .function
                    .push(indeterminate_left * indeterminate_right);
            }
            // add this result to the polynomial collector
            polynomial_collector.push(help_result.clone());
        }
        // add all the terms together
        let mut result_var: Polynomial = Polynomial::new(vec![]);
        for polynomial_res in polynomial_collector {
            if result_var.function.is_empty() {
                result_var.function = polynomial_res.function;
            } else {
                result_var = result_var + polynomial_res;
            }
        }
        result_var.reduce()
    }
}

impl From<Vec<u8>> for Polynomial {
    fn from(item: Vec<u8>) -> Polynomial {
        let mut degree: usize = item.len();
        let mut result: Polynomial = Polynomial::new(vec![]);
        for element in item.iter() {
            degree -= 1;
            result
                .function
                .push(Indeterminate::new(*element, (degree) as i16));
        }
        result
    }
}

impl From<Polynomial> for Vec<u8> {
    fn from(item: Polynomial) -> Vec<u8> {
        let mut return_vec: Vec<u8> = vec![];
        let item_copy: Polynomial = item.clone().reduce();
        for var in item_copy.function.iter() {
            return_vec.push(var.coefficient);
        }
        return_vec
    }
}

mod tests {

    #[test]
    pub fn mul_ind() {
        use super::Indeterminate;

        let inde1: Indeterminate = Indeterminate::new(3, 2);
        let inde2: Indeterminate = Indeterminate::new(5, 6);
        let result2: Indeterminate = inde1 * inde2;
        println!("{} multiplied with {} equals {} ", inde1, inde2, result2);
        assert_eq!(result2, Indeterminate::new(15, 8));
    }

    #[test]
    pub fn reduce_poly() {
        use super::{Indeterminate, Polynomial};

        // simple test polynomial
        let reduce_test1: Polynomial =
            Polynomial::new(vec![Indeterminate::new(6, 4), Indeterminate::new(5, 4)]);
        println!(
            "{} simplified to {}",
            reduce_test1,
            reduce_test1.clone().reduce()
        );
        assert_eq!(
            reduce_test1.clone().reduce(),
            Polynomial::new(vec![Indeterminate::new(11, 4)])
        );
        // make shure coefficients of value zero get deleted
        let mut test_pol_zero: Polynomial = Polynomial::new(vec![
            Indeterminate::new(0, 3),
            Indeterminate::new(0, 1),
            Indeterminate::new(14, 0),
            // also check sorting while being at it
            Indeterminate::new(128, 2),
        ]);
        let expected_result: Polynomial =
            Polynomial::new(vec![Indeterminate::new(128, 2), Indeterminate::new(14, 0)]);
        println!(
            "{} should get reduced to {}",
            test_pol_zero.reduce(),
            expected_result
        );
        assert_eq!(test_pol_zero.reduce(), expected_result);
    }

    #[test]
    pub fn add_poly() {
        use super::{Indeterminate, Polynomial};

        let add1: Polynomial =
            Polynomial::new(vec![Indeterminate::new(4, 3), Indeterminate::new(2, 2)]);
        let add2: Polynomial = Polynomial::new(vec![Indeterminate::new(65, 3)]);
        let result_add: Polynomial = add1.clone() + add2.clone();
        println!("{} plus {} equals {}", add1, add2, result_add);
        assert_eq!(
            result_add,
            Polynomial::new(vec![Indeterminate::new(69, 3), Indeterminate::new(2, 2)])
        );
    }

    #[test]
    pub fn mul_poly() {
        use super::{Indeterminate, Polynomial};

        let mul1: Polynomial = Polynomial::new(vec![
            Indeterminate::new(1, 2),
            Indeterminate::new(1, 1),
            Indeterminate::new(1, 0),
        ]);
        let mul2: Polynomial =
            Polynomial::new(vec![Indeterminate::new(1, 1), Indeterminate::new(1, 0)]);
        let result: Polynomial = mul1.clone() * mul2.clone();
        println!("{} multiplied by {} equals {}", mul1, mul2, result);
        assert_eq!(
            result,
            Polynomial::new(vec![
                Indeterminate::new(1, 3),
                Indeterminate::new(2, 2),
                Indeterminate::new(2, 1),
                Indeterminate::new(1, 0),
            ])
        );
    }
}

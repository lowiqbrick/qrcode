use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};
use std::vec;

/// structure that represents an indetermiante as coefficient*x^degree
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Indeterminate {
    /// factor in front pof the indeterminate
    coefficient: i8,
    /// the 'to the power of' factor
    degree: i16,
}

impl Indeterminate {
    pub fn new(coefficient: i8, degree: i16) -> Indeterminate {
        Indeterminate {
            coefficient,
            degree,
        }
    }

    pub fn get_coefficient(&self) -> i8 {
        self.coefficient
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
            ((self.coefficient as i16 * rhs.coefficient as i16) % 256) as i8,
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
            let mut degree_coefficient: i8 = 0;
            for z in self.function.iter() {
                if *degree == z.degree {
                    degree_coefficient =
                        ((degree_coefficient as i16 + z.coefficient as i16) % 256) as i8;
                }
            }
            result.push(Indeterminate::new(degree_coefficient, *degree));
        }
        // delete terms that have a coefficient equal zero
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
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut is_first_element: bool = true;
        for indeterminate in self.function.iter() {
            // print the + in front of x
            // - get printed automatically
            if indeterminate.coefficient >= 0 && !is_first_element {
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
            intermediate_help.coefficient = (-(indeterminate.coefficient as i16) % 256) as i8;
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

impl Div for Polynomial {
    type Output = Polynomial;
    fn div(self, rhs: Self) -> Self::Output {
        let mut quotient: Vec<Indeterminate> = vec![];
        if rhs.function.is_empty() {
            panic!("tried to divide by polynomial of length zero");
        }
        if self.function.is_empty() {
            panic!("tried to divide a polynomial of zero length");
        }
        let mut help: Polynomial = rhs.clone();
        // make shure all is sorted
        let divisor: Polynomial = help.reduce().clone();
        let mut working_dividend: Polynomial = self.clone().reduce();
        let coefficient_divisor: i8 = divisor.function[0].coefficient;
        let degree_divisor: i16 = divisor.function[0].degree;
        // keep track of number of loops and abort if necessary
        let mut highest_poly_divident: i16 = working_dividend.function[0].degree;
        loop {
            // check what the divisor needs to be multiplied by so it's highest powered term equals the
            // highest powered term of the dividend
            let coefficient_divident: i8 = working_dividend.function[0].coefficient;
            let degree_divident: i16 = working_dividend.function[0].degree;
            let coefficient_factor: i8 = coefficient_divident / coefficient_divisor;
            let degree_difference: i16 = degree_divident - degree_divisor;
            // division complete?
            if degree_difference < 0 {
                return Polynomial::new(quotient).reduce();
            }
            let result_part: Indeterminate =
                Indeterminate::new(coefficient_factor, degree_difference);
            quotient.push(result_part);
            // multiply the divisor by an indeterminate for later subtraction
            let working_divisor: Polynomial = divisor.clone() * Polynomial::new(vec![result_part]);
            // println!(
            //     "work dividend: {}; divisor: {}; working divisor {}; result part: {}; quotient so far: {}",
            //     working_dividend,
            //     divisor,
            //     working_divisor,
            //     result_part,
            //     Polynomial::new(quotient.clone())
            // );
            // subtract working_divisor from dividend
            working_dividend = working_dividend - working_divisor;
            working_dividend.reduce();
            // division complete?
            if working_dividend.function.is_empty() {
                return Polynomial::new(quotient).reduce();
            }
            // println!("new working divident {}", working_dividend);
            // prepare for next loop
            highest_poly_divident -= 1;
            assert!(highest_poly_divident >= 0);
        }
    }
}

impl From<Vec<i8>> for Polynomial {
    fn from(item: Vec<i8>) -> Polynomial {
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

impl From<Polynomial> for Vec<i8> {
    fn from(item: Polynomial) -> Vec<i8> {
        let mut return_vec: Vec<i8> = vec![];
        let item_copy: Polynomial = item.clone().reduce();
        for var in item_copy.function.iter() {
            assert!(var.coefficient >= 0);
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
        let inde2: Indeterminate = Indeterminate::new(-1, 6);
        let result2: Indeterminate = inde1 * inde2;
        println!("{} multiplied with {} equals {} ", inde1, inde2, result2);
        assert_eq!(result2, Indeterminate::new(-3, 8));
    }

    #[test]
    pub fn reduce_poly() {
        use super::{Indeterminate, Polynomial};

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
        // make shure coefficients of value zero get deleted
        let mut test_pol_zero: Polynomial = Polynomial::new(vec![
            Indeterminate::new(0, 3),
            Indeterminate::new(0, 1),
            Indeterminate::new(14, 0),
            // also check sorting while being at it
            Indeterminate::new(-3, 2),
        ]);
        let expected_result: Polynomial =
            Polynomial::new(vec![Indeterminate::new(-3, 2), Indeterminate::new(14, 0)]);
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
        let add2: Polynomial = Polynomial::new(vec![Indeterminate::new(-2, 3)]);
        let result_add: Polynomial = add1.clone() + add2.clone();
        println!("{} plus {} equals {}", add1, add2, result_add);
        assert_eq!(
            result_add,
            Polynomial::new(vec![Indeterminate::new(2, 3), Indeterminate::new(2, 2)])
        );
    }

    #[test]
    pub fn minus_poly() {
        use super::{Indeterminate, Polynomial};

        let add1: Polynomial =
            Polynomial::new(vec![Indeterminate::new(4, 3), Indeterminate::new(2, 2)]);
        let add2: Polynomial = Polynomial::new(vec![Indeterminate::new(-2, 3)]);
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

    #[test]
    fn poly_div() {
        use super::{Indeterminate, Polynomial};
        let divident: Polynomial = Polynomial::new(vec![
            Indeterminate::new(1, 3),
            Indeterminate::new(1, 2),
            Indeterminate::new(-9, 1),
            Indeterminate::new(7, 0),
        ]);
        let divisor: Polynomial =
            Polynomial::new(vec![Indeterminate::new(1, 1), Indeterminate::new(-1, 0)]);
        println!(
            "{} divided by {} equals {}",
            divident,
            divisor,
            divident.clone() / divisor.clone()
        );
        assert_eq!(
            divident.clone() / divisor.clone(),
            Polynomial::new(vec![
                Indeterminate::new(1, 2),
                Indeterminate::new(2, 1),
                Indeterminate::new(-7, 0)
            ])
        );
    }

    #[test]
    // with numbers from the veritasium video
    // https://www.youtube.com/watch?v=w5ebcowAJD8
    fn calculate_a_b() {
        use super::{Indeterminate, Polynomial};
        let final_poly_without_a_b: Polynomial = Polynomial::new(vec![
            Indeterminate::new(1, 5),
            Indeterminate::new(-2, 4),
            Indeterminate::new(3, 3),
            Indeterminate::new(5, 2),
        ]);
        let error_correction_x1_x2: Polynomial =
            Polynomial::new(vec![Indeterminate::new(1, 1), Indeterminate::new(-1, 0)])
                * Polynomial::new(vec![Indeterminate::new(1, 1), Indeterminate::new(-2, 0)]);
        println!("original polynom: {}", final_poly_without_a_b);
        println!("error correction term: {}", error_correction_x1_x2);
        let start_with_a_b_extracted: Polynomial =
            final_poly_without_a_b.clone() / error_correction_x1_x2.clone();
        println!(
            "{} divided by {} equals {}",
            final_poly_without_a_b, error_correction_x1_x2, start_with_a_b_extracted
        );
        let mut a_b_term: Polynomial = (start_with_a_b_extracted.clone()
            * error_correction_x1_x2.clone())
            - final_poly_without_a_b.clone();
        a_b_term.reduce();
        println!(
            "the complete term is {}",
            start_with_a_b_extracted.clone() * error_correction_x1_x2.clone()
        );
        println!(
            "the term for a and b is {} and the resulting values are: a={}, b={}",
            a_b_term, a_b_term.function[0].coefficient, a_b_term.function[1].coefficient
        );
        assert_eq!(
            a_b_term,
            Polynomial::new(vec![Indeterminate::new(-37, 1), Indeterminate::new(30, 0)])
        );
    }

    #[test]
    fn conversion_vecu8_poly() {
        use super::Polynomial;
        let test_vec: Vec<i8> = vec![102, 51, 1, 3];
        let to_poly: Polynomial = Polynomial::from(test_vec.clone());
        let back_to_vec: Vec<i8> = Vec::from(to_poly.clone());
        println!(
            "vector {:?} after converison {} and {:?} again",
            test_vec, to_poly, back_to_vec
        );
        assert!(test_vec == back_to_vec)
    }
}

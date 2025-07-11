/// based on "Tutorial on Reed-Solomon Error Correction Coding" by William A. Geisel (August 1990)
/// https://ntrs.nasa.gov/api/citations/19900019023/downloads/19900019023.pdf
/// (last viewed 17.04.2025)
use std::{collections::HashMap, fmt::Display, vec};

use crate::polynomials::Indeterminate;

use super::polynomials::Polynomial;

#[derive(Debug, PartialEq, Clone)]
pub struct GaloisFields {
    m: u8,
    mod_poly: Polynomial,
    galois_table: HashMap<u8, u8>,
}

impl GaloisFields {
    /// converts a polynomila into it's obinary representation
    /// x^4+x^3+x^2+1 => 0b00011101
    pub fn _to_galois_u8(polynomial: Polynomial) -> u8 {
        let mut mask = 0;
        for indeterminate in polynomial.get_function().iter() {
            if (indeterminate.get_degree()) >= 8 {
                panic!(
                    "galois u8 generation failed; encountered degree {} in polynimial",
                    indeterminate.get_degree()
                );
            }
            if indeterminate.get_coefficient() != 1 {
                panic!(
                    "galois u8 generation failed; coefficient in polynomial wasn't 1 (was {} instead)",
                    indeterminate.get_coefficient()
                );
            }
            mask += 1 << indeterminate.get_degree();
        }
        mask
    }

    pub fn _new(m: u8, mod_fx: Polynomial) -> Self {
        assert!(m > 0);
        assert!(m <= 8);
        let mut res_map = HashMap::new();

        // first two entires can be hard coded
        res_map.insert(0, 0);
        res_map.insert(1, 1);
        if m == 1 {
            return GaloisFields {
                m,
                mod_poly: mod_fx,
                galois_table: res_map,
            };
        }

        // put in the generic stuff
        let x1 = Polynomial::new(vec![Indeterminate::new(1, 1)]);
        let mut x = x1.clone();
        for index in 2..=m {
            if let Some(existing_value) =
                res_map.insert(index, GaloisFields::_to_galois_u8(x.clone()))
            {
                eprintln!("overwrote exisitng entry {existing_value:?} in hasmap");
            }
            // don't if this is the last iteration
            if index != m {
                x = x * x1.clone();
            }
        }

        // the acually interesting stuff
        let for_range = if m != 8 {
            m + 1..=(2_u8.pow(m.into()) - 1)
        } else {
            m + 1..=255
        };
        for index in for_range {
            x = x.galois_mul_x1(mod_fx.clone(), m);
            if let Some(existing_value) =
                res_map.insert(index, GaloisFields::_to_galois_u8(x.clone()))
            {
                eprintln!("overwrote exisitng entry {existing_value:?} in hasmap");
            }
        }

        GaloisFields {
            m,
            mod_poly: mod_fx,
            galois_table: res_map,
        }
    }

    /// takes the index of an alpha value and returns the value stored in that alpha
    pub fn index_to_value(&self, alpha_indice: u8) -> u8 {
        // compensate the frost two elements 0 and 1 in the galois field
        let indice = ((alpha_indice as u16 + 1_u16) % 256) as u8;
        assert!(self.galois_table.len() >= (indice as usize));
        // get value
        if let Some(alpha) = self.galois_table.get(&indice) {
            *alpha
        } else {
            panic!("key {} wasn't in hashmap {:?}", indice, self.galois_table);
        }
    }

    /// takes an alpha value and returns the index that value has in the galois field
    pub fn value_to_index(&self, value: u8) -> Result<u8, String> {
        for (index, value_map) in self.galois_table.clone().into_iter() {
            if value_map == value && index != 0 {
                return Ok(index - 1);
            }
        }
        Err(format!("value {value} not found"))
    }

    /// generates the error correction polynomial based on the given galois field and coefficients
    fn generate_corr_polynomial(galois_field: &Self, coefficients_indices: Vec<u8>) -> Polynomial {
        let mut polynomial = Polynomial::new(vec![]);
        let mut current_degree = coefficients_indices.len() - 1;
        for coefficient_index in coefficients_indices.iter() {
            println!("index: {current_degree}");
            println!("coefficient_index: {coefficient_index}");
            let alpha_value = galois_field.index_to_value(*coefficient_index);
            println!("value to index: {alpha_value}");
            if current_degree == (coefficients_indices.len() - 1) {
                polynomial.push(Indeterminate::new(1, current_degree.try_into().unwrap()));
            } else {
                polynomial.push(Indeterminate::new(
                    alpha_value,
                    current_degree.try_into().unwrap(),
                ));
            }
            current_degree = current_degree.saturating_sub(1);
        }
        assert!(current_degree == 0);
        println!("poly: {polynomial}");
        polynomial
    }

    /// meant to be used on a galosi field of m=8 and Polynomial x^4+x^3+x^2+1
    pub fn correction_polynomial(&self, num_error_corr: u8) -> Option<Polynomial> {
        match num_error_corr {
            2 => Some(Self::generate_corr_polynomial(self, vec![1, 25, 1])),
            5 => Some(Self::generate_corr_polynomial(
                self,
                vec![1, 113, 164, 166, 119, 10],
            )),
            7 => Some(Self::generate_corr_polynomial(
                self,
                vec![1, 87, 229, 146, 149, 238, 102, 21],
            )),
            8 => Some(Self::generate_corr_polynomial(
                self,
                vec![1, 175, 238, 208, 249, 215, 252, 196, 28],
            )),
            10 => Some(Self::generate_corr_polynomial(
                self,
                vec![1, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45],
            )),
            13 => Some(Self::generate_corr_polynomial(
                self,
                vec![
                    1, 74, 152, 176, 100, 86, 100, 106, 104, 130, 218, 206, 140, 78,
                ],
            )),
            14 => Some(Self::generate_corr_polynomial(
                self,
                vec![
                    1, 199, 249, 155, 48, 190, 124, 218, 137, 216, 87, 207, 59, 22, 91,
                ],
            )),
            16 => Some(Self::generate_corr_polynomial(
                self,
                vec![
                    1, 120, 104, 107, 109, 102, 161, 76, 3, 91, 191, 147, 169, 182, 194, 225, 120,
                ],
            )),
            17 => Some(Self::generate_corr_polynomial(
                self,
                vec![
                    1, 43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163,
                    136,
                ],
            )),
            18 => Some(Self::generate_corr_polynomial(
                self,
                vec![
                    1, 215, 234, 158, 94, 184, 97, 118, 170, 79, 187, 152, 148, 252, 179, 5, 98,
                    96, 153,
                ],
            )),
            20 => Some(Self::generate_corr_polynomial(
                self,
                vec![
                    1, 17, 60, 79, 50, 61, 163, 26, 187, 202, 180, 221, 225, 83, 239, 156, 164,
                    212, 212, 188, 190,
                ],
            )),
            22 => Some(Self::generate_corr_polynomial(
                self,
                vec![
                    1, 210, 171, 247, 242, 93, 230, 14, 109, 221, 53, 200, 74, 8, 172, 98, 80, 219,
                    134, 160, 105, 165, 231,
                ],
            )),
            _ => None,
        }
    }

    pub fn calculate_error_correction(
        data_bytes: &Polynomial,
        correction_polynomial: Polynomial,
        galois_field: GaloisFields,
    ) -> Polynomial {
        // println!("given data: {}", data_bytes);
        let mut work_data_bytes = data_bytes.clone();
        let number_iterations =
            data_bytes.get_function().len() - correction_polynomial.get_function().len();
        let num_valid_operations = correction_polynomial.get_function().len();
        // println!("num valid operations: {}", num_valid_operations);
        // println!("number itertaions: {}", number_iterations);
        for iteration_index in 0..=number_iterations {
            // get highest degree
            let mut highest_degree = 0;
            for element in work_data_bytes.get_function().iter() {
                if element.get_degree() > highest_degree && element.get_coefficient() != 0 {
                    highest_degree = element.get_degree();
                }
            }
            // println!("highest degree: {}", highest_degree);
            // get value of highest degree
            let mut highest_degree_value_option = None;
            for element in work_data_bytes.get_function().iter() {
                if element.get_degree() == highest_degree {
                    highest_degree_value_option = Some(element.get_coefficient());
                }
            }
            if highest_degree_value_option.is_none() {
                panic!("highest degree couldn't be found even though is should exitst");
            }
            let highest_degree_value = highest_degree_value_option.unwrap();
            // println!("highest degree value: {}", highest_degree_value);
            // multiply everything with highest value
            let mut offset = iteration_index;
            let mut working_iterations = num_valid_operations;
            // is the same for all following iterations
            let highest_value_index = galois_field.value_to_index(highest_degree_value).unwrap();
            // println!(
            //     "index of the highest value in the data is {}",
            //     highest_value_index
            // );
            for element in work_data_bytes.get_function_mut().iter_mut() {
                if offset == 0 && working_iterations != 0 {
                    // get alpha indices from correction and highest value
                    let mut value_correction_value_option = None;
                    for correction_element in correction_polynomial.clone() {
                        if correction_element.get_degree()
                            == (working_iterations - 1).try_into().unwrap()
                        {
                            value_correction_value_option =
                                Some(correction_element.get_coefficient());
                        }
                    }
                    let value_correction_index =
                        if let Some(value_correction_value) = value_correction_value_option {
                            galois_field.value_to_index(value_correction_value).unwrap()
                        } else {
                            panic!(
                                "no index for value {} in galois field",
                                value_correction_value_option.unwrap()
                            );
                        };
                    // println!("--------------\ndata byte at index {}", _index);
                    // add indices
                    let new_alpha_index =
                        ((highest_value_index as u16 + value_correction_index as u16) % 255) as u8;
                    // println!(
                    //     "added indices ({} + {}) % 255 = {}",
                    //     highest_value_index, value_correction_index, new_alpha_index
                    // );
                    let new_alpha_value = galois_field.index_to_value(new_alpha_index);
                    // XOR the values
                    let final_value = new_alpha_value ^ element.get_coefficient();
                    // println!(
                    //     "XOR'ing {} (index {}) and {} to {}",
                    //     new_alpha_value,
                    //     new_alpha_index,
                    //     element.get_coefficient(),
                    //     final_value
                    // );
                    // assign new value
                    element.set_coefficient(final_value);
                }
                if working_iterations != 0 && offset == 0 {
                    working_iterations -= 1;
                }
                offset = offset.saturating_sub(1);
            }
            // println!("after iteration {}: {}", iteration_index, work_data_bytes);
        }
        work_data_bytes
    }
}

impl Display for GaloisFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "GF({})", 2_u16.pow(self.m.into()))?;
        writeln!(f, "mod polynomial: {}", self.mod_poly)?;
        writeln!(
            f,
            "elements   alpha   polynimial representation   dec_value"
        )?;
        writeln!(
            f,
            "--------   -----   -------------------------   ---------"
        )?;
        // get the elements
        let mut keys: Vec<u8> = self.galois_table.keys().copied().collect();
        keys.sort();
        for (index, key) in keys.iter().enumerate() {
            // index
            write!(f, "{key:>8}   ")?;
            // alpha index
            if index == 0 {
                write!(f, "  nan   ")?;
            } else {
                write!(f, "{:>5}   ", index - 1)?;
            }
            // alpha value
            let current_enum = self
                .galois_table
                .get_key_value(key)
                .expect("key that was upposed to be in hashmap wasn't");
            let mut is_leading = true;
            for bit_index in (0..8).rev() {
                let mask = 0b0000_0001 << bit_index;
                if (current_enum.1 & mask) > 0 {
                    write!(f, "1")?;
                    is_leading = false;
                // only print leading zeroes within the set bit limit m
                } else if is_leading && (bit_index >= (self.m)) {
                    write!(f, " ")?;
                } else {
                    write!(f, "0")?;
                }
            }
            write!(f, "                    ")?;
            // decimal value
            writeln!(f, "{:>9}", current_enum.1)?;
        }
        write!(f, "")
    }
}

#[test]
fn test_values_m_4() {
    let example = GaloisFields::_new(
        4,
        Polynomial::new(vec![Indeterminate::new(1, 1), Indeterminate::new(1, 0)]),
    );
    // print value in case of panic!()
    eprintln!("{example}");
    let mut correct_map = HashMap::new();
    correct_map.insert(0, 0b0000_u8);
    correct_map.insert(1, 0b0001_u8);
    correct_map.insert(2, 0b0010_u8);
    correct_map.insert(3, 0b0100_u8);
    correct_map.insert(4, 0b1000_u8);
    correct_map.insert(5, 0b0011_u8);
    correct_map.insert(6, 0b0110_u8);
    correct_map.insert(7, 0b1100_u8);
    correct_map.insert(8, 0b1011_u8);
    correct_map.insert(9, 0b0101_u8);
    correct_map.insert(10, 0b1010_u8);
    correct_map.insert(11, 0b0111_u8);
    correct_map.insert(12, 0b1110_u8);
    correct_map.insert(13, 0b1111_u8);
    correct_map.insert(14, 0b1101_u8);
    correct_map.insert(15, 0b1001_u8);
    let correct_result = GaloisFields {
        m: 4,
        mod_poly: Polynomial::new(vec![Indeterminate::new(1, 1), Indeterminate::new(1, 0)]),
        galois_table: correct_map,
    };
    assert_eq!(example, correct_result);
}

#[test]
fn sanity_alpha_conversion() {
    let galois_field = GaloisFields::_new(
        8,
        Polynomial::new(vec![
            Indeterminate::new(1, 4),
            Indeterminate::new(1, 3),
            Indeterminate::new(1, 2),
            Indeterminate::new(1, 0),
        ]),
    );
    let alpha_index = 120;
    let alpha_value = galois_field.index_to_value(alpha_index);
    let reconstructed_alpha_index = galois_field.value_to_index(alpha_value).unwrap();
    assert_eq!(alpha_index, reconstructed_alpha_index);
}

#[test]
/// default example on red titan
/// https://www.pclviewer.com/rs2/calculator.html (last viewed 20.06.2025)
fn test_calculate_error_correction() {
    let galois_field = GaloisFields::_new(
        8,
        Polynomial::new(vec![
            Indeterminate::new(1, 4),
            Indeterminate::new(1, 3),
            Indeterminate::new(1, 2),
            Indeterminate::new(1, 0),
        ]),
    );
    let Some(correction_polynomial) = galois_field.correction_polynomial(10) else {
        panic!("test couldn't obtain generator polynomial for 10 error bytes");
    };
    let mut data = Polynomial::new(vec![
        Indeterminate::new(16, 15),
        Indeterminate::new(32, 14),
        Indeterminate::new(12, 13),
        Indeterminate::new(86, 12),
        Indeterminate::new(97, 11),
        Indeterminate::new(128, 10),
        Indeterminate::new(236, 9),
        Indeterminate::new(17, 8),
        Indeterminate::new(236, 7),
        Indeterminate::new(17, 6),
        Indeterminate::new(236, 5),
        Indeterminate::new(17, 4),
        Indeterminate::new(236, 3),
        Indeterminate::new(17, 2),
        Indeterminate::new(236, 1),
        Indeterminate::new(17, 0),
    ]);
    data = data * Polynomial::new(vec![Indeterminate::new(1, 10)]);
    // append missing 0x^n's
    for index in (0..=9).rev() {
        data.get_function_mut().push(Indeterminate::new(0, index));
    }
    let result =
        GaloisFields::calculate_error_correction(&data, correction_polynomial, galois_field);
    let expected_result = Polynomial::new(vec![
        Indeterminate::new(0, 25),
        Indeterminate::new(0, 24),
        Indeterminate::new(0, 23),
        Indeterminate::new(0, 22),
        Indeterminate::new(0, 21),
        Indeterminate::new(0, 20),
        Indeterminate::new(0, 19),
        Indeterminate::new(0, 18),
        Indeterminate::new(0, 17),
        Indeterminate::new(0, 16),
        Indeterminate::new(0, 15),
        Indeterminate::new(0, 14),
        Indeterminate::new(0, 13),
        Indeterminate::new(0, 12),
        Indeterminate::new(0, 11),
        Indeterminate::new(0, 10),
        Indeterminate::new(165, 9),
        Indeterminate::new(36, 8),
        Indeterminate::new(212, 7),
        Indeterminate::new(193, 6),
        Indeterminate::new(237, 5),
        Indeterminate::new(54, 4),
        Indeterminate::new(199, 3),
        Indeterminate::new(135, 2),
        Indeterminate::new(44, 1),
        Indeterminate::new(85, 0),
    ]);
    println!("results\ncalculated: {result}\nexpected:   {expected_result}");
    assert_eq!(result, expected_result);
}

/// based on "Tutorial on Reed-Solomon Error Correction Coding" by William A. Geisel (August 1990)
/// https://ntrs.nasa.gov/api/citations/19900019023/downloads/19900019023.pdf
/// (last viewed 17.04.2025)
use std::{collections::HashMap, fmt::Display};

use crate::polynomials::Indeterminate;

use super::polynomials::Polynomial;

#[derive(Debug, PartialEq)]
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
                eprintln!("overwrote exisitng entry {:?} in hasmap", existing_value);
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
                eprintln!("overwrote exisitng entry {:?} in hasmap", existing_value);
            }
        }

        GaloisFields {
            m,
            mod_poly: mod_fx,
            galois_table: res_map,
        }
    }

    pub fn _get_alpha(&self, alpha_indice: u8) -> u8 {
        // compensate the frost two elements 0 and 1 in the galois field
        let indice = alpha_indice + 2;
        assert!(self.galois_table.len() >= (indice as usize));
        // get value
        if let Some(alpha) = self.galois_table.get(&indice) {
            *alpha
        } else {
            panic!("key {} wasn't in hashmap {:?}", indice, self.galois_table);
        }
    }
    /// meant to be used on a galosi field of m=8 and Polynomial x^4+x^3+x^2+1
    pub fn _correction_polynomial(&self, num_error_corr: u8) -> Option<Polynomial> {
        match num_error_corr {
            7 => Some(Polynomial::new(vec![
                Indeterminate::new(1, 7),
                Indeterminate::new(self._get_alpha(87) as i8, 6),
                Indeterminate::new(self._get_alpha(229) as i8, 5),
                Indeterminate::new(self._get_alpha(146) as i8, 4),
                Indeterminate::new(self._get_alpha(149) as i8, 3),
                Indeterminate::new(self._get_alpha(238) as i8, 2),
                Indeterminate::new(self._get_alpha(102) as i8, 1),
                Indeterminate::new(self._get_alpha(21) as i8, 0),
            ])),
            16 => Some(Polynomial::new(vec![
                Indeterminate::new(1, 16),
                Indeterminate::new(self._get_alpha(120) as i8, 15),
                Indeterminate::new(self._get_alpha(104) as i8, 14),
                Indeterminate::new(self._get_alpha(107) as i8, 13),
                Indeterminate::new(self._get_alpha(109) as i8, 12),
                Indeterminate::new(self._get_alpha(102) as i8, 11),
                Indeterminate::new(self._get_alpha(161) as i8, 10),
                Indeterminate::new(self._get_alpha(76) as i8, 9),
                Indeterminate::new(self._get_alpha(3) as i8, 8),
                Indeterminate::new(self._get_alpha(91) as i8, 7),
                Indeterminate::new(self._get_alpha(191) as i8, 6),
                Indeterminate::new(self._get_alpha(147) as i8, 5),
                Indeterminate::new(self._get_alpha(169) as i8, 4),
                Indeterminate::new(self._get_alpha(182) as i8, 3),
                Indeterminate::new(self._get_alpha(194) as i8, 2),
                Indeterminate::new(self._get_alpha(225) as i8, 1),
                Indeterminate::new(self._get_alpha(120) as i8, 0),
            ])),
            _ => None,
        }
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
            write!(f, "{:>8}   ", key)?;
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

// TODO remove debugging test
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

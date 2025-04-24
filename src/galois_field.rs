/// based on "Tutorial on Reed-Solomon Error Correction Coding" by William A. Geisel (August 1990)
/// https://ntrs.nasa.gov/api/citations/19900019023/downloads/19900019023.pdf
/// (last viewed 17.04.2025)
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{write, Display},
};

use crate::polynomials::Indeterminate;

use super::polynomials::Polynomial;

#[derive(Debug, PartialEq)]
pub struct GaloisFields {
    m: u8,
    mod_poly: Polynomial,
    galois_table: HashMap<u8, u8>,
}

impl GaloisFields {
    pub fn new(m: u8, mod_fx: Polynomial) -> Self {
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

        // insert the simple terms
        let mut last_value = 0;
        for index in 2..(m + 1) {
            if (index - 2) < 8 {
                res_map.insert(index, 0b10 << (index - 2));
                last_value = 0b10 << (index - 2);
            }
        }

        // insert remaining values
        // convert polynomial into mask
        let mut mod_mask = 0;
        for indeter in mod_fx.get_function() {
            let current_degree = indeter.get_degree();
            // println!("degree: {}", current_degree);
            assert!((current_degree > -1) && (current_degree < m.into()));
            // println!("current mask: {}", current_mask);
            mod_mask |= 1 << current_degree;
        }
        // create mask to limit values to m
        let mut limit_mask = 0;
        for index in 0..m {
            limit_mask |= 1 << index;
        }
        // println!("limit mask: {:8b}", limit_mask);
        // println!("polynimial as binary: {:8b}", mod_mask);
        // mask values calculate to m
        let upper_limit = match m.cmp(&8) {
            Ordering::Less => 2_u8.pow(m.into()),
            Ordering::Equal => 255_u8,
            Ordering::Greater => panic!("wrong m made it through assert!()"),
        };
        // insert the more complicated parts
        for index in (m + 1)..upper_limit {
            let is_msb_set = (last_value & (1 << (m - 1))) > 0;
            let mut current_value = if is_msb_set {
                ((last_value & 0b0111_1111) << 1) ^ mod_mask
            } else {
                last_value << 1
            };
            current_value &= limit_mask;
            last_value = current_value;
            res_map.insert(index, current_value);
        }
        GaloisFields {
            m,
            mod_poly: mod_fx,
            galois_table: res_map,
        }
    }

    pub fn get_alpha(&self, alpha_indice: u8) -> u8 {
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
    pub fn correction_polynomial(&self, num_error_corr: u8) -> Option<Polynomial> {
        match num_error_corr {
            7 => Some(Polynomial::new(vec![
                Indeterminate::new(1, 7),
                Indeterminate::new(self.get_alpha(87) as i8, 6),
                Indeterminate::new(self.get_alpha(229) as i8, 5),
                Indeterminate::new(self.get_alpha(146) as i8, 4),
                Indeterminate::new(self.get_alpha(149) as i8, 3),
                Indeterminate::new(self.get_alpha(238) as i8, 2),
                Indeterminate::new(self.get_alpha(102) as i8, 1),
                Indeterminate::new(self.get_alpha(21) as i8, 0),
            ])),
            _ => None,
        }
    }
}

impl Display for GaloisFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "GF({})", 2_u16.pow(self.m.into()))?;
        writeln!(f, "mod polynomial: {}", self.mod_poly)?;
        writeln!(f, "elements   polynimial representation")?;
        writeln!(f, "--------   -------------------------")?;
        // get the elements
        let mut keys: Vec<u8> = self.galois_table.keys().copied().collect();
        keys.sort();
        for key in keys {
            write!(f, "{:>8}   ", key)?;
            let current_enum = self
                .galois_table
                .get_key_value(&key)
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
            writeln!(f)?;
        }
        write!(f, "")
    }
}

// TODO remove debugging test
#[test]
fn test_values_m_4() {
    let example = GaloisFields::new(
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

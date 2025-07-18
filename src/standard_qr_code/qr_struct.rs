use crate::galois_field::GaloisFields;
use crate::input::ErrorLevel;
use crate::polynomials::{Indeterminate, Polynomial};
use crate::standard_qr_code::version_constants::{alignment_pattern_data, version_info};
use crate::{standard_qr_code::utils::get_verison_info, Settings};
use std::vec;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
    ops::BitXor,
    vec::Vec,
};

use super::version_constants::information_sequences;

// constants for ANSI colors
// https://en.wikipedia.org/wiki/ANSI_escape_code
// FG  BG
// 30  40 black
// 37  47 white
// 97 107 bright white
const COLORSTOP: &str = "\x1b[m";
const BRIGHTMAGENTA: &str = "\x1b[30;105m";
const BRIGHTWHITE: &str = "\x1b[30;107m";
const WHITE: &str = "\x1b[30;47m";
const BRIGHTBLACK: &str = "\x1b[30;100m";
const BLACK: &str = "\x1b[37;40m";
const RED: &str = "\x1b[37;41m";
const MAGENTA: &str = "\x1b[30;45m";
const BRIGHTRED: &str = "\x1b[37;101m";
const BRIGHTYELLOW: &str = "\x1b[30;103m";
const BRIGHTBLUE: &str = "\x1b[30;104m";
const BLUE: &str = "\x1b[37;44m";
const BRIGHTCYAN: &str = "\x1b[30;106m";

/// constant for byte mode indicator
const BYTEMODEINDICATOR: u8 = 0b0100;
const CHARACTERBITS: u8 = 8;

/// supports writing u8 values bitwise in a vector
#[derive(Clone, Debug)]
struct MyBitVector {
    /// holds the max size of bytes this struct holds
    capacity: u16,
    /// holds the current bit that will be written to next
    curr_position: u32,
    // the actual data
    data: Vec<u8>,
}

impl MyBitVector {
    pub fn print_hex(&self) {
        for element in self.data.iter() {
            print!("{:02x} ", *element);
        }
        println!("(hex)");
    }

    pub fn pad_empty_rest(&mut self) {
        let _begin_pad_index = (self.curr_position as f32 / 8.0).ceil() as usize;
        let padding_value_1 = 0xec;
        let padding_value_2 = 0x11;
        let mut is_value_1 = true;
        for byte_index in _begin_pad_index..self.data.len() {
            if is_value_1 {
                self.data[byte_index] = padding_value_1
            } else {
                self.data[byte_index] = padding_value_2
            }
            is_value_1 ^= true;
        }
    }
}

impl Display for MyBitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut bytes_left = self.data.len();
        for byte in self.data.iter() {
            write!(f, "{}", if (byte & 0b1000_0000u8) > 0 { 1 } else { 0 })?;
            write!(f, "{}", if (byte & 0b0100_0000u8) > 0 { 1 } else { 0 })?;
            write!(f, "{}", if (byte & 0b0010_0000u8) > 0 { 1 } else { 0 })?;
            write!(f, "{}_", if (byte & 0b0001_0000u8) > 0 { 1 } else { 0 })?;
            write!(f, "{}", if (byte & 0b0000_1000u8) > 0 { 1 } else { 0 })?;
            write!(f, "{}", if (byte & 0b0000_0100u8) > 0 { 1 } else { 0 })?;
            write!(f, "{}", if (byte & 0b0000_0010u8) > 0 { 1 } else { 0 })?;
            write!(f, "{}", if (byte & 0b0000_0001u8) > 0 { 1 } else { 0 })?;
            bytes_left -= 1;
            if bytes_left != 0 {
                write!(f, " ")?;
            }
        }
        write!(f, "")
    }
}

impl MyBitVector {
    /// generates new struct with max_size bytes
    fn new_with_capacity(max_size: u16) -> MyBitVector {
        // fill vector with zeroes
        let mut zero_vec: Vec<u8> = Vec::with_capacity(max_size as usize);
        for _index in 0..max_size {
            zero_vec.push(0);
            // if (_index != 0) && (_index % 10 == 0) {
            //     println!("wrote 10 zeroes");
            // }
        }
        // println!(
        //     "new capacity has length {}; max size: {}",
        //     zero_vec.len(),
        //     max_size
        // );
        MyBitVector {
            capacity: max_size,
            curr_position: 0,
            data: zero_vec,
        }
    }

    /// writes size bits of value into MyBitVector
    fn push(&mut self, value: u8, size: u8) {
        // check whether the data fits into the vector
        if self.capacity as u32 * 8 < (self.curr_position + size as u32) {
            eprintln!(
                "MyBitVector has capacity of {} bits is at index {} and as a \
                result {} additional bits can't be written",
                self.capacity * 8,
                self.curr_position,
                size
            );
            panic!();
        }
        // which bit should be read from
        let mut current_bit_read: u8 = size - 1;
        for index in self.curr_position..self.curr_position + size as u32 {
            // println!("read bit {}", current_bit_read);
            assert!(current_bit_read < 8);
            // get the value of the bit in question
            let current_bit: u8 = match current_bit_read {
                0 => value & 0b0000_0001,
                1 => (value & 0b0000_0010) >> 1,
                2 => (value & 0b0000_0100) >> 2,
                3 => (value & 0b0000_1000) >> 3,
                4 => (value & 0b0001_0000) >> 4,
                5 => (value & 0b0010_0000) >> 5,
                6 => (value & 0b0100_0000) >> 6,
                7 => (value & 0b1000_0000) >> 7,
                _ => {
                    panic!("tried to read bit {current_bit_read} of an u8");
                }
            };
            // update for next loop
            current_bit_read = current_bit_read.saturating_sub(1);
            // which bit of the byte index should be written into
            let current_bit_write: u32 = (7 - (index % 8)) % 8;
            // write the value into the data vector
            // round down aka cutt of all decimals of a f32 by converting it to an u-type
            let byte_index: u16 = (self.curr_position as f32 / 8.0) as u16;
            // println!("{}: {}", byte_index, self.capacity);
            assert!(byte_index < self.capacity);
            match current_bit_write {
                0 => self.data[byte_index as usize] |= current_bit,
                1 => self.data[byte_index as usize] |= current_bit << 1,
                2 => self.data[byte_index as usize] |= current_bit << 2,
                3 => self.data[byte_index as usize] |= current_bit << 3,
                4 => self.data[byte_index as usize] |= current_bit << 4,
                5 => self.data[byte_index as usize] |= current_bit << 5,
                6 => self.data[byte_index as usize] |= current_bit << 6,
                7 => self.data[byte_index as usize] |= current_bit << 7,
                _ => panic!("attempted to write to bit {current_bit_write} in u8"),
            }
            // update the byte index
            self.curr_position += 1;
        }
    }

    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

/// represents the qr code symbols statuses, which are uninitialised, true false
#[derive(Debug, Clone, Copy, PartialEq)]

pub enum SymbolStatus {
    /// white symbol in qr code; false
    LogicalFalse,
    /// black symbol in qr code; true
    LogicalTrue,
    /// the symbol hasn't been initialised
    Uninitialised,
}

impl BitXor for SymbolStatus {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        match self {
            SymbolStatus::LogicalFalse => match rhs {
                SymbolStatus::LogicalFalse => SymbolStatus::LogicalFalse,
                SymbolStatus::LogicalTrue => SymbolStatus::LogicalTrue,
                SymbolStatus::Uninitialised => panic!("SymbolStatus::Uninitialised can't be XOR'd"),
            },
            SymbolStatus::LogicalTrue => match rhs {
                SymbolStatus::LogicalFalse => SymbolStatus::LogicalTrue,
                SymbolStatus::LogicalTrue => SymbolStatus::LogicalFalse,
                SymbolStatus::Uninitialised => panic!("SymbolStatus::Uninitialised can't be XOR'd"),
            },
            SymbolStatus::Uninitialised => panic!("SymbolStatus::Uninitialised can't be XOR'd"),
        }
    }
}

/// represents the role of symbol inside the qr code
#[derive(Debug, PartialEq, Clone)]
pub enum SymbolRole {
    /// role not get determined
    Uninitialised,
    /// quiet zone around code
    QuietZone,
    /// Finder Pattern; the recognisable squares in the corners
    FinderPattern,
    /// separator of Finder Patterns and actual data
    Separator,
    /// Timing pattern; defines spacing of symbols
    TimingPattern,
    /// Alignment Pattern; 3x3 squares for qrcodes of version 2 or larger
    AlignmentPattern,
    /// Encoding region; acual data
    EncodingRegion,
    /// reserving Version Information for writing later
    ReservedVersionInformation,
    /// Version information; version 7 or larger
    VersionInformation,
    /// reserving Format information for writing later
    ReservedFormatInformation,
    /// Format Information; error correction level and mask pattern
    FormatInformation,
    /// Element that is always LogicalTrue/black
    AlwaysTrue,
}

/// contains error correction block information
#[derive(Clone, Copy, Debug)]
pub struct ErrorBlockInfo {
    /// amount of this block in this version
    pub num_block: u8,
    /// data length in this block
    pub num_data_bytes: u8,
    /// number of error correction bytes in this block
    pub num_error_bytes: u8,
}

impl ErrorBlockInfo {
    pub fn new(num_block: u8, total_block_len: u8, num_data_bytes: u8) -> ErrorBlockInfo {
        ErrorBlockInfo {
            num_block,
            num_data_bytes,
            num_error_bytes: total_block_len - num_data_bytes,
        }
    }
}

/// encomposes all data required to generate a qr code
#[derive(Debug, Clone)]
pub struct QRData {
    output_data: Vec<Vec<SymbolStatus>>,
    role_data: Vec<Vec<SymbolRole>>,
    version: u8,
    error_blocks: Vec<ErrorBlockInfo>,
    settings: Settings,
}

macro_rules! bit_to_qrcode {
    ($self: expr, $x_1: expr, $y_1: expr, $x_2: expr, $y_2: expr, $bit: expr) => {
        if $bit {
            $self.output_data[$x_1][$y_1] = SymbolStatus::LogicalTrue;
        } else {
            $self.output_data[$x_1][$y_1] = SymbolStatus::LogicalFalse;
        }
        $self.role_data[$x_1][$y_1] = SymbolRole::FormatInformation;
        if $bit {
            $self.output_data[$x_2][$y_2] = SymbolStatus::LogicalTrue;
        } else {
            $self.output_data[$x_2][$y_2] = SymbolStatus::LogicalFalse;
        }
        $self.role_data[$x_2][$y_2] = SymbolRole::FormatInformation;
    };
}

macro_rules! write_format_info {
    ($self:expr, $smallest_index: expr, $biggest_index: expr, $final_data_bits: expr) => {
        // 14
        bit_to_qrcode!(
            $self,
            $smallest_index,
            $smallest_index + 8,
            $smallest_index + 8,
            $biggest_index,
            ($final_data_bits & 0b0100_0000_0000_0000) > 0
        );
        // 13
        bit_to_qrcode!(
            $self,
            $smallest_index + 1,
            $smallest_index + 8,
            $smallest_index + 8,
            $biggest_index - 1,
            ($final_data_bits & 0b0010_0000_0000_0000) > 0
        );
        // 12
        bit_to_qrcode!(
            $self,
            $smallest_index + 2,
            $smallest_index + 8,
            $smallest_index + 8,
            $biggest_index - 2,
            ($final_data_bits & 0b0001_0000_0000_0000) > 0
        );
        // 11
        bit_to_qrcode!(
            $self,
            $smallest_index + 3,
            $smallest_index + 8,
            $smallest_index + 8,
            $biggest_index - 3,
            ($final_data_bits & 0b0000_1000_0000_0000) > 0
        );
        // 10
        bit_to_qrcode!(
            $self,
            $smallest_index + 4,
            $smallest_index + 8,
            $smallest_index + 8,
            $biggest_index - 4,
            ($final_data_bits & 0b0000_0100_0000_0000) > 0
        );
        // 9
        bit_to_qrcode!(
            $self,
            $smallest_index + 5,
            $smallest_index + 8,
            $smallest_index + 8,
            $biggest_index - 5,
            ($final_data_bits & 0b0000_0010_0000_0000) > 0
        );
        // 8
        bit_to_qrcode!(
            $self,
            $smallest_index + 7,
            $smallest_index + 8,
            $smallest_index + 8,
            $biggest_index - 6,
            ($final_data_bits & 0b0000_0001_0000_0000) > 0
        );
        // since the other fields are changed from ReservedFormatInformation to
        // FormatInformation the element that is always black, should be adjusted aswell
        $self.role_data[$smallest_index + 8][$biggest_index - 7] = SymbolRole::AlwaysTrue;
        // 7
        bit_to_qrcode!(
            $self,
            $smallest_index + 8,
            $smallest_index + 8,
            $biggest_index - 7,
            $smallest_index + 8,
            ($final_data_bits & 0b0000_0000_1000_0000) > 0
        );
        // 6
        bit_to_qrcode!(
            $self,
            $smallest_index + 8,
            $smallest_index + 7,
            $biggest_index - 6,
            $smallest_index + 8,
            ($final_data_bits & 0b0000_0000_0100_0000) > 0
        );
        // 5
        bit_to_qrcode!(
            $self,
            $smallest_index + 8,
            $smallest_index + 5,
            $biggest_index - 5,
            $smallest_index + 8,
            ($final_data_bits & 0b0000_0000_0010_0000) > 0
        );
        // 4
        bit_to_qrcode!(
            $self,
            $smallest_index + 8,
            $smallest_index + 4,
            $biggest_index - 4,
            $smallest_index + 8,
            ($final_data_bits & 0b0000_0000_0001_0000) > 0
        );
        // 3
        bit_to_qrcode!(
            $self,
            $smallest_index + 8,
            $smallest_index + 3,
            $biggest_index - 3,
            $smallest_index + 8,
            ($final_data_bits & 0b0000_0000_0000_1000) > 0
        );
        // 2
        bit_to_qrcode!(
            $self,
            $smallest_index + 8,
            $smallest_index + 2,
            $biggest_index - 2,
            $smallest_index + 8,
            ($final_data_bits & 0b0000_0000_0000_0100) > 0
        );
        // 1
        bit_to_qrcode!(
            $self,
            $smallest_index + 8,
            $smallest_index + 1,
            $biggest_index - 1,
            $smallest_index + 8,
            ($final_data_bits & 0b0000_0000_0000_0010) > 0
        );
        // 0
        bit_to_qrcode!(
            $self,
            $smallest_index + 8,
            $smallest_index,
            $biggest_index,
            $smallest_index + 8,
            ($final_data_bits & 0b0000_0000_0000_0001) > 0
        );
    };
}

impl QRData {
    /// generate the data
    pub fn new(input: Settings) -> QRData {
        let (version, error_blocks) =
            match get_verison_info(input.information.len(), input.error_level) {
                Ok(result) => result,
                Err(msg) => {
                    eprintln!("{msg}");
                    panic!()
                }
            };
        // calculate width of the code
        // width = 17 + 4 * <version number>
        let width: usize = 17 + 4 * version as usize;
        // size of the code with the quiet zone
        // quiet zone: 4 square whide border around the qr code
        let width_quite_zone: usize = width + (2 * 4);
        // declare the vectors
        let mut output_data: Vec<Vec<SymbolStatus>> = Vec::with_capacity(width_quite_zone);
        let mut role_data: Vec<Vec<SymbolRole>> = Vec::with_capacity(width_quite_zone);
        // push the defaults
        for row in 0..width_quite_zone {
            output_data.push(Vec::with_capacity(width_quite_zone));
            role_data.push(Vec::with_capacity(width_quite_zone));
            // initialise the vectors
            for _ in 0..width_quite_zone {
                output_data[row].push(SymbolStatus::Uninitialised);
                role_data[row].push(SymbolRole::Uninitialised);
            }
        }
        QRData {
            output_data,
            role_data,
            version,
            error_blocks,
            settings: input,
        }
    }

    /// returns the version
    pub fn get_version(&self) -> u8 {
        self.version
    }

    /// returns the witdh
    pub fn get_width(&self) -> usize {
        self.output_data[0].len()
    }

    /// returns a reference to the settings
    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    /// returns a reference to the error block information
    pub fn get_error_info(&self) -> &Vec<ErrorBlockInfo> {
        &self.error_blocks
    }

    // draws the quiet zone around the code
    pub fn quiet_zone(&mut self) {
        if self.get_settings().debugging {
            println!("default initialisation of the data");
            print!("{self}");
        }
        let width: usize = self.output_data.len();
        for x in 0..width {
            for y in 0..width {
                if x <= 3 || x >= width - 4 || y <= 3 || y >= width - 4 {
                    self.output_data[x][y] = SymbolStatus::LogicalFalse;
                    self.role_data[x][y] = SymbolRole::QuietZone;
                }
            }
        }
        if self.get_settings().debugging {
            println!("drew quiet zone");
            print!("{self}");
        }
    }

    /// adds the finder patterns for the code
    pub fn finders(&mut self) {
        if self.settings.debugging {
            println!(
                "width: {} height: {}",
                self.output_data.len(),
                self.output_data[0].len()
            );
        }
        // finding pattern
        let logical_true: SymbolStatus = SymbolStatus::LogicalTrue;
        let logical_false: SymbolStatus = SymbolStatus::LogicalFalse;
        let pattern: Vec<Vec<SymbolStatus>> = vec![
            vec![
                logical_true,
                logical_true,
                logical_true,
                logical_true,
                logical_true,
                logical_true,
                logical_true,
            ],
            vec![
                logical_true,
                logical_false,
                logical_false,
                logical_false,
                logical_false,
                logical_false,
                logical_true,
            ],
            vec![
                logical_true,
                logical_false,
                logical_true,
                logical_true,
                logical_true,
                logical_false,
                logical_true,
            ],
            vec![
                logical_true,
                logical_false,
                logical_true,
                logical_true,
                logical_true,
                logical_false,
                logical_true,
            ],
            vec![
                logical_true,
                logical_false,
                logical_true,
                logical_true,
                logical_true,
                logical_false,
                logical_true,
            ],
            vec![
                logical_true,
                logical_false,
                logical_false,
                logical_false,
                logical_false,
                logical_false,
                logical_true,
            ],
            vec![
                logical_true,
                logical_true,
                logical_true,
                logical_true,
                logical_true,
                logical_true,
                logical_true,
            ],
        ];
        // width with quiet zone
        let width: usize = self.output_data[0].len();
        // vector that contains the centre coordinate of the finder patterns
        // centre qiet pattern = quiet zone + distance to centre - index starting at zero
        // 7 = 4 + 4 - 1
        let pattern_centers: Vec<(usize, usize)> = vec![(7, 7), (width - 8, 7), (7, width - 8)];
        // go over the qrcode to fill in the patterns
        // left to right
        for x in 0..width {
            // top to bottom
            for y in 0..width {
                // check manhattan distance from current coodinate to pattern centers
                for pattern_centre in pattern_centers.iter() {
                    let x_difference_neg: i16 = x as i16 - pattern_centre.0 as i16;
                    let x_difference = x_difference_neg.abs();
                    let y_difference_neg: i16 = y as i16 - pattern_centre.1 as i16;
                    let y_difference = y_difference_neg.abs();
                    // make shure that the current position is within the pattern
                    if x_difference < 4
                        && y_difference < 4
                        && self.role_data[x][y] == SymbolRole::Uninitialised
                    {
                        self.output_data[x][y] = pattern[(x_difference_neg + 3) as usize]
                            [(y_difference_neg + 3) as usize];
                        self.role_data[x][y] = SymbolRole::FinderPattern;
                    }
                }
            }
        }
        if self.get_settings().debugging {
            println!("drew finding patterns");
            print!("{self}");
        }
    }

    /// add separators between finder patterns and data
    pub fn separators(&mut self) {
        let width: usize = self.get_width();
        for x in 0..width {
            for y in 0..width {
                // make shure the current position is not at the edge
                if (x > 0) && (x < width - 1) && (y > 0) && (y < width - 1) {
                    // make shure an finder pattern element is in direct vicinity
                    if (self.role_data[x + 1][y] == SymbolRole::FinderPattern
                        || self.role_data[x][y + 1] == SymbolRole::FinderPattern
                        || self.role_data[x - 1][y] == SymbolRole::FinderPattern
                        || self.role_data[x][y - 1] == SymbolRole::FinderPattern
                        || self.role_data[x - 1][y - 1] == SymbolRole::FinderPattern
                        || self.role_data[x + 1][y - 1] == SymbolRole::FinderPattern
                        || self.role_data[x - 1][y + 1] == SymbolRole::FinderPattern)
                        && (self.role_data[x][y] == SymbolRole::Uninitialised
                            || self.role_data[x][y] == SymbolRole::TimingPattern)
                    {
                        self.output_data[x][y] = SymbolStatus::LogicalFalse;
                        self.role_data[x][y] = SymbolRole::Separator;
                    }
                }
            }
        }
        if self.get_settings().debugging {
            println!("drew separators");
            print!("{self}");
        }
    }

    /// adding timing patterns to the code
    pub fn timing_pattern(&mut self) {
        // width with quiet zone
        let width: usize = self.output_data[0].len();
        for x in 0..width {
            for y in 0..width {
                if (x == 10 || y == 10) && self.role_data[x][y] == SymbolRole::Uninitialised {
                    if x == 10 {
                        if (y + 1) % 2 == 1 {
                            self.output_data[x][y] = SymbolStatus::LogicalTrue;
                        } else {
                            self.output_data[x][y] = SymbolStatus::LogicalFalse;
                        }
                    } else {
                        self.output_data[x][y] = if (x + 1) % 2 == 1 {
                            SymbolStatus::LogicalTrue
                        } else {
                            SymbolStatus::LogicalFalse
                        }
                    }
                    self.role_data[x][y] = SymbolRole::TimingPattern;
                }
            }
        }
        if self.get_settings().debugging {
            println!("drew timing patterns");
            print!("{self}");
        }
    }

    /// reserve space for the format information so the data can be written without overwriting
    /// the format information data
    pub fn reserve_format_information(&mut self) {
        let width: usize = self.get_width();
        let max_index_width: usize = width - 1;
        for x in 0..width {
            for y in 0..width {
                // make shure the current position is not at the edge
                if (x > 0) && (x < width - 1) && (y > 0) && (y < width - 1) {
                    // make shure an finder pattern element is in direct vicinity
                    if (self.role_data[x + 1][y] == SymbolRole::Separator
                        || self.role_data[x][y + 1] == SymbolRole::Separator
                        || self.role_data[x - 1][y] == SymbolRole::Separator
                        || self.role_data[x][y - 1] == SymbolRole::Separator
                        || self.role_data[x - 1][y - 1] == SymbolRole::Separator
                        || self.role_data[x + 1][y - 1] == SymbolRole::Separator
                        || self.role_data[x - 1][y + 1] == SymbolRole::Separator)
                        && self.role_data[x][y] == SymbolRole::Uninitialised
                    {
                        // make shure nothing is added "on top" and "the left side" of Separators
                        if y != (max_index_width - 4 - 7 - 1) && x != (max_index_width - 4 - 7 - 1)
                        {
                            self.role_data[x][y] = SymbolRole::ReservedFormatInformation;
                            // turn the elements white for mask calculation later
                            if x == 12 && y == (width - 1 - 4 - 7) {
                                self.output_data[x][y] = SymbolStatus::LogicalTrue;
                            } else {
                                self.output_data[x][y] = SymbolStatus::LogicalFalse;
                            }
                        }
                    }
                }
            }
        }
        if self.get_settings().debugging {
            println!("after reserving place for format information");
            print!("{self}");
        }
    }

    /// draw alignment pattern into qr code
    pub fn draw_alignment_pattern(&mut self) {
        let width: usize = self.get_width();
        let logical_true: SymbolStatus = SymbolStatus::LogicalTrue;
        let logical_false: SymbolStatus = SymbolStatus::LogicalFalse;
        let alignment_pattern: Vec<Vec<SymbolStatus>> = vec![
            vec![
                logical_true,
                logical_true,
                logical_true,
                logical_true,
                logical_true,
            ],
            vec![
                logical_true,
                logical_false,
                logical_false,
                logical_false,
                logical_true,
            ],
            vec![
                logical_true,
                logical_false,
                logical_true,
                logical_false,
                logical_true,
            ],
            vec![
                logical_true,
                logical_false,
                logical_false,
                logical_false,
                logical_true,
            ],
            vec![
                logical_true,
                logical_true,
                logical_true,
                logical_true,
                logical_true,
            ],
        ];
        if self.settings.debugging {
            println!("version: {}", self.version);
        }
        // alignment_information.1 is empty for version 1, since version 1 doesn't have any alignment patterns
        if self.version > 1 {
            let mut alignment_information: (u8, Vec<u8>) = alignment_pattern_data(self.version);
            // increase the centres of the alignment patterns by four to compensate for the quiet zone
            // in the indices
            for alignment_centre in alignment_information.1.iter_mut() {
                *alignment_centre += 4;
            }
            // first and last element of the alignment centres to avoid drawing into finder patterns
            let lower_end: u8 = alignment_information.1[0];
            let upper_end: u8 = alignment_information.1[alignment_information.1.len() - 1];
            if self.settings.debugging {
                println!("alignment lower: {lower_end} upper: {upper_end}");
            }
            // go over all qr code elements
            for x in 0..width {
                for y in 0..width {
                    // go over all rq code elements
                    for x_align in alignment_information.1.iter() {
                        for y_align in alignment_information.1.iter() {
                            // don't draw on finder patterns
                            if !(((*x_align == upper_end || *x_align == lower_end)
                                && *y_align == lower_end)
                                || (*x_align == lower_end && *y_align == upper_end))
                            {
                                let x_diff: i32 = *x_align as i32 - x as i32;
                                let y_diff: i32 = *y_align as i32 - y as i32;
                                if (-2..=2).contains(&x_diff) && (-2..=2).contains(&y_diff) {
                                    self.output_data[x][y] = alignment_pattern
                                        [(x_diff + 2) as usize]
                                        [(y_diff + 2) as usize];
                                    self.role_data[x][y] = SymbolRole::AlignmentPattern;
                                }
                            }
                        }
                    }
                }
            }
        }
        if self.get_settings().debugging {
            println!("after drawing alignment patterns");
            print!("{self}");
        }
    }

    /// reserve version information
    pub fn reserve_version_information(&mut self) {
        // version information exists only in version 7 and up
        if self.version >= 7 {
            let width: usize = self.get_width();
            for x in 0..width {
                for y in 0..width {
                    // draw in bottom left
                    if (4..=9).contains(&x)
                        && (y <= (width - 4 - 7 - 1))
                        && (y >= (width - 4 - 7 - 1 - 3))
                        && self.role_data[x][y] != SymbolRole::Separator
                    {
                        self.role_data[x][y] = SymbolRole::ReservedVersionInformation;
                    }
                    if (x <= (width - 4 - 7 - 1))
                        && (x >= (width - 4 - 7 - 1 - 3))
                        && (4..=9).contains(&y)
                        && self.role_data[x][y] != SymbolRole::Separator
                    {
                        self.role_data[x][y] = SymbolRole::ReservedVersionInformation;
                    }
                }
            }
        }
        if self.get_settings().debugging {
            println!("after reserving version information");
            print!("{self}");
        }
    }

    /// get all information required for the processing of the data down the road
    fn generate_error_blocks(&self) -> (Vec<ErrorBlockInfo>, Vec<Vec<u8>>, usize, usize) {
        let error_blocks: Vec<ErrorBlockInfo> = self.error_blocks.clone();
        if self.settings.debugging {
            println!("error blocks: {error_blocks:?}");
        }
        // create vector to contain the errorblock data
        let mut all_blocks: Vec<Vec<u8>> = vec![];
        let mut _tot_num_blocks: usize = 0;
        let mut _tot_num_codewords: usize = 0;
        for block in error_blocks.iter() {
            for _ in 0..block.num_block {
                all_blocks.push(vec![]);
                _tot_num_blocks += 1;
                _tot_num_codewords += (block.num_data_bytes + block.num_error_bytes) as usize;
            }
        }
        (
            error_blocks,
            all_blocks,
            _tot_num_blocks,
            _tot_num_codewords,
        )
    }

    /// all the data that should be written into the qrcode is taken from the struct and inserted into
    /// a custom struct, that allows easier processing later
    fn write_data_into_vectors(&self, error_blocks: &[ErrorBlockInfo]) -> Vec<MyBitVector> {
        // get the data
        let data: String = self.settings.information.clone();
        // create MyBitVectors to write data into
        let mut bit_vectors: Vec<MyBitVector> = vec![];
        for block in error_blocks.iter() {
            for _ in 0..block.num_block {
                if self.settings.debugging {
                    println!("num data bytes: {}", block.num_data_bytes);
                }
                bit_vectors.push(MyBitVector::new_with_capacity(block.num_data_bytes.into()));
            }
        }
        if self.settings.debugging {
            println!(
                "bit vectors: {:?} (length {})",
                bit_vectors,
                bit_vectors.len()
            );
            for vector in bit_vectors.iter() {
                vector.print_hex();
            }
            for vector in bit_vectors.iter() {
                println!("{vector}");
            }
        }
        // write mode bits into data
        assert!(!bit_vectors.is_empty());
        let mut bit_vector_index: usize = 0;
        // add byte mode indicator (for byte mode)
        bit_vectors[bit_vector_index].push(BYTEMODEINDICATOR, 4);
        // add character count indicator
        let len_text: usize = data.len();
        if self.version >= 10 {
            bit_vectors[bit_vector_index].push(((len_text & 0b1111_1111_0000_0000) >> 8) as u8, 8);
            bit_vectors[bit_vector_index].push((len_text & 0b0000_0000_1111_1111) as u8, 8);
        } else {
            bit_vectors[bit_vector_index].push((len_text & 0b1111_1111) as u8, 8);
        }
        for char in data.chars() {
            // get index and size of MyBitVector
            let vector_index: u32 = bit_vectors[bit_vector_index].curr_position;
            let vector_capacity: u16 = bit_vectors[bit_vector_index].capacity * 8;
            // check if entire char can be written into MyBitVector
            let remaining_capacity: u32 = vector_capacity as u32 - vector_index;
            if remaining_capacity >= CHARACTERBITS as u32 {
                bit_vectors[bit_vector_index].push(char as u8, CHARACTERBITS);
            } else if remaining_capacity == 4 {
                // write what fits into vector
                bit_vectors[bit_vector_index]
                    .push((char as u8 & 0b1111_0000) >> 4, remaining_capacity as u8);
                // increase index
                bit_vector_index += 1;
                assert!(bit_vector_index < bit_vectors.len());
                bit_vectors[bit_vector_index]
                    .push(char as u8 & 0b0000_1111, remaining_capacity as u8);
            } else {
                panic!("remaining capacity wasn't 4, but {remaining_capacity}");
            }
        }
        if self.settings.debugging {
            println!(
                "bit vectors: {:?} (length {})",
                bit_vectors,
                bit_vectors.len()
            );
            for vector in bit_vectors.iter() {
                vector.print_hex();
            }
            for vector in bit_vectors.iter() {
                println!("{vector}");
            }
        }
        // pad a bit vector if it has bytes that are unused
        for bit_vector in bit_vectors.iter_mut() {
            bit_vector.pad_empty_rest();
        }
        if self.settings.debugging {
            println!("padded vectors");
            for vector in bit_vectors.iter() {
                vector.print_hex();
            }
        }
        bit_vectors
    }

    /// calculates the erroro correction term that must be attached to the data bytes
    /// to restore information lost due to low readablity of the code
    fn generate_error_corrction(
        &self,
        error_blocks: &[ErrorBlockInfo],
        all_blocks: &mut [Vec<u8>],
        bit_vectors: &[MyBitVector],
    ) {
        let mut vector_index: u8 = 0;
        if self.settings.debugging {
            println!("individual error blocks:");
        }
        let galois_field = GaloisFields::_new(
            8,
            Polynomial::new(vec![
                Indeterminate::new(1, 4),
                Indeterminate::new(1, 3),
                Indeterminate::new(1, 2),
                Indeterminate::new(1, 0),
            ]),
        );
        // go through all blocks
        for block in error_blocks.iter() {
            let Some(generator_polynomial) = galois_field.correction_polynomial(
                (block.num_data_bytes + block.num_error_bytes) - block.num_data_bytes,
            ) else {
                panic!("correction polynomial wasn't found")
            };
            if self.settings.debugging {
                println!("error correction polynomial: {generator_polynomial}");
            }
            // process individual blocks
            for _ in 0..block.num_block {
                let mut raw_polynomial: Polynomial =
                    Polynomial::from(bit_vectors[vector_index as usize].get_data())
                        * Polynomial::new(vec![Indeterminate::new(
                            1,
                            block.num_error_bytes as i16,
                        )]);
                if self.settings.debugging {
                    println!(
                        "pre expansion polynomial (block {}): {}",
                        vector_index,
                        Polynomial::from(bit_vectors[vector_index as usize].get_data())
                    );
                    println!("raw polynomial (block {vector_index}): {raw_polynomial}");
                }
                // attach missing 0x^n
                let high_degree = raw_polynomial.get_function()[0].get_degree();
                for degree in (0..=high_degree).rev() {
                    if !raw_polynomial.is_degree_in_field(degree) {
                        raw_polynomial
                            .get_function_mut()
                            .push(Indeterminate::new(0, degree));
                    }
                }
                // generate error bytes
                let error_polynomial = GaloisFields::calculate_error_correction(
                    &raw_polynomial,
                    generator_polynomial.clone(),
                    galois_field.clone(),
                );
                // put data and error correction together
                let final_polynomial = raw_polynomial + error_polynomial;
                // write results into final vector
                for values in final_polynomial.into_iter() {
                    all_blocks[vector_index as usize].push(values.get_coefficient());
                }
                vector_index += 1;
            }
        }
        if self.settings.debugging {
            println!("Galois Field: {galois_field}");
        }
        if self.settings.debugging {
            println!("all values:");
            for data_printout in all_blocks.iter() {
                println!("{data_printout:?}");
            }
        }
    }

    /// the to be written data is distributed across the qrcode to raise
    /// the chances of restoring data that is either lost or not read correctly
    fn shuffle_bit_vectors(
        &self,
        error_blocks: &[ErrorBlockInfo],
        all_blocks: &[Vec<u8>],
        tot_num_blocks: usize,
        tot_num_codewords: usize,
    ) -> Vec<u8> {
        if self.settings.debugging {
            println!("all blocks info: {all_blocks:x?}");
            for vector in all_blocks.iter() {
                println!("block len: {}", vector.len());
            }
        }
        // write the data into one vector that contains all data to be written into the code
        // adhering to the construction of the final message codeword sequence
        // Block 1      D1 | D2 | ..... D11|      E1 | E2 | ..... E22|
        // Block 2      D12| D13| ..... D22|      E23| E24| ..... E44|
        // Block 3      D23| D24| ..... D33| D34| E45| E46| ..... E66|
        // Block 4      D35| D36| ..... D45| D46| E67| E68| ..... E88|
        // =>              V    V          V    V    V    V          V
        // D1, D12, D23, D35, D2, D13, D24, D36, ... D11, D22, D33, D45, D34, D46, E1, E23, E45, E67, E2, E24, E46, E68, ... E22, E44, E66,E88
        // the tricky part is that the first two block(s) are potentially shorter than the last two blocks and the "nonexistend" end of the first blocks
        // must be skipped
        // stores final data
        let mut final_data_vect: Vec<u8> = vec![];
        // indices for iteration trougth all blocks
        let mut vector_index: usize = 0;
        let mut block_index: usize = 0;
        // index for prevention of infinite loop
        let mut sanity_loop: u16 = 0;
        // if there is only one block all data can just be copied as is
        if error_blocks.len() == 1 {
            loop {
                // end loop if arrived at the end
                if block_index == 0 && vector_index == all_blocks[0].len() {
                    break;
                }

                // write values in finalvector
                final_data_vect.push(all_blocks[block_index][vector_index]);

                // increase vector index
                block_index += 1;
                // prepare indices for next loop
                // next block if at the end of current block
                if block_index >= tot_num_blocks {
                    block_index = 0;
                    vector_index += 1;
                }
                if sanity_loop > 3706 {
                    eprintln!("emergency loop abort triggered");
                    break;
                } else {
                    sanity_loop += 1;
                }
            }
        } else if error_blocks.len() == 2 {
            if self.settings.debugging {
                println!("entered the shuffeling of the data bytes");
            }
            // loop from beginning of vector to the end
            // go full length over the longer vector
            for vector_env in
                0..(error_blocks[1].num_data_bytes + error_blocks[1].num_error_bytes) as usize
            {
                // go over every vector at the current index
                for block_env in all_blocks.iter().enumerate() {
                    // is this the shorter block?
                    if block_env.0 < error_blocks[0].num_block.into() {
                        let left_out_index = (error_blocks[0].num_data_bytes) as usize;
                        // check for the index that is supposed to be left out
                        match vector_env.cmp(&left_out_index) {
                            // if index above the index to be left out a value wasn't written and
                            // the index must be artificialy lowered to account for the öeft out value
                            Ordering::Greater => {
                                if vector_env == 0 {
                                    eprintln!("came in greater part with vector_env=0")
                                } else if vector_env
                                    < (error_blocks[0].num_data_bytes
                                        + error_blocks[0].num_error_bytes)
                                        as usize
                                {
                                    final_data_vect.push(all_blocks[block_env.0][vector_env - 1])
                                // this triggers in the last index
                                } else {
                                    final_data_vect.push(all_blocks[block_env.0][vector_env - 2])
                                }
                            }
                            // if at the index to be left out, do nothing
                            Ordering::Equal => (),
                            // if below the index to be left out, write as usual
                            Ordering::Less => {
                                final_data_vect.push(all_blocks[block_env.0][vector_env])
                            }
                        }
                    } else {
                        // write data into vector
                        final_data_vect.push(all_blocks[block_env.0][vector_env]);
                    }
                }
            }
        } else {
            panic!(
                "qr code contained {} error blocks (1, or 2 are valid)",
                error_blocks.len()
            );
        }
        // everything was written?
        if final_data_vect.len() != tot_num_codewords {
            eprint!(
                "final data vector length: {}; total number of codewords: {}",
                final_data_vect.len(),
                tot_num_codewords
            );
            eprintln!("final data: {final_data_vect:x?}")
        }
        assert!(final_data_vect.len() == tot_num_codewords);
        if self.settings.debugging {
            println!(
                "data in final vector: {:x?} (length {})",
                final_data_vect,
                final_data_vect.len()
            );
            for element in final_data_vect.iter() {
                print!("{element:02x} ")
            }
            println!();
        }
        final_data_vect
    }

    /// write all data into the QR code struct
    fn write_into_self(&mut self, final_data_vect: &[u8]) {
        // write all data into the actual QR code
        let mut vector_bit_index: usize = 0;
        let mut x_index: usize = self.output_data.len() - 1 - 4;
        let mut y_index: usize = self.output_data.len() - 1 - 4;
        if self.settings.debugging {
            println!("starting indices for writing\nx: {x_index}\ny: {y_index}");
        }
        let mut is_y_shrinking: bool = true;
        let mut is_right: bool = true;
        // go througth all elements of the qr code and write data into
        // everything uninitialised yet
        loop {
            // if nothing is inside the element yet
            if self.role_data[x_index][y_index] == SymbolRole::Uninitialised {
                // writing data
                if (vector_bit_index / 8) < final_data_vect.len() {
                    // figure out if the current bit is true or false
                    let element_value: bool = match vector_bit_index % 8 {
                        0 => (final_data_vect[vector_bit_index / 8] & 0b1000_0000) > 0,
                        1 => (final_data_vect[vector_bit_index / 8] & 0b0100_0000) > 0,
                        2 => (final_data_vect[vector_bit_index / 8] & 0b0010_0000) > 0,
                        3 => (final_data_vect[vector_bit_index / 8] & 0b0001_0000) > 0,
                        4 => (final_data_vect[vector_bit_index / 8] & 0b0000_1000) > 0,
                        5 => (final_data_vect[vector_bit_index / 8] & 0b0000_0100) > 0,
                        6 => (final_data_vect[vector_bit_index / 8] & 0b0000_0010) > 0,
                        7 => (final_data_vect[vector_bit_index / 8] & 0b0000_0001) > 0,
                        _ => panic!(
                            "wrong bit index in qr code writing {}",
                            vector_bit_index % 8
                        ),
                    };
                    if element_value {
                        self.output_data[x_index][y_index] = SymbolStatus::LogicalTrue;
                    } else {
                        self.output_data[x_index][y_index] = SymbolStatus::LogicalFalse;
                    }
                } else {
                    // if all data is written fill the remaining data in the code with logical false
                    self.output_data[x_index][y_index] = SymbolStatus::LogicalFalse;
                    if self.settings.debugging {
                        println!("wrote filler false");
                    }
                }
                self.role_data[x_index][y_index] = SymbolRole::EncodingRegion;

                // increase bit index
                vector_bit_index += 1;
                // println!("{}", self);
            }

            // update the indices
            // elements get written in a right-left-right-left manner bottom up in
            // the lower right corner of the code
            // once the top is reached the index shifts 2 elements to the left and goes down again
            // until it reaches the bottom shifts 2 elements to the left again
            // repeat until yout hit
            if is_right {
                x_index -= 1;
            } else {
                x_index += 1;
                if is_y_shrinking {
                    y_index -= 1;
                } else {
                    y_index += 1;
                }
            }
            // always shift between left and right
            is_right = !is_right;
            // direction changes (down/up)
            // if y index hits the quiet zone move x to the left and reverse course
            if (y_index < 4 || y_index > self.output_data.len() - 1 - 4) && is_right {
                x_index -= 2;
                // change direction
                is_y_shrinking = !is_y_shrinking;
            }
            // avoid the timing pattern
            if x_index == 10 {
                x_index -= 1;
                if self.settings.debugging {
                    println!("avoided the timing pattern");
                }
            }

            // end loop if everything is done/left quiet zone is hit
            if x_index < 4 {
                break;
            }
            // println!("x: {}; y: {}", x_index, y_index);
        }
    }

    /// reads the text from self.settings and write it into the qr code
    pub fn read_and_write(&mut self) {
        // let width: usize = self.get_width();
        // let max_index: usize = width - 1;
        // get the data
        let data: String = self.settings.information.clone();
        if self.settings.debugging {
            println!("encoded data: {data} (length {})", data.len());
        }

        // get info in the error blocks
        let (error_blocks, mut all_blocks, tot_num_blocks, tot_num_codewords) =
            self.generate_error_blocks();

        // write all data into the error block vector
        let bit_vectors: Vec<MyBitVector> = self.write_data_into_vectors(&error_blocks);
        assert!(all_blocks.len() == bit_vectors.len());

        // convert the datavectors, so that they
        // also contain the error correction numbers
        self.generate_error_corrction(&error_blocks, &mut all_blocks, &bit_vectors);

        // all vectors get shuffeled around to spread all information across the qrcode
        let final_data_vect = self.shuffle_bit_vectors(
            &error_blocks,
            &all_blocks,
            tot_num_blocks,
            tot_num_codewords,
        );

        self.write_into_self(&final_data_vect);

        if self.get_settings().debugging {
            println!("after writing the actual data");
            print!("{self}");
        }
    }

    pub fn version_information(&mut self) {
        let version: u8 = self.version;
        // only version 7 or larger
        if (7..=40).contains(&version) {
            let bit_stream: u32 = version_info(version);
            // write into lower left
            let mut mask_left: u32 = 2_u32.pow(17);
            let y_left_start: usize = self.output_data.len() - 4 - 9;
            let mut x_index_left: usize = 3 + 6;
            let mut y_index_left: usize = y_left_start;
            // write data into field in the bottom left
            loop {
                // only write into elements reserved
                if self.role_data[x_index_left][y_index_left]
                    == SymbolRole::ReservedVersionInformation
                {
                    // write
                    if (bit_stream & mask_left) > 0 {
                        self.output_data[x_index_left][y_index_left] = SymbolStatus::LogicalTrue;
                    } else {
                        self.output_data[x_index_left][y_index_left] = SymbolStatus::LogicalFalse;
                    }
                } else {
                    println!("{self}");
                    panic!("tried to write into field that isn't reserved for version information (x: {x_index_left}, y: {y_index_left})" );
                }
                self.role_data[x_index_left][y_index_left] = SymbolRole::VersionInformation;

                // if mask is one the entire information was written
                if mask_left == 1 {
                    break;
                }
                // update mask
                mask_left /= 2;

                // update the indeces
                y_index_left -= 1;
                if y_index_left <= y_left_start - 3 {
                    x_index_left -= 1;
                    y_index_left = y_left_start;
                }
            }
            // write data into field in the top right
            let mut mask_right: u32 = 2_u32.pow(17);
            let x_left_start: usize = self.output_data.len() - 4 - 9;
            let mut x_index_right: usize = x_left_start;
            let mut y_index_right: usize = 3 + 6;
            loop {
                // only write into elements reserved
                if self.role_data[x_index_right][y_index_right]
                    == SymbolRole::ReservedVersionInformation
                {
                    // write
                    if (bit_stream & mask_right) > 0 {
                        self.output_data[x_index_right][y_index_right] = SymbolStatus::LogicalTrue;
                    } else {
                        self.output_data[x_index_right][y_index_right] = SymbolStatus::LogicalFalse;
                    }
                } else {
                    println!("{self}");
                    panic!("tried to write into field that isn't reserved for version information (x: {x_index_right}, y: {y_index_right})" );
                }
                self.role_data[x_index_right][y_index_right] = SymbolRole::VersionInformation;

                // if mask is one the entire information was written
                if mask_right == 1 {
                    break;
                }
                // update mask
                mask_right /= 2;

                // update the indeces
                x_index_right -= 1;
                if x_index_right <= x_left_start - 3 {
                    y_index_right -= 1;
                    x_index_right = x_left_start;
                }
            }
        }
        if self.get_settings().debugging {
            println!("after writing the version information");
            print!("{self}");
        }
    }

    fn apply_mask(&self, mask_number: u8) -> Self {
        assert!((0..=7).contains(&mask_number));
        let mut working_copy = self.clone();
        // iterate over qr code and apply the masking
        let smallest_index = 4;
        let biggest_index = self.output_data.len() - 4;
        // left to right
        for i in smallest_index..biggest_index {
            // top to bottom
            for j in smallest_index..biggest_index {
                // only mask data
                if self.role_data[i][j] == SymbolRole::EncodingRegion {
                    // remove qiet zone around qr code for masking purposes
                    // TODO figure out why i and j need to be flipped here in order for
                    // the masks to work properly (for i_mod and j_mod)
                    let i_mod = j - 4;
                    let j_mod = i - 4;
                    // apply mask according to mask_number
                    let match_bool = match mask_number {
                        0 => (i_mod + j_mod) % 2 == 0,
                        1 => i_mod % 2 == 0,
                        2 => j_mod % 3 == 0,
                        3 => (i_mod + j_mod) % 3 == 0,
                        4 => {
                            ((i_mod as f32 / 2.0) as usize + (j_mod as f32 / 3.0) as usize) % 2 == 0
                        }
                        5 => (i_mod * j_mod) % 2 + (i_mod * j_mod) % 3 == 0,
                        6 => ((i_mod * j_mod) % 2 + (i_mod * j_mod) % 3) % 2 == 0,
                        7 => ((i_mod + j_mod) % 2 + (i_mod * j_mod) % 3) % 2 == 0,
                        // cannot be reached due to assert earlier
                        _ => false,
                    };
                    // invert value if necessary
                    if match_bool {
                        working_copy.output_data[i][j] =
                            working_copy.output_data[i][j] ^ SymbolStatus::LogicalTrue;
                    }
                }
            }
        }
        working_copy
    }

    // calculates the penalty score for consecutive elements of the same color
    fn penalty_consecutive_same_color(&self) -> u32 {
        let mut penalty_consecuetive_same_color = 0;
        // multiple consecutive elements of the same color
        let low_index = 4;
        let high_index = self.output_data.len() - 4;
        // move top to bottom, left to right
        for x in low_index..high_index {
            let mut former_status = SymbolStatus::LogicalFalse;
            let mut consecutive_counter = 0;
            for y in low_index..high_index {
                if self.output_data[x][y] == former_status {
                    consecutive_counter += 1;
                    match consecutive_counter.cmp(&5) {
                        Ordering::Equal => penalty_consecuetive_same_color += 3,
                        Ordering::Greater => penalty_consecuetive_same_color += 1,
                        Ordering::Less => (),
                    }
                } else {
                    consecutive_counter = 0;
                    former_status = self.output_data[x][y];
                }
            }
        }
        // move left to right, top to bottom
        for y in low_index..high_index {
            let mut former_status = SymbolStatus::LogicalFalse;
            let mut consecutive_counter = 0;
            for x in low_index..high_index {
                if self.output_data[x][y] == former_status {
                    consecutive_counter += 1;
                    match consecutive_counter.cmp(&5) {
                        Ordering::Equal => penalty_consecuetive_same_color += 3,
                        Ordering::Greater => penalty_consecuetive_same_color += 1,
                        Ordering::Less => (),
                    }
                } else {
                    consecutive_counter = 0;
                    former_status = self.output_data[x][y];
                }
            }
        }
        penalty_consecuetive_same_color
    }

    // calculates the penalty score for 2x2 blocks of the same color
    fn penalty_color_blocks(&self) -> u32 {
        let mut penalty_color_blocks = 0;
        let low_index = 4;
        let high_index = self.output_data.len() - 4;
        // blocks of the same color
        for x in low_index..high_index - 1 {
            for y in low_index..high_index - 1 {
                // are all elements in a 2x2 box the same?
                if self.output_data[x][y] == self.output_data[x + 1][y]
                    && self.output_data[x][y] == self.output_data[x][y + 1]
                    && self.output_data[x][y] == self.output_data[x + 1][y + 1]
                {
                    penalty_color_blocks += 3;
                }
            }
        }
        penalty_color_blocks
    }

    // calculates the penalty score for possible finder patterns in the generated code
    fn penalty_finder_patterns(&self) -> u32 {
        let mut penalty_finder_patterns = 0;
        let low_index = 4;
        let high_index = self.output_data.len() - 4;
        // sequences that are similar to the finder pattern
        let pattern1 = [
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalFalse,
        ];
        let pattern2 = [
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalTrue,
        ];
        // move top to bottom, left to right
        for x in low_index..high_index {
            let mut element_vector = vec![];
            for y in low_index..high_index {
                element_vector.push(self.output_data[x][y]);
                // if longer than the pattern shorten the vector to the length of the pattern
                if element_vector.len() > 11 {
                    _ = element_vector.remove(0);
                }
                if element_vector.len() == 11
                    && (element_vector == pattern1 || element_vector == pattern2)
                {
                    penalty_finder_patterns += 40;
                }
            }
        }
        // move left to right, top to bottom
        for y in low_index..high_index {
            let mut element_vector = vec![];
            for x in low_index..high_index {
                element_vector.push(self.output_data[x][y]);
                // if longer than the pattern shorten the vector to the length of the pattern
                if element_vector.len() > 11 {
                    _ = element_vector.remove(0);
                }
                if element_vector.len() == 11
                    && (element_vector == pattern1 || element_vector == pattern2)
                {
                    penalty_finder_patterns += 40;
                }
            }
        }
        penalty_finder_patterns
    }

    // calculates the penalty score for the ratios of black and/or white elements
    fn penalty_color_ratios(&self) -> u32 {
        let mut penalty_color_ratios = 0;
        let low_index = 4;
        let high_index = self.output_data.len() - 4;
        // ratio of black to white elements
        let total_elements = (high_index - low_index) * (high_index - low_index);
        let mut total_black = 0;
        for x in low_index..high_index {
            for y in low_index..high_index {
                if self.output_data[x][y] == SymbolStatus::LogicalTrue {
                    total_black += 1;
                }
            }
        }
        let precentage_dark = (total_black as f32 / total_elements as f32) * 100.0;
        let dark_floor = (precentage_dark / 5.0).floor() * 5.0;
        let dark_result1 = (dark_floor - 50.0).abs() / 5.0;
        let dark_result2 = ((dark_floor + 5.0) - 50.0).abs() / 5.0;
        if dark_result1 >= dark_result2 {
            penalty_color_ratios += dark_result2 as u32 * 10;
        } else {
            penalty_color_ratios *= dark_result1 as u32 * 10;
        }
        penalty_color_ratios
    }

    /// calulates the penalty score of a qr code based on:
    /// consecutive elements of the same color,
    /// 2x2 blocks of the same color,
    /// possible finder patterns in the generated code
    /// and the ratios of black and/or white elements
    fn calculate_penalty(&self) -> u32 {
        // based on https://www.thonky.com/qr-code-tutorial/data-masking (14.03.2025)
        let mut total_score = 0;
        // condition #1
        total_score += self.penalty_consecutive_same_color();
        // condition #2
        total_score += self.penalty_color_blocks();
        // condition #3
        total_score += self.penalty_finder_patterns();
        // condition #4
        total_score += self.penalty_color_ratios();
        // summ of all penalties
        total_score
    }

    /// this function applies the mask with the lowest penalty
    /// to the qr code and applies/writes the format informaiton
    pub fn masking_format_information(&mut self) {
        let smallest_index = 4;
        let biggest_index = self.output_data.len() - 4 - 1;
        // create white qr code for debugging the masks
        let mut self_blank = self.clone();
        for x in 0..self.output_data.len() {
            for y in 0..self.output_data.len() {
                // 'paint' everything white
                self_blank.output_data[x][y] = SymbolStatus::LogicalFalse;
            }
        }
        // apply every mask to the qr code and select the mask with the lowest penalty
        let mut lowest_penalty_code = self.clone();
        let mut lowest_penalty_so_far = u32::MAX;
        let mut lowest_panalty_mask_number = 0;
        for mask_number in 0..8 {
            let current_masked = self.apply_mask(mask_number);
            let current_loss = current_masked.calculate_penalty();
            if current_loss < lowest_penalty_so_far {
                lowest_penalty_code = current_masked;
                lowest_penalty_so_far = current_loss;
                lowest_panalty_mask_number = mask_number;
            }
            if self.settings.debugging {
                print!(
                    "mask {mask_number} (penalty: {current_loss}):\n{}",
                    self_blank.apply_mask(mask_number)
                );
            }
        }

        // overwrite own data with the masked data
        self.output_data = lowest_penalty_code.output_data;

        // write the mask information into the qrcode
        // get the databits
        let mut data_bits: u8 = 0;
        data_bits |= match self.settings.error_level {
            ErrorLevel::L => 0b01 << 3,
            // value is 0, so do nothing
            ErrorLevel::M => 0b00 << 3,
            ErrorLevel::Q => 0b11 << 3,
            ErrorLevel::H => 0b10 << 3,
        };
        data_bits |= lowest_panalty_mask_number;
        let final_data_bits = information_sequences(data_bits);
        // write info into qrcode
        write_format_info!(self, smallest_index, biggest_index, final_data_bits);
        if self.get_settings().debugging {
            println!("after applying the mask and format information");
            print!("{self}");
            // additional info
            println!(
                "version: {}\nwidth: {}\ntext length: {}\nerror blocks:",
                self.get_version(),
                self.get_width(),
                self.get_settings().information.len()
            );
            for error_block in self.get_error_info() {
                println!("    {error_block:?}");
            }
        }
    }

    /// prints debugging output of the output data
    pub fn print_data(&self) {
        println!("{:?}", self.output_data);
    }

    // only used in tests
    pub fn _get_data(&self) -> Vec<Vec<SymbolStatus>> {
        self.output_data.clone()
    }
}

impl Display for QRData {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.settings.debugging {
            writeln!(f, "{BRIGHTMAGENTA}Uninitialised{COLORSTOP}")?;
            writeln!(f, "{BLACK}LogicalTrue{COLORSTOP}")?;
            writeln!(f, "{BRIGHTWHITE}LogicalFalse{COLORSTOP}")?;
        }
        // put every data element in formatter
        for row in 0..self.output_data.len() {
            for column in 0..self.output_data[row].len() {
                match self.output_data[column][row] {
                    // color output utilising with ANSI
                    // 105 => bright magenta
                    SymbolStatus::Uninitialised => write!(f, "{BRIGHTMAGENTA}   {COLORSTOP}")?,

                    // 40 => Black
                    SymbolStatus::LogicalTrue => write!(f, "{BLACK}   {COLORSTOP}")?,
                    // 107 => Bright White
                    SymbolStatus::LogicalFalse => write!(f, "{BRIGHTWHITE}   {COLORSTOP}")?,
                }
            }
            // don't forget the newlines
            writeln!(f)?;
        }
        // if debugging print the role data as well
        if self.settings.debugging {
            writeln!(f, "{BRIGHTMAGENTA}Uninitialised{COLORSTOP}")?;
            writeln!(f, "{BRIGHTWHITE}QuietZone{COLORSTOP}")?;
            writeln!(f, "{WHITE}FinderPattern{COLORSTOP}")?;
            writeln!(f, "{BRIGHTBLACK}AlignmentPattern{COLORSTOP}")?;
            writeln!(f, "{BRIGHTRED}TimingPattern{COLORSTOP}")?;
            writeln!(f, "{RED}Separator{COLORSTOP}")?;
            writeln!(f, "{MAGENTA}ReservedFormatInformation{COLORSTOP}")?;
            writeln!(f, "{BRIGHTYELLOW}FormatInformation{COLORSTOP}")?;
            writeln!(f, "{BLUE}ReservedVersionInformation{COLORSTOP}")?;
            writeln!(f, "{BRIGHTCYAN}VersionInformation{COLORSTOP}")?;
            writeln!(f, "{BRIGHTBLUE}EncodingRegion{COLORSTOP}")?;
            writeln!(f, "{BLACK}AlwaysTrue{COLORSTOP}")?;
            for row in 0..self.role_data.len() {
                for column in 0..self.role_data[row].len() {
                    match self.role_data[column][row] {
                        // for now all magenta; rest to be implemented
                        SymbolRole::Uninitialised => write!(f, "{BRIGHTMAGENTA}   {COLORSTOP}")?,
                        SymbolRole::QuietZone => write!(f, "{BRIGHTWHITE}   {COLORSTOP}")?,
                        SymbolRole::FinderPattern => write!(f, "{WHITE}   {COLORSTOP}")?,
                        SymbolRole::AlignmentPattern => write!(f, "{BRIGHTBLACK}   {COLORSTOP}")?,
                        SymbolRole::TimingPattern => write!(f, "{BRIGHTRED}   {COLORSTOP}")?,
                        SymbolRole::Separator => write!(f, "{RED}   {COLORSTOP}")?,
                        SymbolRole::ReservedFormatInformation => {
                            write!(f, "{MAGENTA}   {COLORSTOP}")?
                        }
                        SymbolRole::FormatInformation => write!(f, "{BRIGHTYELLOW}   {COLORSTOP}")?,
                        SymbolRole::ReservedVersionInformation => {
                            write!(f, "{BLUE}   {COLORSTOP}")?
                        }
                        SymbolRole::VersionInformation => write!(f, "{BRIGHTCYAN}   {COLORSTOP}")?,
                        SymbolRole::EncodingRegion => write!(f, "{BRIGHTBLUE}   {COLORSTOP}")?,
                        SymbolRole::AlwaysTrue => write!(f, "{BLACK}   {COLORSTOP}")?,
                    }
                }
                // don't forget the newlines
                writeln!(f,)?;
            }
        }
        // end
        write!(f, "")
    }
}

mod tests {
    #[test]
    fn test_my_vect() {
        use super::MyBitVector;
        let mut test_vec: MyBitVector = MyBitVector::new_with_capacity(2);
        test_vec.push(0b0000_0100, 4);
        test_vec.push(0b0101_0101, 8);
        println!(
            "created: {:?}: {:?}",
            test_vec.data,
            vec![0b0100_0101, 0b0101_0000]
        );
        assert_eq!(test_vec.data, vec![0b0100_0101, 0b0101_0000]);
    }

    #[test]
    fn test_my_vect_padding() {
        use super::MyBitVector;
        let mut test_vector = MyBitVector::new_with_capacity(10);
        test_vector.push(0b1010, 4);
        test_vector.push(0b01010101, 8);
        test_vector.pad_empty_rest();
        for byte in test_vector.data.clone() {
            print!("{byte:#x} ");
        }
        assert_eq!(
            test_vector.data,
            vec![0xa5, 0x50, 0xec, 0x11, 0xec, 0x11, 0xec, 0x11, 0xec, 0x11]
        );
    }

    #[test]
    fn symbol_status_xor() {
        use crate::standard_qr_code::qr_struct::SymbolStatus;
        assert_eq!(
            SymbolStatus::LogicalFalse ^ SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalFalse
        );
        assert_eq!(
            SymbolStatus::LogicalFalse ^ SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalTrue
        );
        assert_eq!(
            SymbolStatus::LogicalTrue ^ SymbolStatus::LogicalFalse,
            SymbolStatus::LogicalTrue
        );
        assert_eq!(
            SymbolStatus::LogicalTrue ^ SymbolStatus::LogicalTrue,
            SymbolStatus::LogicalFalse
        );
    }
}

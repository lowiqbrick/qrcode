use crate::polynomials::{Indeterminate, Polynomial};
use crate::standard_qr_code::version_constants::alignment_pattern_data;
use crate::{standard_qr_code::utils::get_verison_info, Settings};
use std::fmt::{Display, Formatter, Result};

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
            let current_bit: u8;
            // get the value of the bit in question
            current_bit = match current_bit_read {
                0 => value & 0b0000_0001,
                1 => (value & 0b0000_0010) >> 1,
                2 => (value & 0b0000_0100) >> 2,
                3 => (value & 0b0000_1000) >> 3,
                4 => (value & 0b0001_0000) >> 4,
                5 => (value & 0b0010_0000) >> 5,
                6 => (value & 0b0100_0000) >> 6,
                7 => (value & 0b1000_0000) >> 7,
                _ => {
                    panic!("tried to read bit {} of an u8", current_bit_read);
                }
            };
            // update for next loop
            if current_bit_read != 0 {
                current_bit_read -= 1;
            }
            // which bit of the byte index should be written into
            let current_bit_write: u32 = (7 - (index % 8)) % 8;
            // write the value into the data vector
            // round down aka cutt of all decimals of a f32 by converting it to an u-type
            let byte_index: u16 = (self.curr_position as f32 / 8.0) as u16;
            // println!("{}: {}", byte_index, self.capacity);
            assert!(byte_index < self.capacity);
            match current_bit_write {
                0 => {
                    self.data[byte_index as usize] = self.data[byte_index as usize] | current_bit;
                }
                1 => {
                    self.data[byte_index as usize] =
                        self.data[byte_index as usize] | (current_bit << 1);
                }
                2 => {
                    self.data[byte_index as usize] =
                        self.data[byte_index as usize] | (current_bit << 2);
                }
                3 => {
                    self.data[byte_index as usize] =
                        self.data[byte_index as usize] | (current_bit << 3);
                }
                4 => {
                    self.data[byte_index as usize] =
                        self.data[byte_index as usize] | (current_bit << 4);
                }
                5 => {
                    self.data[byte_index as usize] =
                        self.data[byte_index as usize] | (current_bit << 5);
                }
                6 => {
                    self.data[byte_index as usize] =
                        self.data[byte_index as usize] | (current_bit << 6);
                }
                7 => {
                    self.data[byte_index as usize] =
                        self.data[byte_index as usize] | (current_bit << 7);
                }
                _ => panic!("attempted to write to bit {} in u8", current_bit_write),
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
#[derive(Debug, Clone, Copy)]

pub enum SymbolStatus {
    /// white symbol in qr code; false
    LogicalFalse,
    /// black symbol in qr code; true
    LogicalTrue,
    /// the symbol hasn't been initialised
    Uninitialised,
}

/// represents the role of symbol inside the qr code
#[derive(Debug, PartialEq)]
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
}

/// contains error correction block information
#[derive(Clone, Copy, Debug)]
pub struct ErrorBlockInfo {
    /// amount of this block in this version
    pub num_block: u8,
    /// total length of the block
    pub total_block_len: u8,
    /// data length in this block
    pub num_data_bytes: u8,
    /// number of error correction bytes in this block
    pub num_error_bytes: u8,
}

impl ErrorBlockInfo {
    pub fn new(num_block: u8, total_block_len: u8, num_data_bytes: u8) -> ErrorBlockInfo {
        ErrorBlockInfo {
            num_block,
            total_block_len,
            num_data_bytes,
            num_error_bytes: total_block_len - num_data_bytes,
        }
    }
}

/// encomposes all data required to generate a qr code
#[derive(Debug)]
pub struct QRData {
    output_data: Vec<Vec<SymbolStatus>>,
    role_data: Vec<Vec<SymbolRole>>,
    version: u8,
    error_blocks: Vec<ErrorBlockInfo>,
    width: usize,
    settings: Settings,
}

impl QRData {
    /// generate the data
    pub fn new(input: Settings) -> QRData {
        let (version, error_blocks) =
            match get_verison_info(input.information.len(), input.error_level) {
                Ok(result) => result,
                Err(msg) => {
                    eprintln!("{}", msg);
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
            error_blocks: error_blocks,
            width,
            settings: input,
        }
    }

    /// returns the version
    pub fn get_version(&self) -> u8 {
        self.version as u8
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
        let width = self.output_data.len();
        for x in 0..width {
            for y in 0..width {
                if x <= 3 || x >= width - 4 || y <= 3 || y >= width - 4 {
                    self.output_data[x][y] = SymbolStatus::LogicalFalse;
                    self.role_data[x][y] = SymbolRole::QuietZone;
                }
            }
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
                        if (x + 1) % 2 == 1 {
                            self.output_data[x][y] = SymbolStatus::LogicalTrue;
                        } else {
                            self.output_data[x][y] = SymbolStatus::LogicalFalse;
                        }
                    }
                    self.role_data[x][y] = SymbolRole::TimingPattern;
                }
            }
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
                        }
                    }
                }
            }
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
        let mut alignment_information: (u8, Vec<u8>) = alignment_pattern_data(self.version);
        // increase the centres of the alignment patterns by four to compensate for the quiet zone
        // in the indices
        for alignment_centre in alignment_information.1.iter_mut() {
            *alignment_centre += 4;
        }
        // first and last element of the alignment centres to avoid drawing into finder patterns
        let lower_end: u8 = alignment_information.1[0];
        let upper_end: u8 = alignment_information.1[alignment_information.1.len() - 1];
        println!("alignment lower: {} upper: {}", lower_end, upper_end);
        // go over all qr code elements
        for x in 0..width {
            for y in 0..width {
                // go over all rq code elements
                for x_align in alignment_information.1.iter() {
                    for y_align in alignment_information.1.iter() {
                        // don't draw on finder patterns
                        if !((*x_align == lower_end && *y_align == lower_end)
                            || (*x_align == upper_end && *y_align == lower_end)
                            || (*x_align == lower_end && *y_align == upper_end))
                        {
                            let x_diff: i32 = *x_align as i32 - x as i32;
                            let y_diff: i32 = *y_align as i32 - y as i32;
                            if (x_diff <= 2 && x_diff >= -2) && (y_diff <= 2 && y_diff >= -2) {
                                self.output_data[x][y] =
                                    alignment_pattern[(x_diff + 2) as usize][(y_diff + 2) as usize];
                                self.role_data[x][y] = SymbolRole::AlignmentPattern;
                            }
                        }
                    }
                }
            }
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
                    if x >= 4
                        && x <= 9
                        && (y <= (width - 4 - 7 - 1))
                        && (y >= (width - 4 - 7 - 1 - 3))
                        && self.role_data[x][y] != SymbolRole::Separator
                    {
                        self.role_data[x][y] = SymbolRole::ReservedVersionInformation;
                    }
                    if (x <= (width - 4 - 7 - 1))
                        && (x >= (width - 4 - 7 - 1 - 3))
                        && y >= 4
                        && y <= 9
                        && self.role_data[x][y] != SymbolRole::Separator
                    {
                        self.role_data[x][y] = SymbolRole::ReservedVersionInformation;
                    }
                }
            }
        }
    }

    /// reads the text from self.settings and write it into the qr code
    pub fn read_and_write(&mut self) {
        let width: usize = self.get_width();
        let max_index: usize = width - 1;
        // get the data
        let data: String = self.settings.information.clone();
        // get info on the error blocks
        let error_blocks: Vec<ErrorBlockInfo> = self.error_blocks.clone();
        // println!("error blocks: {:?}", error_blocks);
        // create vector to contain the errorblock data
        let mut all_blocks: Vec<Vec<u8>> = vec![];
        for block in error_blocks.iter() {
            for _ in 0..block.num_block {
                all_blocks.push(vec![]);
            }
        }

        // write all data into the error block vector
        // create MyBitVectors to write data into
        let mut bit_vectors: Vec<MyBitVector> = vec![];
        for block in error_blocks.iter() {
            for _ in 0..block.num_block {
                // println!("num data bytes: {}", block.num_data_bytes);
                bit_vectors.push(MyBitVector::new_with_capacity(block.num_data_bytes.into()));
            }
        }
        // println!("bit vectors: {:?}", bit_vectors);
        // write mode bits into data
        assert!(bit_vectors.len() >= 1);
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
            } else {
                if remaining_capacity == 4 {
                    // write what fits into vector
                    bit_vectors[bit_vector_index]
                        .push((char as u8 & 0b1111_0000) >> 4, remaining_capacity as u8);
                    // increase index
                    bit_vector_index += 1;
                    // println!(
                    //     "increased vector index from {} to {}",
                    //     bit_vector_index - 1,
                    //     bit_vector_index
                    // );
                    assert!(bit_vector_index < bit_vectors.len());
                    bit_vectors[bit_vector_index]
                        .push(char as u8 & 0b0000_1111, remaining_capacity as u8);
                } else {
                    panic!("remaining capacity wasn't 4, but {}", remaining_capacity);
                }
            }
        }
        assert!(all_blocks.len() == bit_vectors.len());
        // convert the datavectors, so that they
        // go through all blocks
        let mut vector_index: u8 = 0;
        for block in error_blocks.iter() {
            // go through all block repetitions
            for _ in 0..block.num_block {
                // go through all data vectors
                let raw_polynomial: Polynomial =
                    Polynomial::from(bit_vectors[vector_index as usize].get_data());
                let difference_degree: u8 = block.num_error_bytes;
                let mut divisor: Polynomial = Polynomial::new(vec![Indeterminate::new(1, 1)])
                    * Polynomial::new(vec![Indeterminate::new(-1, 0)]);
                // build (x - 1) * (x - 2) * ...
                for degree in 2..difference_degree + 1 {
                    divisor = divisor
                        * Polynomial::new(vec![
                            Indeterminate::new(1, 1),
                            Indeterminate::new((degree as i16 * -1) as i16, 0),
                        ]);
                    println!(
                        "{} and divisor {}",
                        Polynomial::new(vec![
                            Indeterminate::new(1, 1),
                            Indeterminate::new((degree as i16 * -1) as i16, 0),
                        ]),
                        divisor
                    );
                }
                vector_index += 1;
            }
        }
        // println!(
        //     "bit vectors (len {}): {:?}",
        //     bit_vectors[0].data.len(),
        //     bit_vectors
        // );
    }
}

impl Display for QRData {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.settings.debugging {
            writeln!(f, "{}Uninitialised{}", BRIGHTMAGENTA, COLORSTOP)?;
            writeln!(f, "{}LogicalTrue{}", BLACK, COLORSTOP)?;
            writeln!(f, "{}LogicalFalse{}", BRIGHTWHITE, COLORSTOP)?;
        }
        // put every data element in formatter
        for row in 0..self.output_data.len() {
            for column in 0..self.output_data[row].len() {
                match self.output_data[row][column] {
                    // color output utilising with ANSI
                    // 105 => bright magenta
                    SymbolStatus::Uninitialised => {
                        write!(f, "{}{}{}", BRIGHTMAGENTA, "  ", COLORSTOP)?
                    }

                    // 40 => Black
                    SymbolStatus::LogicalTrue => write!(f, "{}{}{}", BLACK, "  ", COLORSTOP)?,
                    // 107 => Bright White
                    SymbolStatus::LogicalFalse => {
                        write!(f, "{}{}{}", BRIGHTWHITE, "  ", COLORSTOP)?
                    }
                }
            }
            // don't forget the newlines
            writeln!(f)?;
        }
        // if debugging print the role data as well
        if self.settings.debugging {
            writeln!(f, "{}Uninitialised{}", BRIGHTMAGENTA, COLORSTOP)?;
            writeln!(f, "{}QuietZone{}", BRIGHTWHITE, COLORSTOP)?;
            writeln!(f, "{}FinderPattern{}", WHITE, COLORSTOP)?;
            writeln!(f, "{}AlignmentPattern{}", BRIGHTBLACK, COLORSTOP)?;
            writeln!(f, "{}TimingPattern{}", BRIGHTRED, COLORSTOP)?;
            writeln!(f, "{}Separator{}", RED, COLORSTOP)?;
            writeln!(f, "{}ReservedFormatInformation{}", MAGENTA, COLORSTOP)?;
            writeln!(f, "{}FormatInformation{}", BRIGHTYELLOW, COLORSTOP)?;
            writeln!(f, "{}ReservedVersionInformation{}", BLUE, COLORSTOP)?;
            writeln!(f, "{}VersionInformation{}", BRIGHTCYAN, COLORSTOP)?;
            writeln!(f, "{}EncodingRegion{}", BRIGHTBLUE, COLORSTOP)?;
            for row in 0..self.role_data.len() {
                for column in 0..self.role_data[row].len() {
                    match self.role_data[row][column] {
                        // for now all magenta; rest to be implemented
                        SymbolRole::Uninitialised => {
                            write!(f, "{}{}{}", BRIGHTMAGENTA, "  ", COLORSTOP)?
                        }
                        SymbolRole::QuietZone => write!(f, "{}{}{}", BRIGHTWHITE, "  ", COLORSTOP)?,
                        SymbolRole::FinderPattern => write!(f, "{}{}{}", WHITE, "  ", COLORSTOP)?,
                        SymbolRole::AlignmentPattern => {
                            write!(f, "{}{}{}", BRIGHTBLACK, "  ", COLORSTOP)?
                        }
                        SymbolRole::TimingPattern => {
                            write!(f, "{}{}{}", BRIGHTRED, "  ", COLORSTOP)?
                        }
                        SymbolRole::Separator => write!(f, "{}{}{}", RED, "  ", COLORSTOP)?,
                        SymbolRole::ReservedFormatInformation => {
                            write!(f, "{}{}{}", MAGENTA, "  ", COLORSTOP)?
                        }
                        SymbolRole::FormatInformation => {
                            write!(f, "{}{}{}", BRIGHTYELLOW, "  ", COLORSTOP)?
                        }
                        SymbolRole::ReservedVersionInformation => {
                            write!(f, "{}{}{}", BLUE, "  ", COLORSTOP)?
                        }
                        SymbolRole::VersionInformation => {
                            write!(f, "{}{}{}", BRIGHTCYAN, "  ", COLORSTOP)?
                        }
                        SymbolRole::EncodingRegion => {
                            write!(f, "{}{}{}", BRIGHTBLUE, "  ", COLORSTOP)?
                        }
                    }
                }
                // don't forget the newlines
                write!(f, "{}", "\n")?;
            }
        }
        // end
        write!(f, "{}", "")
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
}

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
const BRIGHTRED: &str = "\x1b[37;101m";
const BRIGHTYELLOW: &str = "\x1b[30;103m";
const BRIGHTBLUE: &str = "\x1b[30;104m";
const BRIGHTCYAN: &str = "\x1b[30;106m";

/// constant for byte mode indicator
#[allow(dead_code)]
const BYTEMODEINDICATOR: u8 = 0b0100;

/// represents the qr code symbols statuses, which are uninitialised, true false
#[allow(dead_code)]

pub enum SymbolStatus {
    /// white symbol in qr code; false
    LogicalFalse,
    /// black symbol in qr code; true
    LogicalTrue,
    /// the symbol hasn't been initialised
    Uninitialised,
}

/// represents the role of symbol inside the qr code
#[allow(dead_code)]
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
    /// Version information; version 7 or larger
    VersionInformation,
    /// Format Information; error correction level and mask pattern
    FormatInformation,
}

/// contains error correction block information
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct ErrorBlockInfo {
    /// amount of this block in this version
    pub num_block: u8,
    /// total length of the block
    total_block_len: u8,
    /// data length in this block
    pub num_data_bytes: u8,
    /// number of error correction bytes in this block
    num_error_bytes: u8,
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
#[allow(dead_code)]
pub struct QRData {
    output_data: Vec<Vec<SymbolStatus>>,
    role_data: Vec<Vec<SymbolRole>>,
    version: u8,
    error_blocks: Vec<ErrorBlockInfo>,
    width: usize,
    settings: Settings,
}

#[allow(dead_code)]
impl QRData {
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

    pub fn get_version(&self) -> u8 {
        self.version as u8
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
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
                        write!(f, "{}{}{}", BRIGHTMAGENTA, " ", COLORSTOP)?
                    }

                    // 40 => Black
                    SymbolStatus::LogicalTrue => write!(f, "{}{}{}", BLACK, " ", COLORSTOP)?,
                    // 107 => Bright White
                    SymbolStatus::LogicalFalse => write!(f, "{}{}{}", BRIGHTWHITE, " ", COLORSTOP)?,
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
            writeln!(f, "{}FormatInformation{}", BRIGHTYELLOW, COLORSTOP)?;
            writeln!(f, "{}VersionInformation{}", BRIGHTCYAN, COLORSTOP)?;
            writeln!(f, "{}EncodingRegion{}", BRIGHTBLUE, COLORSTOP)?;
            for row in 0..self.role_data.len() {
                for column in 0..self.role_data[row].len() {
                    match self.role_data[row][column] {
                        // for now all magenta; rest to be implemented
                        SymbolRole::Uninitialised => {
                            write!(f, "{}{}{}", BRIGHTMAGENTA, " ", COLORSTOP)?
                        }
                        SymbolRole::QuietZone => write!(f, "{}{}{}", BRIGHTWHITE, " ", COLORSTOP)?,
                        SymbolRole::FinderPattern => write!(f, "{}{}{}", WHITE, " ", COLORSTOP)?,
                        SymbolRole::AlignmentPattern => {
                            write!(f, "{}{}{}", BRIGHTBLACK, " ", COLORSTOP)?
                        }
                        SymbolRole::TimingPattern => {
                            write!(f, "{}{}{}", BRIGHTRED, " ", COLORSTOP)?
                        }
                        SymbolRole::Separator => write!(f, "{}{}{}", RED, " ", COLORSTOP)?,
                        SymbolRole::FormatInformation => {
                            write!(f, "{}{}{}", BRIGHTYELLOW, " ", COLORSTOP)?
                        }
                        SymbolRole::VersionInformation => {
                            write!(f, "{}{}{}", BRIGHTCYAN, " ", COLORSTOP)?
                        }
                        SymbolRole::EncodingRegion => {
                            write!(f, "{}{}{}", BRIGHTBLUE, " ", COLORSTOP)?
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

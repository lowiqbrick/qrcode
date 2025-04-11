use std::fmt::{self, Display};

use clap::{Parser, Subcommand, ValueEnum};

/// indicates the level of error correction
/// desired from the generated QR Code
#[derive(Debug, Subcommand, Clone, ValueEnum, Copy, PartialEq)]
pub enum ErrorLevel {
    /// approx. 7% correction
    L,
    /// approx. 15% correction
    M,
    /// approx. 25% correction
    Q,
    /// approx. 30% correction
    H,
}

impl Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorLevel::L => write!(f, "l"),
            ErrorLevel::M => write!(f, "m"),
            ErrorLevel::Q => write!(f, "q"),
            ErrorLevel::H => write!(f, "h"),
        }
    }
}

/// stores the to be encoded text and the
/// error correction level
#[derive(Parser, Debug, Clone)]
#[clap(about, long_about = None)]
pub struct Settings {
    /// the text to be incoded into the qr code
    pub information: String,
    /// specifies the error correction level of the generated qr code
    #[clap(default_value_t = ErrorLevel::L)]
    pub error_level: ErrorLevel,
    /// Print debugging info to the terminal, while generating the code
    #[arg(short, long)]
    pub debugging: bool,
}

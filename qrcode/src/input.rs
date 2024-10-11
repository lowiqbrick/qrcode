use clap::{Parser, Subcommand, ValueEnum};

/// indicates the level of error correction
/// desired from the generated QR Code
#[derive(Debug, Subcommand, Clone, ValueEnum)]
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

impl ToString for ErrorLevel {
    fn to_string(&self) -> String {
        match self {
            ErrorLevel::L => String::from("l"),
            ErrorLevel::M => String::from("m"),
            ErrorLevel::Q => String::from("q"),
            ErrorLevel::H => String::from("h"),
        }
    }
}

/// stores the to be encoded text and the
/// error correction level
#[derive(Parser, Debug, Clone)]
#[clap(about, long_about = None)]
pub struct Settings {
    /// the text to be incoded into the qr code
    information: String,
    /// specifies the error correction level of the generated qr code
    #[clap(default_value_t = ErrorLevel::L)]
    error_level: ErrorLevel,
    /// Print debugging info to the terminal, while generating the code
    #[arg(short, long)]
    debugging: bool,
}

mod input;
mod standard_qr_code;
use clap::Parser;
use input::*;
use standard_qr_code::qr_code;

fn main() {
    // get environment variables
    let settings: Settings = Settings::parse();
    qr_code(settings);
}

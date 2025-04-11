/// this program is supposed to be handed a string which gets converted into
/// a QR code following norm ISO/IEC CD 18004
mod input;
mod polynomials;
mod standard_qr_code;
use clap::Parser;
use input::*;
use standard_qr_code::qr_code;

fn main() {
    // get environment variables
    let settings: Settings = Settings::parse();
    qr_code(settings.clone());
}

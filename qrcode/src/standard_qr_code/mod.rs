use crate::input::Settings;
mod qr_struct;
mod utils;
mod version_constants;
use crate::standard_qr_code::qr_struct::QRData;

pub fn qr_code(input: Settings) {
    // all text is assumed to be encoded in byte mode
    if input.debugging {
        println!("{:?}", input);
    }
    // struct for all data
    let mut qrdata: QRData = QRData::new(input);
    // fill in everything required
    qrdata.quiet_zone();
    qrdata.finders();
    qrdata.timing_pattern();
    if qrdata.get_settings().debugging {
        print!("{}", qrdata);
    }
}

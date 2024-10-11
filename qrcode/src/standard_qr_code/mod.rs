use crate::input::Settings;
mod qr_struct;
mod utils;
use crate::standard_qr_code::qr_struct::QRData;

pub fn qr_code(input: Settings) {
    // all text is assumed to be encoded in byte mode
    if input.debugging {
        println!("{:?}", input);
    }
    // struct for all data
    let qrdata: QRData = QRData::new(input);
    if qrdata.get_settings().debugging {
        println!("{}", qrdata);
    }
}

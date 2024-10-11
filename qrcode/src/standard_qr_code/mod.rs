use crate::input::Settings;
mod utils;
use crate::standard_qr_code::utils::get_verison_info;

pub fn qr_code(input: Settings) {
    // all text is assumed to be encoded in byte mode
    if input.debugging {
        println!("test {:?}", input);
        let (version_no, capacity) = get_verison_info(input.information.len());
        // calculate width of the code
        // width = 17 + 4 * <version number>
        let width: usize = 17 + 4 * version_no;
        // size of the code with the quiet zone
        // quiet zone: 4 square whide border around the qr code
        let width_quite_zone: usize = width + (2 * 8);
    }
}

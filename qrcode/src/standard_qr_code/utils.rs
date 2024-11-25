use std::u16;

use crate::input::ErrorLevel;
use crate::standard_qr_code::version_constants::get_error_block_info;

use super::qr_struct::ErrorBlockInfo;

/// takes text length and error correction level and returns the reqired version
/// and the total databytes that can be stored in it
pub fn get_verison_info(
    text_length: usize,
    error_level: ErrorLevel,
) -> Result<(u8, Vec<ErrorBlockInfo>), String> {
    // increase the text length by one to account for the QR code symbols
    let text_length_complete: usize = text_length + 1;
    // check the length of the text for being to long
    if text_length_complete > (3706 - 750) {
        eprintln!("text is way to long for a qr code; biggest possible is 2956 characters");
        panic!();
    }
    let search_length: u16 = text_length_complete as u16;
    let all_info: Vec<(u8, u16, Vec<(ErrorLevel, Vec<ErrorBlockInfo>)>)> = get_error_block_info();
    // look for the fitting version
    for version in all_info {
        // does this one potentally fit the text
        if search_length <= version.1 {
            // does the error level fit the text
            for (error_enum, block_vector) in version.2 {
                // does the level fit
                if error_enum == error_level {
                    let mut current_length: u16 = 0;
                    for info_block in block_vector.clone() {
                        current_length +=
                            info_block.num_block as u16 * info_block.num_data_bytes as u16;
                    }
                    // if the length that can be fit is larger or equal to the text length
                    // select this information for return
                    if current_length >= search_length {
                        return Ok((version.0, block_vector.clone()));
                    }
                }
            }
        }
    }
    let return_result: Result<(u8, Vec<ErrorBlockInfo>), String> =
        Err(String::from("text length or error level invalid"));
    return_result
}

/// takes a version number and returns the amount of bits in the character code indicator
#[allow(dead_code)]
pub fn num_bits_character_code_indicator(version: u8) -> u8 {
    match version {
        1..=9 => return 8,
        10..=26 => return 16,
        27..=40 => return 16,
        _ => {
            eprintln!("invlaid version number for bits in character code indicator");
            panic!();
        }
    }
}

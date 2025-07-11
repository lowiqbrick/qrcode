use crate::input::Settings;
mod generation_tests;
mod qr_struct;
mod utils;
mod version_constants;
use crate::standard_qr_code::qr_struct::QRData;

pub fn qr_code(input: Settings) -> QRData {
    // all text is assumed to be encoded in byte mode
    if input.debugging {
        println!("{input:?}");
    }
    // struct for all data
    let mut qrdata: QRData = QRData::new(input);
    // fill in everything required

    // draw white elements around the qrcode to
    // separate it visualy from the background
    qrdata.quiet_zone();

    // draw the finding patterns to determine rotation of the qrcode
    qrdata.finders();

    // draw a zebra pattern between the finding patterns,
    // so the scale between elemnts can be determined
    qrdata.timing_pattern();

    // draw a white border around the finder patterns to separate them
    // visualy from the rest of the qrcode
    qrdata.separators();

    // mark elements for format information,
    // so they won't get overwritten
    qrdata.reserve_format_information();

    // draw elements into qrcode to detect and account for
    // an uneven surface underneath the qrcode
    qrdata.draw_alignment_pattern();

    // mark elements for version information,
    // so they won't get overwritten
    qrdata.reserve_version_information();

    // after all preparations are done process and write the data
    qrdata.read_and_write();

    // write the version information

    qrdata.version_information();

    // apply a mask onto the code, so that the code has
    // no major white and/or black spots
    qrdata.masking_format_information();
    // actually display the qrcode, if not in debugging mode
    if !qrdata.get_settings().debugging {
        println!("{qrdata}");
    } else {
        println!("printing the qrcodes raw data:");
        qrdata.print_data();
    }
    qrdata
}

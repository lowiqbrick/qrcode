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
    if qrdata.get_settings().debugging {
        println!("default initialisation of the data");
        print!("{}", qrdata);
    }
    qrdata.quiet_zone();
    if qrdata.get_settings().debugging {
        println!("drew quiet zone");
        print!("{}", qrdata);
    }
    qrdata.finders();
    if qrdata.get_settings().debugging {
        println!("drew finding patterns");
        print!("{}", qrdata);
    }
    qrdata.timing_pattern();
    if qrdata.get_settings().debugging {
        println!("drew timing patterns");
        print!("{}", qrdata);
    }
    qrdata.separators();
    if qrdata.get_settings().debugging {
        println!("drew separators");
        print!("{}", qrdata);
    }
    qrdata.reserve_format_information();
    if qrdata.get_settings().debugging {
        println!("after reserfing place for format information");
        print!("{}", qrdata);
    }
    qrdata.draw_alignment_pattern();
    if qrdata.get_settings().debugging {
        println!("after reserfing place for format information");
        print!("{}", qrdata);
    }
    qrdata.reserve_version_information();
    if qrdata.get_settings().debugging {
        println!("after reserfing place for format information");
        print!("{}", qrdata);
    }
    // after all preparations are done process and write the data
    qrdata.read_and_write();
    if qrdata.get_settings().debugging {
        // additional info
        println!(
            "version: {}\nwidth: {}\ntext length: {}\nerror blocks:",
            qrdata.get_version(),
            qrdata.get_width(),
            qrdata.get_settings().information.len()
        );
        for error_block in qrdata.get_error_info() {
            println!("    {:?}", error_block);
        }
    }
}

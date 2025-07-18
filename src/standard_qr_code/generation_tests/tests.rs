#[test]
fn generated_output_basics() {
    use crate::input::{ErrorLevel, Settings};
    use crate::qr_code;
    use crate::standard_qr_code::{
        generation_tests::test_data,
        qr_struct::SymbolStatus::{self},
    };

    let settings = Settings {
        information: String::from("Hello, world! 123"),
        error_level: ErrorLevel::M,
        debugging: false,
    };
    let qrdata = qr_code(settings)._get_data();
    let test_data = test_data::_test_data_basics();
    let mut test_wrapper_vector: Vec<Vec<SymbolStatus>> = vec![];
    for line in test_data.iter() {
        test_wrapper_vector.push(line.to_vec());
    }
    // check if two outputs are the same
    assert_eq!(test_wrapper_vector, qrdata);
}

#[test]
fn generated_output_byte_shuffling() {
    use crate::input::{ErrorLevel, Settings};
    use crate::qr_code;
    use crate::standard_qr_code::{
        generation_tests::test_data,
        qr_struct::SymbolStatus::{self},
    };

    let settings = Settings {
        information: String::from("Example string to test qr code program for byte shuffling"),
        error_level: ErrorLevel::Q,
        debugging: false,
    };
    let qrdata = qr_code(settings)._get_data();
    let test_data = test_data::_test_data_byte_shuffling();
    let mut test_wrapper_vector: Vec<Vec<SymbolStatus>> = vec![];
    for line in test_data.iter() {
        test_wrapper_vector.push(line.to_vec());
    }
    // check if two outputs are the same
    assert_eq!(test_wrapper_vector, qrdata);
}

#[test]
fn generated_output_version_information() {
    use crate::input::{ErrorLevel, Settings};
    use crate::qr_code;
    use crate::standard_qr_code::{
        generation_tests::test_data,
        qr_struct::SymbolStatus::{self},
    };

    let settings = Settings {
        information: String::from("This example is testing the version information, which appears in versions 7 or higher."),
        error_level: ErrorLevel::Q,
        debugging: false,
    };
    let qrdata = qr_code(settings)._get_data();
    let test_data = test_data::_test_data_version_information();
    let mut test_wrapper_vector: Vec<Vec<SymbolStatus>> = vec![];
    for line in test_data.iter() {
        test_wrapper_vector.push(line.to_vec());
    }
    // check if two outputs are the same
    assert_eq!(test_wrapper_vector, qrdata);
}

use qr_generator::{QRGenerator, QRSymbolTypes, EncodingModes, CorrectionLevels};
use std::process;

fn main() {
    let options = qr_generator::Options {
        // mode: Some(EncodingModes::Numeric),
        // version: Some(8),
        // correction_level: Some(CorrectionLevels::H),
        ..Default::default()
    };
    let mut generator = QRGenerator { options };
    let ret = generator.make_qr_code("@123456".to_string());
    if let Err(err) = ret {
        println!("save_qr_image failed with {}", err);
        process::exit(1);
    };

    println!("Successfully saved {}", ret.unwrap());
}

use qr_generator::{QRGenerator, QRSymbolTypes, EncodingModes};
use std::process;

fn main() {
    let options = qr_generator::Options {
    //    mode: Some(EncodingModes::Numeric),
        version: Some(2),
        ..Default::default()
    };
    let mut generator = QRGenerator { options };
    let ret = generator.make_qr_code("135792468".to_string());
    if let Err(err) = ret {
        println!("save_qr_image failed with {}", err);
        process::exit(1);
    };

    println!("Successfully saved {}", ret.unwrap());
}

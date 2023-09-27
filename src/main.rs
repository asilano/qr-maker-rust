use qr_generator::{QRGenerator, QRSymbolTypes};
use std::process;

fn main() {
    let options = qr_generator::Options {
        ..Default::default()
    };
    let mut generator = QRGenerator { options };
    let ret = generator.make_qr_code("1234567891".to_string());
    if let Err(err) = ret {
        println!("save_qr_image failed with {}", err);
        process::exit(1);
    };

    println!("Successfully saved {}", ret.unwrap());
}

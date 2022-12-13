use std::process;
use qr_generator::QRGenerator;

fn main() {
    let options = qr_generator::Options { ..Default::default() };
    let generator = QRGenerator { options };
    let ret = generator.make_qr_code("hi".to_string());
    if let Err(err) = ret {
        println!("save_qr_image failed with {}", err); 
        process::exit(1);
    };

    println!("Successfully saved {}", ret.unwrap());
}

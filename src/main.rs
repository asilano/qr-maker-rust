use std::process;

fn main() {
    let filepath = "./qr.png".to_string();
    let ret = qr_generator::save_qr_image(&filepath);
    ret.unwrap_or_else(|err| { 
        println!("save_qr_image failed with {}", err); 
        process::exit(1)
    });

    println!("Successfully saved {}", filepath);
}

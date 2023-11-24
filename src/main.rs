use qr_generator::{QRGenerator, QRSymbolTypes, EncodingModes, CorrectionLevels};
use std::process;

mod cli;
use cli::{Cli, Parser};

fn main() {
    let cli = Cli::parse();

    let options = qr_generator::Options {
        mode: Some(EncodingModes::from(cli.encoding)),
        version: cli.version,
        correction_level: Some(CorrectionLevels::from(cli.correction_level)),
        ..Default::default()
    };
    let mut generator = QRGenerator { options };
    let ret = generator.make_qr_code(cli.data);
    if let Err(err) = ret {
        println!("save_qr_image failed with {}", err);
        process::exit(1);
    };

    println!("Successfully saved {}", ret.unwrap());
}

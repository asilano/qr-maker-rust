pub use clap::{Parser, ValueEnum};
use qr_generator::{EncodingModes, CorrectionLevels};

macro_rules! bidir_from {
  ( $owned:ident, $foreign:ident; $( $variant:ident ),+ ) => {
    impl From<$foreign> for $owned {
      fn from(other: $foreign) -> $owned {
          match other {
            $(
              $foreign::$variant => $owned::$variant
            ),+
           }
      }
    }
    impl From<$owned> for $foreign {
      fn from(other: $owned) -> $foreign {
          match other {
            $(
              $owned::$variant => $foreign::$variant
            ),+
           }
      }
    }
  };
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliEncoding {
  #[value(alias("n"), help("(abbrev: n) Data must consist only of digits 0-9. Most compact"))]
  Numeric,
  #[value(alias("a"), help("(abbrev: a) Data must consist only of 0-9, A-Z (uppercase), $%*+-./:, and <space>"))]
  AlphaNumeric,
  #[value(alias("b"), help("(abbrev: b) Data will be encoded as bytes. Least compact"))]
  Byte,
  #[value(skip)] //, alias("k"), help("(abbrev: k) Data will be encoded as Kanji from Shift-JIS"))]
  Kanji,
  #[value(alias("d"), help("(abbrev: d) Default. Data encoding will change as necessary to obtain the smallest possible code"))]
  Dynamic
}
bidir_from!(CliEncoding, EncodingModes; Numeric, AlphaNumeric, Byte, Dynamic, Kanji);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliCorrectionLevels {
    #[value(alias("d"), help("(abbrev: d) Only valid for M1 Micro QR Codes"))]
    DetectionOnly,
    L,
    M,
    Q,
    H,
}
bidir_from!(CliCorrectionLevels, CorrectionLevels; DetectionOnly, L, M, Q, H);

#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
  // qr_type: SomeTypeHere,
  /// What encoding to use for the data
  #[arg(long, short, value_enum, default_value = "dynamic")]
  pub encoding: CliEncoding,
  #[arg(long, short='l', value_enum, default_value = "q")]
  pub correction_level: CliCorrectionLevels,
  #[arg(long, short, help = "The 'size' of the QR Code. If omitted, the smallest size that fits will be used")]
  pub version: Option<u32>,
  #[arg()]
  pub data: String
}
use crate::{QRGenerator, QRSymbolTypes, QRError, qr_errors::EncodingError, error_correction::CorrectionLevels};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum EncodingModes {
    Numeric,        // 0-9
    AlphaNumeric,   // 0-9, A-Z (ucase), sp, $%*+-./:
    Byte,           // 0x00-0xFF
    Kanji,          // Shift-JIS
    Dynamic
}
type CharacterTypes = EncodingModes;

#[derive(Eq, PartialEq, Debug)]
struct CharTypeRun {
    character_type: CharacterTypes,
    run_length: u32
}

pub struct Encoder<'a> {
    generator: &'a QRGenerator,
    input_data: String,
    pub output_data: Vec<u8>,

    size_estimate: u32,
    data_run_lengths: Vec<CharTypeRun>
}

impl<'a> Encoder<'a> {
    pub fn new(generator: &'a QRGenerator, input_data: String) -> Self {
        Self {
            generator,
            input_data,
            output_data: vec![],
            size_estimate: 0,
            data_run_lengths: vec![]
        }
    }

    // For the moment, we'll ignore Kanji encoding
    pub fn encode_data_into_byte_stream(&mut self) -> Result<Vec<u8>, QRError> {
        self.validate_data_stream_vs_options()?;

        self.estimate_size();
        self.data_run_lengths = self.run_length_encode_data();

        let mut current_encoding = self.generator.options.mode.unwrap_or(EncodingModes::Dynamic);
        let dynamic_mode = current_encoding == EncodingModes::Dynamic;
        if current_encoding == EncodingModes::Dynamic {
            current_encoding = self.select_initial_encoding();
        }

        Ok(vec![1])
    }

    fn validate_data_stream_vs_options(&self) -> Result<(), EncodingError> {
        if self.input_data.is_empty() {
            return Err(EncodingError::new("No data to encode"));
        }

        // MicroQR Codes limit the data types they can handle
        if let Some(version) = self.generator.options.version {
            if self.generator.options.qr_type == Some(QRSymbolTypes::MicroQRCode) {
                if self.input_data.chars().any(|c| !c.is_ascii_digit()) && version == 1 {
                    // We cannot encode the data if it's non-Numeric, and the Options specify an M1 MicroQR
                    return Err(EncodingError::new("Can't encode non-numeric characters in M1 MicroQR"));
                } else if self.input_data.chars().any(|c| !Self::is_qr_alphanumeric(c)) && version <= 2 {
                    // We cannot encode the data if it's non-AlphaNumeric, and the Options specify an M1/2 MicroQR
                    return Err(EncodingError::new("Can't encode non-alphanumeric characters in M1 or M2 MicroQR"));
                }
            }
        }

        let requested_mode = self.generator.options.mode.unwrap_or(EncodingModes::Dynamic);

        // If mode has been specified and the data stream is not consistent with that, error
        if requested_mode != EncodingModes::Byte && self.input_data.chars().any(|c| !Self::is_qr_alphanumeric(c)) {
            return Err(EncodingError::new("Can't encode non-alphanumeric characters when not in Byte mode"));
        }
        if requested_mode == EncodingModes::Numeric && self.input_data.chars().any(|c| !c.is_ascii_digit()) {
            return Err(EncodingError::new("Can't encode non-numeric characters when in Numeric mode"));
        }

        Ok(())
    }

    fn select_initial_encoding(&self) -> EncodingModes {
        if self.generator.options.qr_type == Some(QRSymbolTypes::MicroQRCode) {
            panic!("Unimplemented select_initial_encoding for Micro QR codes");
        }

        let first_char = self.input_data.chars().nth(0).unwrap();
        // J2 a) 1) - If initial data is Byte, start in Byte mode
        if !Self::is_qr_alphanumeric(first_char) {
            return EncodingModes::Byte;
        }

        // J2 a) 2) - IGNORED: Kanji mode first character
        if !first_char.is_ascii_digit() {
            // J2 a) 3) - If data starts Alphanumeric: start in Byte mode if a Byte within [6,7,8] chars; else AN
            let dist_to_byte = if self.size_estimate <= 9 { 6 } else if self.size_estimate <= 26 { 7 } else { 8 };
            if self.input_data.chars().take(dist_to_byte).any(|c| !Self::is_qr_alphanumeric(c)) {
                EncodingModes::Byte
            } else {
                EncodingModes::AlphaNumeric
            }
        } else {
            // J2 a) 4) - If data starts Numeric: start in Byte mode if a Byte within [4, 4, 5] chars; else
            // start in AN if an AN within [7, 8, 9] (and no Bytes first); else start in Numeric.
            let dist_to_byte = if self.size_estimate <= 26 { 4 } else { 5 };
            if self.input_data.chars().take(dist_to_byte).any(|c| !Self::is_qr_alphanumeric(c)) {
                EncodingModes::Byte
            } else {
                let dist_to_an = if self.size_estimate <= 9 { 7 } else if self.size_estimate <= 26 { 8 } else { 9 };
                let first_non_numeric = self.input_data.chars().take(dist_to_an).find(|c| !c.is_ascii_digit());
                match first_non_numeric {
                    Some(c) if Self::is_qr_alphanumeric(c) => EncodingModes::AlphaNumeric,
                    _ => EncodingModes::Numeric
                }
            }
        }
    }

    // Get a rough guess of how large a QR-code this will be. It doesn't need to be exact -
    // we only care about the thresholds <=9, <=26 and over. Assume we're going to use Byte
    // mode.
    fn estimate_size(&mut self) {
        self.size_estimate = self.generator.options.version.unwrap_or_else(|| {
            let (small_st, small_end, med_st, med_end) = match self.generator.options.correction_level {
                Some(CorrectionLevels::L) => { (0, 230, 231, 1367) },
                Some(CorrectionLevels::M) => { (0, 180, 181, 1059) },
                Some(CorrectionLevels::Q) => { (0, 130, 131, 751) },
                Some(CorrectionLevels::H) => { (0, 98, 99, 593) },
                None | Some(CorrectionLevels::DetectionOnly) => unreachable!()
            };

            if (small_st..=small_end).contains(&self.input_data.len()) { 9 }
            else if (med_st..=med_end).contains(&self.input_data.len()) { 26 }
            else { 40 }
        });
    }

    fn run_length_encode_data(&self) -> Vec<CharTypeRun> {
        let mut run_length = 0u32;
        let mut data_run_lengths = vec![];

        let mut chars = self.input_data.chars().peekable();
        let mut next_char_type = chars.peek().and_then(|&c| Some(Self::char_type(c)));
        while chars.next().is_some() {
            run_length += 1;
            let character_type = next_char_type.unwrap();
            next_char_type = chars.peek().and_then(|&c| Some(Self::char_type(c)));

            if Some(character_type) != next_char_type {
                data_run_lengths.push(CharTypeRun { character_type, run_length });
                run_length = 0;
            }
        }
        data_run_lengths
    }

    fn is_qr_alphanumeric(c: char) -> bool {
        ('0'..='9').contains(&c) ||
        ('A'..='Z').contains(&c) ||
        " $%*+-./:".contains(c)
    }

    fn char_type(c: char) -> CharacterTypes {
        if c.is_ascii_digit() {
            CharacterTypes::Numeric
        } else if Self::is_qr_alphanumeric(c) {
            CharacterTypes::AlphaNumeric
        } else {
            CharacterTypes::Byte
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;
    use crate::qr_errors;

    #[test]
    fn validation_fails_if_data_empty() {
        let options = Options { ..Default::default() };
        let generator = QRGenerator { options };
        let encoder = Encoder::new(&generator, "".to_string());

        let result = encoder.validate_data_stream_vs_options();
        assert!(result.is_err());
    }

    #[test]
    // We cannot encode the data if it's non-Numeric, and the Options specify an M1 MicroQR
    fn cannot_encode_non_numeric_in_m1() {
        let generator = QRGenerator {
            options: Options {
                qr_type: Some(QRSymbolTypes::MicroQRCode),
                version: Some(1),
                ..Default::default()
            }
        };
        let encoder = Encoder::new(&generator, "123A456".to_string());
        let result = encoder.validate_data_stream_vs_options();
        assert!(result.is_err());
    }

    #[test]
    // We cannot encode the data if it's non-Alphnum, and the Options specify an M1 or M2 MicroQR
    fn cannot_encode_non_alphanum_in_m1() {
        let generator = QRGenerator {
            options: Options {
                qr_type: Some(QRSymbolTypes::MicroQRCode),
                version: Some(1),
                ..Default::default()
            }
        };
        let encoder = Encoder::new(&generator, "123^456".to_string());
        let result = encoder.validate_data_stream_vs_options();
        assert!(result.is_err());
    }
    #[test]
    fn cannot_encode_non_alphanum_in_m2() {
        let generator = QRGenerator {
            options: Options {
                qr_type: Some(QRSymbolTypes::MicroQRCode),
                version: Some(2),
                ..Default::default()
            }
        };
        let encoder = Encoder::new(&generator, "123^456".to_string());
        let result = encoder.validate_data_stream_vs_options();
        assert!(result.is_err());
    }

    #[test]
    fn cannot_encode_non_numeric_in_numeric_mode() {
        let generator = QRGenerator {
            options: Options {
                mode: Some(EncodingModes::Numeric),
                ..Default::default()
            }
        };
        let encoder = Encoder::new(&generator, "123Z456".to_string());
        let result = encoder.validate_data_stream_vs_options();
        assert!(result.is_err());
    }
    #[test]
    fn cannot_encode_non_alphanum_in_alphanum_mode() {
        let generator = QRGenerator {
            options: Options {
                mode: Some(EncodingModes::AlphaNumeric),
                ..Default::default()
            }
        };
        let encoder = Encoder::new(&generator, "A#BC".to_string());
        let result = encoder.validate_data_stream_vs_options();
        assert!(result.is_err());
    }

    #[test]
    fn when_first_character_not_alphanum_starts_in_byte() {
        let generator = QRGenerator::default();
        let encoder = Encoder::new(&generator, "#ABC123PLO.".to_string());
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Byte);
    }
    #[test]
    fn when_first_character_alphanum_but_non_alphanum_follows_starts_in_byte() {
        let generator = QRGenerator::default();
        let encoder = Encoder::new(&generator, "ABC1#23PLO.".to_string());
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Byte);
    }
    #[test]
    fn when_first_character_alphanum_and_non_alphanum_follows_much_later_starts_in_alphanum() {
        let generator = QRGenerator::default();
        let encoder = Encoder::new(&generator, "ABC123#PLO.".to_string());
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::AlphaNumeric);
    }
    #[test]
    fn when_first_character_numeric_but_non_alphanum_follows_starts_in_byte() {
        let generator = QRGenerator::default();
        let encoder = Encoder::new(&generator, "12#2423PLO.".to_string());
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Byte);
    }
    #[test]
    fn when_first_character_numeric_but_alphanum_follows_starts_in_alphanum() {
        let generator = QRGenerator::default();
        let encoder = Encoder::new(&generator, "12345F456PLO.".to_string());
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::AlphaNumeric);
    }
    #[test]
    fn when_first_character_numeric_and_non_alphanum_follows_much_later_starts_in_numeric() {
        let generator = QRGenerator::default();
        let encoder = Encoder::new(&generator, "1234#PLO.".to_string());
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Numeric);
    }
    #[test]
    fn when_first_character_numeric_and_alphanum_follows_much_later_starts_in_numeric() {
        let generator = QRGenerator::default();
        let encoder = Encoder::new(&generator, "1234567PLO.".to_string());
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Numeric);
    }

    #[test]
    fn run_length_encoder_functions_as_expected() {
        let generator = QRGenerator::default();

        let encoder = Encoder::new(&generator, "1234567".to_string());
        let run_length_encoding = encoder.run_length_encode_data();
        assert_eq!(&run_length_encoding[..], &[CharTypeRun { character_type: CharacterTypes::Numeric, run_length: 7 }]);

        let encoder = Encoder::new(&generator, "1234ABCD567".to_string());
        let run_length_encoding = encoder.run_length_encode_data();
        assert_eq!(&run_length_encoding[..],
            &[
                CharTypeRun { character_type: CharacterTypes::Numeric, run_length: 4 },
                CharTypeRun { character_type: CharacterTypes::AlphaNumeric, run_length: 4 },
                CharTypeRun { character_type: CharacterTypes::Numeric, run_length: 3 },
            ]);

        let encoder = Encoder::new(&generator, "#)1A345.G^@".to_string());
        let run_length_encoding = encoder.run_length_encode_data();
        assert_eq!(&run_length_encoding[..],
            &[
                CharTypeRun { character_type: CharacterTypes::Byte, run_length: 2 },
                CharTypeRun { character_type: CharacterTypes::Numeric, run_length: 1 },
                CharTypeRun { character_type: CharacterTypes::AlphaNumeric, run_length: 1 },
                CharTypeRun { character_type: CharacterTypes::Numeric, run_length: 3 },
                CharTypeRun { character_type: CharacterTypes::AlphaNumeric, run_length: 2 },
                CharTypeRun { character_type: CharacterTypes::Byte, run_length: 2 },
            ]);
    }

    #[test]
    fn alphanumeric_includes_all_legit_characters() {
        let string = "A B$C%0.1/2*X+Y-Z:";
        assert!(string.chars().all(|c| Encoder::is_qr_alphanumeric(c)));
    }
    #[test]
    fn alphanumeric_doesnt_include_lowercase() {
        let string = "abcpqr";
        assert!(string.chars().all(|c| !Encoder::is_qr_alphanumeric(c)));
    }
}
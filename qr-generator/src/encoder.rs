use crate::{
    error_correction::CorrectionLevels, qr_errors::EncodingError, sizer::Sizer, QRError,
    QRGenerator, QRSymbolTypes,
};
use bitvec::{prelude::*, vec::BitVec};
use core::iter::Peekable;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Debug)]
pub enum EncodingModes {
    Numeric,      // 0-9
    AlphaNumeric, // 0-9, A-Z (ucase), sp, $%*+-./:
    Kanji,        // Shift-JIS
    Byte,         // 0x00-0xFF
    Dynamic,
}
type CharacterTypes = EncodingModes;

#[derive(Eq, PartialEq, Default, Clone, Debug)]
struct DistToNextType {
    numeric: Option<usize>,
    alpha_numeric: Option<usize>,
    kanji: Option<usize>,
    byte: Option<usize>,
}

pub struct Encoder<'a> {
    generator: &'a QRGenerator,
    input_data: String,
    pub output_data: BitVec<u8, Msb0>,

    size_estimate: u32,
    change_distances: Vec<DistToNextType>,
}

impl<'a> Encoder<'a> {
    pub fn new(generator: &'a QRGenerator, input_data: String) -> Self {
        Self {
            generator,
            input_data,
            output_data: bitvec![u8, Msb0;],
            size_estimate: 0,
            change_distances: vec![],
        }
    }

    // For the moment, we'll ignore Kanji encoding
    pub fn encode_data_into_byte_stream(&mut self) -> Result<(), QRError> {
        if self.generator.options.qr_type == Some(QRSymbolTypes::MicroQRCode) {
            panic!("Unimplemented select_initial_encoding for Micro QR codes");
        }
        self.validate_data_stream_vs_options()?;

        self.estimate_size();
        self.change_distances = Self::calculate_change_distances(&self.input_data);

        let mut current_encoding = self
            .generator
            .options
            .mode
            .unwrap_or(EncodingModes::Dynamic);
        let dynamic_mode = current_encoding == EncodingModes::Dynamic;
        if current_encoding == EncodingModes::Dynamic {
            current_encoding = self.select_initial_encoding();
        }

        let mut input_iter = self
            .input_data
            .chars()
            .zip(self.change_distances.iter())
            .peekable();
        while input_iter.peek().is_some() {
            let (next_encoding, mut bit_run, char_count) = match current_encoding {
                EncodingModes::Numeric => Self::encode_numeric_run(&mut input_iter),
                // EncodingModes::AlphaNumeric => self.encode_alphanumeric_run(&mut input_iter),
                // EncodingModes::Byte => self.encode_byte_run(&mut input_iter),
                _ => unreachable!(),
            };
            self.output_data
                .append(&mut self.sequence_preamble(current_encoding, char_count));
            self.output_data.append(&mut bit_run);
            current_encoding = next_encoding;
        }

        self.output_data.append(&mut self.terminator());
        self.output_data
            .append(&mut self.padding_to_codeword_boundary());
        self.output_data.append(&mut self.padding_codewords()?);
        Ok(())
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
                    return Err(EncodingError::new(
                        "Can't encode non-numeric characters in M1 MicroQR",
                    ));
                } else if self
                    .input_data
                    .chars()
                    .any(|c| !Self::is_qr_alphanumeric(c))
                    && version <= 2
                {
                    // We cannot encode the data if it's non-AlphaNumeric, and the Options specify an M1/2 MicroQR
                    return Err(EncodingError::new(
                        "Can't encode non-alphanumeric characters in M1 or M2 MicroQR",
                    ));
                }
            }
        }

        let requested_mode = self
            .generator
            .options
            .mode
            .unwrap_or(EncodingModes::Dynamic);

        // If mode has been specified and the data stream is not consistent with that, error
        if requested_mode != EncodingModes::Byte
            && requested_mode != EncodingModes::Dynamic
            && self
                .input_data
                .chars()
                .any(|c| !Self::is_qr_alphanumeric(c))
        {
            return Err(EncodingError::new(
                "Can't encode non-alphanumeric characters when not in Byte mode",
            ));
        }
        if requested_mode == EncodingModes::Numeric
            && self.input_data.chars().any(|c| !c.is_ascii_digit())
        {
            return Err(EncodingError::new(
                "Can't encode non-numeric characters when in Numeric mode",
            ));
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
            let dist_to_byte: usize = if self.size_estimate <= 9 {
                6
            } else if self.size_estimate <= 26 {
                7
            } else {
                8
            };
            match self.change_distances[0].byte {
                Some(dist) if dist < dist_to_byte => EncodingModes::Byte,
                _ => EncodingModes::AlphaNumeric,
            }
        } else {
            // J2 a) 4) - If data starts Numeric: start in Byte mode if a Byte within [4, 4, 5] chars; else
            // start in AN if an AN within [7, 8, 9] (and no Bytes first); else start in Numeric.
            let dist_to_byte: usize = if self.size_estimate <= 26 { 4 } else { 5 };
            match self.change_distances[0].byte {
                Some(dist) if dist < dist_to_byte => EncodingModes::Byte,
                _ => {
                    let dist_to_an: usize = if self.size_estimate <= 9 {
                        7
                    } else if self.size_estimate <= 26 {
                        8
                    } else {
                        9
                    };
                    match (
                        self.change_distances[0].alpha_numeric,
                        self.change_distances[0].byte,
                    ) {
                        (Some(dist_an), Some(dist_byte))
                            if dist_an < dist_to_an && dist_an < dist_byte =>
                        {
                            EncodingModes::AlphaNumeric
                        }
                        (Some(dist_an), None) if dist_an < dist_to_an => {
                            EncodingModes::AlphaNumeric
                        }
                        _ => EncodingModes::Numeric,
                    }
                }
            }
        }
    }

    fn encode_numeric_run<'b, Input>(
        input: &mut Peekable<Input>,
    ) -> (EncodingModes, BitVec<u8, Msb0>, usize)
    where
        Input: Iterator<Item = (char, &'b DistToNextType)>,
    {
        let numbers = input.peeking_take_while(|(c, _)| c.is_ascii_digit());
        let mut char_count = 0usize;
        let mut encoded_numbers = bitvec![u8, Msb0;];

        for chunk in &numbers.chunks(3) {
            let digits_str: String = chunk.map(|(c, _)| c).collect();
            let chunk_size = digits_str.len();
            char_count += chunk_size;

            let bit_count = match chunk_size {
                3 => 10,
                2 => 7,
                1 => 4,
                _ => unreachable!(),
            };
            let value = digits_str
                .parse::<u16>()
                .expect("Non-number in numeric run");
            let mut bits = bitarr![u16, Msb0; 0; 10];
            bits[0..bit_count].store(value);
            encoded_numbers.extend_from_bitslice(&bits[0..bit_count]);
        }

        (EncodingModes::Numeric, encoded_numbers, char_count)
    }

    fn sequence_preamble(&self, encoding: EncodingModes, char_count: usize) -> BitVec<u8, Msb0> {
        let mut sequence_preamble = bitvec![u8, Msb0;];

        match self.generator.options.qr_type {
            Some(QRSymbolTypes::MicroQRCode) => {
                let bits = (self.generator.options.version.unwrap() - 1) as usize;
                let mut mode_indicator = bitvec![u8, Msb0; 0; bits];
                if bits > 0 {
                    mode_indicator[0..bits].store(match encoding {
                        EncodingModes::Numeric => 0,
                        EncodingModes::AlphaNumeric => 1,
                        EncodingModes::Byte => 2,
                        EncodingModes::Kanji => 3,
                        _ => unreachable!(),
                    })
                }
                sequence_preamble.append(&mut mode_indicator);
            }
            Some(QRSymbolTypes::QRCode) => {
                let mut mode_indicator: BitVec<u8, Msb0> = match encoding {
                    EncodingModes::Numeric => bitvec![u8, Msb0; 0, 0, 0, 1],
                    EncodingModes::AlphaNumeric => bitvec![u8, Msb0; 0, 0, 1, 0],
                    EncodingModes::Byte => bitvec![u8, Msb0; 0, 1, 0, 0],
                    EncodingModes::Kanji => bitvec![u8, Msb0; 1, 0, 0, 0],
                    _ => unreachable!(),
                };
                sequence_preamble.append(&mut mode_indicator);
            }
            _ => unreachable!(),
        };

        let len_indicator_len: usize = match self.generator.options.qr_type {
            Some(QRSymbolTypes::MicroQRCode) => {
                (match encoding {
                    EncodingModes::Numeric => 2 + self.generator.options.version.unwrap(),
                    EncodingModes::AlphaNumeric => 1 + self.generator.options.version.unwrap(),
                    EncodingModes::Byte => 1 + self.generator.options.version.unwrap(),
                    EncodingModes::Kanji => self.generator.options.version.unwrap(),
                    _ => unreachable!(),
                }) as usize
            }
            Some(QRSymbolTypes::QRCode) => match encoding {
                EncodingModes::Numeric => HashMap::from([(9, 10), (26, 12), (40, 14)]),
                EncodingModes::AlphaNumeric => HashMap::from([(9, 9), (26, 11), (40, 13)]),
                EncodingModes::Byte => HashMap::from([(9, 8), (26, 16), (40, 16)]),
                EncodingModes::Kanji => HashMap::from([(9, 8), (26, 10), (40, 12)]),
                _ => unreachable!(),
            }
            .get(&self.size_estimate)
            .unwrap()
            .clone() as usize,
            _ => unreachable!(),
        };
        let mut len_indicator = bitvec![u16, Msb0; 0; len_indicator_len];
        len_indicator[0..len_indicator_len].store(char_count);
        sequence_preamble.append(&mut len_indicator);

        sequence_preamble
    }

    fn terminator(&self) -> BitVec<u8, Msb0> {
        let terminator_len = match self.generator.options.qr_type {
            Some(QRSymbolTypes::MicroQRCode) => match self.generator.options.version {
                Some(1) => 3,
                Some(2) => 5,
                Some(3) => 7,
                Some(4) => 9,
                _ => unreachable!(),
            },
            Some(QRSymbolTypes::QRCode) => 4,
            _ => unreachable!(),
        };

        bitvec![u8, Msb0; 0; terminator_len]
    }

    fn padding_to_codeword_boundary(&self) -> BitVec<u8, Msb0> {
        let remainder = self.output_data.len() % 8;
        let pad_len = 8 - remainder;

        bitvec![u8, Msb0; 0; pad_len]
    }

    fn padding_codewords(&self) -> Result<BitVec<u8, Msb0>, EncodingError> {
        let total_codewords = Sizer::data_codeword_capacity(
            self.generator.options.qr_type.as_ref().unwrap(),
            self.generator.options.version.unwrap(),
            self.generator.options.correction_level.as_ref().unwrap(),
        );

        // Allow up to 4 bits more than the data capacity - that would just indicate a complete-full M1 or M3 code
        if self.output_data.len() > (total_codewords * 8) + 4 {
            return Err(EncodingError::new("Too much data for specified code size."));
        }

        let padding_amount = total_codewords - self.output_data.len() / 8;
        let mut padding_codewords = bitvec![u8, Msb0;];
        for pad in 0..padding_amount {
            if pad == padding_amount - 1
                && self.generator.options.qr_type == Some(QRSymbolTypes::MicroQRCode)
                && (self.generator.options.version == Some(1)
                    || self.generator.options.version == Some(3))
            {
                padding_codewords.append(&mut bitvec![u8, Msb0; 0, 0, 0, 0]);
            } else {
                padding_codewords.append(&mut match pad % 2 {
                    0 => bitvec![u8, Msb0; 1, 1, 1, 0, 1, 1, 0, 0],
                    1 => bitvec![u8, Msb0; 0, 0, 0, 1, 0, 0, 0, 1],
                    _ => unreachable!(),
                });
            }
        }

        Ok(padding_codewords)
    }

    // Get a rough guess of how large a QR-code this will be. It doesn't need to be exact -
    // we only care about the thresholds <=9, <=26 and over. Assume we're going to use Byte
    // mode.
    fn estimate_size(&mut self) {
        let size_estimate = self.generator.options.version.unwrap_or_else(|| {
            let (small_st, small_end, med_st, med_end) =
                match self.generator.options.correction_level {
                    Some(CorrectionLevels::L) => (0, 230, 231, 1367),
                    Some(CorrectionLevels::M) => (0, 180, 181, 1059),
                    Some(CorrectionLevels::Q) => (0, 130, 131, 751),
                    Some(CorrectionLevels::H) => (0, 98, 99, 593),
                    None | Some(CorrectionLevels::DetectionOnly) => unreachable!(),
                };

            if (small_st..=small_end).contains(&self.input_data.len()) {
                9
            } else if (med_st..=med_end).contains(&self.input_data.len()) {
                26
            } else {
                40
            }
        });

        self.size_estimate = match size_estimate {
            (0..=9) => 9,
            (10..=26) => 26,
            (27..) => 40,
        };
    }

    fn calculate_change_distances(input_data: &String) -> Vec<DistToNextType> {
        let mut byte_rindex: Option<usize> = None;
        let mut kanji_rindex: Option<usize> = None;
        let mut alphanum_rindex: Option<usize> = None;
        let mut numeric_rindex: Option<usize> = None;
        let input_len = input_data.len();
        let mut distances = vec![
            DistToNextType {
                ..Default::default()
            };
            input_len
        ];

        for (from_end, c) in input_data.chars().rev().enumerate() {
            match Self::char_type(c) {
                CharacterTypes::Byte => byte_rindex = Some(from_end),
                CharacterTypes::AlphaNumeric => alphanum_rindex = Some(from_end),
                CharacterTypes::Numeric => numeric_rindex = Some(from_end),
                _ => unreachable!(),
            };

            let byte = byte_rindex.and_then(|rix| Some(from_end - rix));
            let alpha_numeric = alphanum_rindex.and_then(|rix| Some(from_end - rix));
            let numeric = numeric_rindex.and_then(|rix| Some(from_end - rix));

            distances[input_len - from_end - 1] = DistToNextType {
                numeric,
                alpha_numeric,
                kanji: None,
                byte,
            };
        }

        distances
    }

    fn is_qr_alphanumeric(c: char) -> bool {
        ('0'..='9').contains(&c) || ('A'..='Z').contains(&c) || " $%*+-./:".contains(c)
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
    use crate::qr_errors;
    use crate::Options;

    #[test]
    fn validation_fails_if_data_empty() {
        let options = Options {
            ..Default::default()
        };
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
            },
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
            },
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
            },
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
            },
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
            },
        };
        let encoder = Encoder::new(&generator, "A#BC".to_string());
        let result = encoder.validate_data_stream_vs_options();
        assert!(result.is_err());
    }

    #[test]
    fn when_first_character_not_alphanum_starts_in_byte() {
        let generator = QRGenerator::default();
        let mut encoder = Encoder::new(&generator, "#ABC123PLO.".to_string());
        encoder.change_distances = Encoder::calculate_change_distances(&encoder.input_data);
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Byte);
    }
    #[test]
    fn when_first_character_alphanum_but_non_alphanum_follows_starts_in_byte() {
        let generator = QRGenerator::default();
        let mut encoder = Encoder::new(&generator, "ABC1#23PLO.".to_string());
        encoder.change_distances = Encoder::calculate_change_distances(&encoder.input_data);
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Byte);
    }
    #[test]
    fn when_first_character_alphanum_and_non_alphanum_follows_much_later_starts_in_alphanum() {
        let generator = QRGenerator::default();
        let mut encoder = Encoder::new(&generator, "ABC123#PLO.".to_string());
        encoder.change_distances = Encoder::calculate_change_distances(&encoder.input_data);
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::AlphaNumeric);
    }
    #[test]
    fn when_first_character_numeric_but_non_alphanum_follows_starts_in_byte() {
        let generator = QRGenerator::default();
        let mut encoder = Encoder::new(&generator, "12#2423PLO.".to_string());
        encoder.change_distances = Encoder::calculate_change_distances(&encoder.input_data);
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Byte);
    }
    #[test]
    fn when_first_character_numeric_but_alphanum_follows_starts_in_alphanum() {
        let generator = QRGenerator::default();
        let mut encoder = Encoder::new(&generator, "12345F456PLO.".to_string());
        encoder.change_distances = Encoder::calculate_change_distances(&encoder.input_data);
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::AlphaNumeric);
    }
    #[test]
    fn when_first_character_numeric_and_non_alphanum_follows_much_later_starts_in_numeric() {
        let generator = QRGenerator::default();
        let mut encoder = Encoder::new(&generator, "1234#PLO.".to_string());
        encoder.change_distances = Encoder::calculate_change_distances(&encoder.input_data);
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Numeric);
    }
    #[test]
    fn when_first_character_numeric_and_alphanum_follows_much_later_starts_in_numeric() {
        let generator = QRGenerator::default();
        let mut encoder = Encoder::new(&generator, "1234567PLO.".to_string());
        encoder.change_distances = Encoder::calculate_change_distances(&encoder.input_data);
        let mode = encoder.select_initial_encoding();
        assert_eq!(mode, EncodingModes::Numeric);
    }

    #[test]
    fn change_distances_are_correctly_calculated() {
        let distances = Encoder::calculate_change_distances(&"ABC".to_string());
        assert_eq!(
            &distances[..],
            &[
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(0),
                    numeric: None,
                    kanji: None
                },
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(0),
                    numeric: None,
                    kanji: None
                },
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(0),
                    numeric: None,
                    kanji: None
                },
            ]
        );

        let distances = Encoder::calculate_change_distances(&"A1C".to_string());
        assert_eq!(
            &distances[..],
            &[
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(0),
                    numeric: Some(1),
                    kanji: None
                },
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(1),
                    numeric: Some(0),
                    kanji: None
                },
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(0),
                    numeric: None,
                    kanji: None
                },
            ]
        );

        let distances = Encoder::calculate_change_distances(&"^^^AAAA1111ZZ^^11A".to_string());
        assert_eq!(
            &distances[..],
            &[
                DistToNextType {
                    byte: Some(0),
                    alpha_numeric: Some(3),
                    numeric: Some(7),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(0),
                    alpha_numeric: Some(2),
                    numeric: Some(6),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(0),
                    alpha_numeric: Some(1),
                    numeric: Some(5),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(10),
                    alpha_numeric: Some(0),
                    numeric: Some(4),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(9),
                    alpha_numeric: Some(0),
                    numeric: Some(3),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(8),
                    alpha_numeric: Some(0),
                    numeric: Some(2),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(7),
                    alpha_numeric: Some(0),
                    numeric: Some(1),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(6),
                    alpha_numeric: Some(4),
                    numeric: Some(0),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(5),
                    alpha_numeric: Some(3),
                    numeric: Some(0),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(4),
                    alpha_numeric: Some(2),
                    numeric: Some(0),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(3),
                    alpha_numeric: Some(1),
                    numeric: Some(0),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(2),
                    alpha_numeric: Some(0),
                    numeric: Some(4),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(1),
                    alpha_numeric: Some(0),
                    numeric: Some(3),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(0),
                    alpha_numeric: Some(4),
                    numeric: Some(2),
                    kanji: None
                },
                DistToNextType {
                    byte: Some(0),
                    alpha_numeric: Some(3),
                    numeric: Some(1),
                    kanji: None
                },
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(2),
                    numeric: Some(0),
                    kanji: None
                },
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(1),
                    numeric: Some(0),
                    kanji: None
                },
                DistToNextType {
                    byte: None,
                    alpha_numeric: Some(0),
                    numeric: None,
                    kanji: None
                },
            ]
        );
    }

    #[test]
    fn encodes_numeric_run_correctly() {
        let generator = QRGenerator::default();
        let mut encoder = Encoder::new(&generator, "01234567".to_string());
        encoder.change_distances = Encoder::calculate_change_distances(&encoder.input_data);
        let mut input = encoder
            .input_data
            .chars()
            .zip(encoder.change_distances.iter())
            .peekable();
        let (_, encoded_run, _) = Encoder::encode_numeric_run(&mut input);

        assert_eq!(
            encoded_run.as_bitslice(),
            bits![u8, Msb0; 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1]
        );
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

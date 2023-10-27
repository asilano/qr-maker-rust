use crate::{
    encoder::EncodingModes, error_correction::CorrectionLevels, qr_errors::EncodingError, Options,
    QRError, QRGenerator, QRSymbolTypes,
};
pub struct Sizer;

impl Sizer {
    pub(crate) fn calculate_version(options: &Options, data: &String) -> Result<u32, QRError> {
        let correction = options.correction_level.as_ref().unwrap();
        let mode = options.mode.unwrap_or(EncodingModes::Dynamic);
        match options.qr_type {
            Some(QRSymbolTypes::MicroQRCode) => {
                Ok(Self::calculate_micro_version(correction, mode, data)?)
            }
            Some(QRSymbolTypes::QRCode) => {
                Ok(Self::calculate_standard_version(correction, mode, data)?)
            }
            None => unreachable!(),
        }
    }

    pub(crate) fn data_codeword_capacity(
        qr_type: &QRSymbolTypes,
        version: u32,
        correction: &CorrectionLevels,
    ) -> usize {
        match qr_type {
            QRSymbolTypes::MicroQRCode => match (version, correction) {
                (1, CorrectionLevels::DetectionOnly) => 3,
                (2, CorrectionLevels::L) => 5,
                (2, CorrectionLevels::M) => 4,
                (3, CorrectionLevels::L) => 11,
                (3, CorrectionLevels::M) => 9,
                (4, CorrectionLevels::L) => 16,
                (4, CorrectionLevels::M) => 14,
                (4, CorrectionLevels::Q) => 10,
                _ => unreachable!(),
            },
            QRSymbolTypes::QRCode => match (version, correction) {
                (1, CorrectionLevels::L) => 19,
                (1, CorrectionLevels::M) => 16,
                (1, CorrectionLevels::Q) => 13,
                (1, CorrectionLevels::H) => 9,
                (2, CorrectionLevels::L) => 34,
                (2, CorrectionLevels::M) => 28,
                (2, CorrectionLevels::Q) => 22,
                (2, CorrectionLevels::H) => 16,
                (3, CorrectionLevels::L) => 55,
                (3, CorrectionLevels::M) => 44,
                (3, CorrectionLevels::Q) => 34,
                (3, CorrectionLevels::H) => 26,
                (4, CorrectionLevels::L) => 80,
                (4, CorrectionLevels::M) => 64,
                (4, CorrectionLevels::Q) => 48,
                (4, CorrectionLevels::H) => 36,
                (5, CorrectionLevels::L) => 108,
                (5, CorrectionLevels::M) => 86,
                (5, CorrectionLevels::Q) => 62,
                (5, CorrectionLevels::H) => 46,
                (6, CorrectionLevels::L) => 136,
                (6, CorrectionLevels::M) => 108,
                (6, CorrectionLevels::Q) => 76,
                (6, CorrectionLevels::H) => 60,
                (7, CorrectionLevels::L) => 156,
                (7, CorrectionLevels::M) => 124,
                (7, CorrectionLevels::Q) => 88,
                (7, CorrectionLevels::H) => 66,
                (8, CorrectionLevels::L) => 194,
                (8, CorrectionLevels::M) => 154,
                (8, CorrectionLevels::Q) => 110,
                (8, CorrectionLevels::H) => 86,
                (9, CorrectionLevels::L) => 232,
                (9, CorrectionLevels::M) => 182,
                (9, CorrectionLevels::Q) => 132,
                (9, CorrectionLevels::H) => 100,
                (10, CorrectionLevels::L) => 274,
                (10, CorrectionLevels::M) => 216,
                (10, CorrectionLevels::Q) => 154,
                (10, CorrectionLevels::H) => 122,
                (11, CorrectionLevels::L) => 324,
                (11, CorrectionLevels::M) => 254,
                (11, CorrectionLevels::Q) => 180,
                (11, CorrectionLevels::H) => 140,
                (12, CorrectionLevels::L) => 370,
                (12, CorrectionLevels::M) => 290,
                (12, CorrectionLevels::Q) => 206,
                (12, CorrectionLevels::H) => 158,
                (13, CorrectionLevels::L) => 428,
                (13, CorrectionLevels::M) => 334,
                (13, CorrectionLevels::Q) => 244,
                (13, CorrectionLevels::H) => 180,
                (14, CorrectionLevels::L) => 461,
                (14, CorrectionLevels::M) => 365,
                (14, CorrectionLevels::Q) => 261,
                (14, CorrectionLevels::H) => 197,
                (15, CorrectionLevels::L) => 523,
                (15, CorrectionLevels::M) => 415,
                (15, CorrectionLevels::Q) => 295,
                (15, CorrectionLevels::H) => 223,
                (16, CorrectionLevels::L) => 589,
                (16, CorrectionLevels::M) => 453,
                (16, CorrectionLevels::Q) => 325,
                (16, CorrectionLevels::H) => 253,
                (17, CorrectionLevels::L) => 647,
                (17, CorrectionLevels::M) => 507,
                (17, CorrectionLevels::Q) => 367,
                (17, CorrectionLevels::H) => 283,
                (18, CorrectionLevels::L) => 721,
                (18, CorrectionLevels::M) => 563,
                (18, CorrectionLevels::Q) => 397,
                (18, CorrectionLevels::H) => 313,
                (19, CorrectionLevels::L) => 795,
                (19, CorrectionLevels::M) => 627,
                (19, CorrectionLevels::Q) => 445,
                (19, CorrectionLevels::H) => 341,
                (20, CorrectionLevels::L) => 861,
                (20, CorrectionLevels::M) => 669,
                (20, CorrectionLevels::Q) => 485,
                (20, CorrectionLevels::H) => 385,
                (21, CorrectionLevels::L) => 932,
                (21, CorrectionLevels::M) => 714,
                (21, CorrectionLevels::Q) => 512,
                (21, CorrectionLevels::H) => 406,
                (22, CorrectionLevels::L) => 1006,
                (22, CorrectionLevels::M) => 782,
                (22, CorrectionLevels::Q) => 568,
                (22, CorrectionLevels::H) => 442,
                (23, CorrectionLevels::L) => 1094,
                (23, CorrectionLevels::M) => 860,
                (23, CorrectionLevels::Q) => 614,
                (23, CorrectionLevels::H) => 464,
                (24, CorrectionLevels::L) => 1174,
                (24, CorrectionLevels::M) => 914,
                (24, CorrectionLevels::Q) => 664,
                (24, CorrectionLevels::H) => 514,
                (25, CorrectionLevels::L) => 1276,
                (25, CorrectionLevels::M) => 1000,
                (25, CorrectionLevels::Q) => 718,
                (25, CorrectionLevels::H) => 538,
                (26, CorrectionLevels::L) => 1370,
                (26, CorrectionLevels::M) => 1062,
                (26, CorrectionLevels::Q) => 754,
                (26, CorrectionLevels::H) => 596,
                (27, CorrectionLevels::L) => 1468,
                (27, CorrectionLevels::M) => 1128,
                (27, CorrectionLevels::Q) => 808,
                (27, CorrectionLevels::H) => 628,
                (28, CorrectionLevels::L) => 1531,
                (28, CorrectionLevels::M) => 1193,
                (28, CorrectionLevels::Q) => 871,
                (28, CorrectionLevels::H) => 661,
                (29, CorrectionLevels::L) => 1631,
                (29, CorrectionLevels::M) => 1267,
                (29, CorrectionLevels::Q) => 911,
                (29, CorrectionLevels::H) => 701,
                (30, CorrectionLevels::L) => 1735,
                (30, CorrectionLevels::M) => 1373,
                (30, CorrectionLevels::Q) => 985,
                (30, CorrectionLevels::H) => 745,
                (31, CorrectionLevels::L) => 1843,
                (31, CorrectionLevels::M) => 1455,
                (31, CorrectionLevels::Q) => 1033,
                (31, CorrectionLevels::H) => 793,
                (32, CorrectionLevels::L) => 1955,
                (32, CorrectionLevels::M) => 1541,
                (32, CorrectionLevels::Q) => 1115,
                (32, CorrectionLevels::H) => 845,
                (33, CorrectionLevels::L) => 2071,
                (33, CorrectionLevels::M) => 1631,
                (33, CorrectionLevels::Q) => 1171,
                (33, CorrectionLevels::H) => 901,
                (34, CorrectionLevels::L) => 2191,
                (34, CorrectionLevels::M) => 1725,
                (34, CorrectionLevels::Q) => 1231,
                (34, CorrectionLevels::H) => 961,
                (35, CorrectionLevels::L) => 2306,
                (35, CorrectionLevels::M) => 1812,
                (35, CorrectionLevels::Q) => 1286,
                (35, CorrectionLevels::H) => 986,
                (36, CorrectionLevels::L) => 2434,
                (36, CorrectionLevels::M) => 1914,
                (36, CorrectionLevels::Q) => 1354,
                (36, CorrectionLevels::H) => 1054,
                (37, CorrectionLevels::L) => 2566,
                (37, CorrectionLevels::M) => 1992,
                (37, CorrectionLevels::Q) => 1426,
                (37, CorrectionLevels::H) => 1096,
                (38, CorrectionLevels::L) => 2702,
                (38, CorrectionLevels::M) => 2102,
                (38, CorrectionLevels::Q) => 1502,
                (38, CorrectionLevels::H) => 1142,
                (39, CorrectionLevels::L) => 2812,
                (39, CorrectionLevels::M) => 2216,
                (39, CorrectionLevels::Q) => 1582,
                (39, CorrectionLevels::H) => 1222,
                (40, CorrectionLevels::L) => 2956,
                (40, CorrectionLevels::M) => 2334,
                (40, CorrectionLevels::Q) => 1666,
                (40, CorrectionLevels::H) => 1276,
                _ => unreachable!(),
            },
        }
    }

    // Returns a vector of (total codewords, ec codewords, repeat)
    pub(crate) fn error_correction_shape(
        qr_type: &QRSymbolTypes,
        version: u32,
        correction: &CorrectionLevels,
    ) -> Vec<(usize, usize, usize)> {
        match qr_type {
            QRSymbolTypes::MicroQRCode => match (version, correction) {
                (1, CorrectionLevels::DetectionOnly) => vec![(5, 3, 1)],
                (2, CorrectionLevels::L) => vec![(10, 5, 1)],
                (2, CorrectionLevels::M) => vec![(10, 4, 1)],
                (3, CorrectionLevels::L) => vec![(17, 11, 1)],
                (3, CorrectionLevels::M) => vec![(17, 9, 1)],
                (4, CorrectionLevels::L) => vec![(24, 16, 1)],
                (4, CorrectionLevels::M) => vec![(24, 14, 1)],
                (4, CorrectionLevels::Q) => vec![(24, 10, 1)],
                _ => unreachable!(),
            },
            QRSymbolTypes::QRCode => match (version, correction) {
                (1, CorrectionLevels::L) => vec![(26, 19, 1)],
                (1, CorrectionLevels::M) => vec![(26, 16, 1)],
                (1, CorrectionLevels::Q) => vec![(26, 13, 1)],
                (1, CorrectionLevels::H) => vec![(26, 9, 1)],
                (2, CorrectionLevels::L) => vec![(44, 34, 1)],
                (2, CorrectionLevels::M) => vec![(44, 28, 1)],
                (2, CorrectionLevels::Q) => vec![(44, 22, 1)],
                (2, CorrectionLevels::H) => vec![(44, 16, 1)],
                (3, CorrectionLevels::L) => vec![(70, 55, 1)],
                (3, CorrectionLevels::M) => vec![(70, 44, 1)],
                (3, CorrectionLevels::Q) => vec![(35, 17, 2)],
                (3, CorrectionLevels::H) => vec![(35, 13, 2)],
                // TODO: finish copying these out
                // (4, CorrectionLevels::L) => 80,
                // (4, CorrectionLevels::M) => 64,
                // (4, CorrectionLevels::Q) => 48,
                // (4, CorrectionLevels::H) => 36,
                // (5, CorrectionLevels::L) => 108,
                // (5, CorrectionLevels::M) => 86,
                // (5, CorrectionLevels::Q) => 62,
                // (5, CorrectionLevels::H) => 46,
                // (6, CorrectionLevels::L) => 136,
                // (6, CorrectionLevels::M) => 108,
                // (6, CorrectionLevels::Q) => 76,
                // (6, CorrectionLevels::H) => 60,
                // (7, CorrectionLevels::L) => 156,
                // (7, CorrectionLevels::M) => 124,
                // (7, CorrectionLevels::Q) => 88,
                // (7, CorrectionLevels::H) => 66,
                // (8, CorrectionLevels::L) => 194,
                // (8, CorrectionLevels::M) => 154,
                // (8, CorrectionLevels::Q) => 110,
                // (8, CorrectionLevels::H) => 86,
                // (9, CorrectionLevels::L) => 232,
                // (9, CorrectionLevels::M) => 182,
                // (9, CorrectionLevels::Q) => 132,
                // (9, CorrectionLevels::H) => 100,
                // (10, CorrectionLevels::L) => 274,
                // (10, CorrectionLevels::M) => 216,
                // (10, CorrectionLevels::Q) => 154,
                // (10, CorrectionLevels::H) => 122,
                // (11, CorrectionLevels::L) => 324,
                // (11, CorrectionLevels::M) => 254,
                // (11, CorrectionLevels::Q) => 180,
                // (11, CorrectionLevels::H) => 140,
                // (12, CorrectionLevels::L) => 370,
                // (12, CorrectionLevels::M) => 290,
                // (12, CorrectionLevels::Q) => 206,
                // (12, CorrectionLevels::H) => 158,
                // (13, CorrectionLevels::L) => 428,
                // (13, CorrectionLevels::M) => 334,
                // (13, CorrectionLevels::Q) => 244,
                // (13, CorrectionLevels::H) => 180,
                // (14, CorrectionLevels::L) => 461,
                // (14, CorrectionLevels::M) => 365,
                // (14, CorrectionLevels::Q) => 261,
                // (14, CorrectionLevels::H) => 197,
                // (15, CorrectionLevels::L) => 523,
                // (15, CorrectionLevels::M) => 415,
                // (15, CorrectionLevels::Q) => 295,
                // (15, CorrectionLevels::H) => 223,
                // (16, CorrectionLevels::L) => 589,
                // (16, CorrectionLevels::M) => 453,
                // (16, CorrectionLevels::Q) => 325,
                // (16, CorrectionLevels::H) => 253,
                // (17, CorrectionLevels::L) => 647,
                // (17, CorrectionLevels::M) => 507,
                // (17, CorrectionLevels::Q) => 367,
                // (17, CorrectionLevels::H) => 283,
                // (18, CorrectionLevels::L) => 721,
                // (18, CorrectionLevels::M) => 563,
                // (18, CorrectionLevels::Q) => 397,
                // (18, CorrectionLevels::H) => 313,
                // (19, CorrectionLevels::L) => 795,
                // (19, CorrectionLevels::M) => 627,
                // (19, CorrectionLevels::Q) => 445,
                // (19, CorrectionLevels::H) => 341,
                // (20, CorrectionLevels::L) => 861,
                // (20, CorrectionLevels::M) => 669,
                // (20, CorrectionLevels::Q) => 485,
                // (20, CorrectionLevels::H) => 385,
                // (21, CorrectionLevels::L) => 932,
                // (21, CorrectionLevels::M) => 714,
                // (21, CorrectionLevels::Q) => 512,
                // (21, CorrectionLevels::H) => 406,
                // (22, CorrectionLevels::L) => 1006,
                // (22, CorrectionLevels::M) => 782,
                // (22, CorrectionLevels::Q) => 568,
                // (22, CorrectionLevels::H) => 442,
                // (23, CorrectionLevels::L) => 1094,
                // (23, CorrectionLevels::M) => 860,
                // (23, CorrectionLevels::Q) => 614,
                // (23, CorrectionLevels::H) => 464,
                // (24, CorrectionLevels::L) => 1174,
                // (24, CorrectionLevels::M) => 914,
                // (24, CorrectionLevels::Q) => 664,
                // (24, CorrectionLevels::H) => 514,
                // (25, CorrectionLevels::L) => 1276,
                // (25, CorrectionLevels::M) => 1000,
                // (25, CorrectionLevels::Q) => 718,
                // (25, CorrectionLevels::H) => 538,
                // (26, CorrectionLevels::L) => 1370,
                // (26, CorrectionLevels::M) => 1062,
                // (26, CorrectionLevels::Q) => 754,
                // (26, CorrectionLevels::H) => 596,
                // (27, CorrectionLevels::L) => 1468,
                // (27, CorrectionLevels::M) => 1128,
                // (27, CorrectionLevels::Q) => 808,
                // (27, CorrectionLevels::H) => 628,
                // (28, CorrectionLevels::L) => 1531,
                // (28, CorrectionLevels::M) => 1193,
                // (28, CorrectionLevels::Q) => 871,
                // (28, CorrectionLevels::H) => 661,
                // (29, CorrectionLevels::L) => 1631,
                // (29, CorrectionLevels::M) => 1267,
                // (29, CorrectionLevels::Q) => 911,
                // (29, CorrectionLevels::H) => 701,
                // (30, CorrectionLevels::L) => 1735,
                // (30, CorrectionLevels::M) => 1373,
                // (30, CorrectionLevels::Q) => 985,
                // (30, CorrectionLevels::H) => 745,
                // (31, CorrectionLevels::L) => 1843,
                // (31, CorrectionLevels::M) => 1455,
                // (31, CorrectionLevels::Q) => 1033,
                // (31, CorrectionLevels::H) => 793,
                // (32, CorrectionLevels::L) => 1955,
                // (32, CorrectionLevels::M) => 1541,
                // (32, CorrectionLevels::Q) => 1115,
                // (32, CorrectionLevels::H) => 845,
                // (33, CorrectionLevels::L) => 2071,
                // (33, CorrectionLevels::M) => 1631,
                // (33, CorrectionLevels::Q) => 1171,
                // (33, CorrectionLevels::H) => 901,
                // (34, CorrectionLevels::L) => 2191,
                // (34, CorrectionLevels::M) => 1725,
                // (34, CorrectionLevels::Q) => 1231,
                // (34, CorrectionLevels::H) => 961,
                // (35, CorrectionLevels::L) => 2306,
                // (35, CorrectionLevels::M) => 1812,
                // (35, CorrectionLevels::Q) => 1286,
                // (35, CorrectionLevels::H) => 986,
                // (36, CorrectionLevels::L) => 2434,
                // (36, CorrectionLevels::M) => 1914,
                // (36, CorrectionLevels::Q) => 1354,
                // (36, CorrectionLevels::H) => 1054,
                // (37, CorrectionLevels::L) => 2566,
                // (37, CorrectionLevels::M) => 1992,
                // (37, CorrectionLevels::Q) => 1426,
                // (37, CorrectionLevels::H) => 1096,
                // (38, CorrectionLevels::L) => 2702,
                // (38, CorrectionLevels::M) => 2102,
                // (38, CorrectionLevels::Q) => 1502,
                // (38, CorrectionLevels::H) => 1142,
                // (39, CorrectionLevels::L) => 2812,
                // (39, CorrectionLevels::M) => 2216,
                // (39, CorrectionLevels::Q) => 1582,
                // (39, CorrectionLevels::H) => 1222,
                // (40, CorrectionLevels::L) => 2956,
                // (40, CorrectionLevels::M) => 2334,
                // (40, CorrectionLevels::Q) => 1666,
                // (40, CorrectionLevels::H) => 1276,
                _ => unreachable!(),
            },
        }
    }

    fn calculate_micro_version(
        correction: &CorrectionLevels,
        mode: EncodingModes,
        data: &String,
    ) -> Result<u32, EncodingError> {
        match correction {
      CorrectionLevels::DetectionOnly => {
        if data.len() > 5 {
          return Err(EncodingError::new("Error Detection Only implies M1 MicroQR; can't encode more than 5 characters in M1 MicroQR"));
        } else if data.chars().any(|c| !c.is_ascii_digit()) {
          return Err(EncodingError::new("Error Detection Only implies M1 MicroQR; can't encode non-numeric characters in M1 MicroQR"));
        }
        Ok(1)
      },
      CorrectionLevels::L => {
        match mode {
          EncodingModes::Numeric => match data.len() {
            0..=10 => Ok(2),
            11..=23 => Ok(3),
            24..=35 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 35 numeric characters in an L-correction MicroQR"))
          },
          EncodingModes::AlphaNumeric => match data.len() {
            0..=6 => Ok(2),
            7..=14 => Ok(3),
            15..=21 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 21 alphanumeric characters in an L-correction MicroQR"))
          },
          EncodingModes::Byte | EncodingModes::Dynamic => match data.len() {
            0..=9 => Ok(3),
            10..=15 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 15 bytes in an L-correction MicroQR"))
          },
          EncodingModes::Kanji => match data.len() {
            0..=6 => Ok(3),
            7..=9 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 9 kanji in an L-correction MicroQR"))
          },
        }
      },
      CorrectionLevels::M => {
        match mode {
          EncodingModes::Numeric => match data.len() {
            0..=8 => Ok(2),
            9..=18 => Ok(3),
            19..=30 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 30 numeric characters in an M-correction MicroQR"))
          },
          EncodingModes::AlphaNumeric => match data.len() {
            0..=5 => Ok(2),
            6..=11 => Ok(3),
            12..=18 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 18 alphanumeric characters in an M-correction MicroQR"))
          },
          EncodingModes::Byte | EncodingModes::Dynamic => match data.len() {
            0..=7 => Ok(3),
            8..=13 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 13 bytes in an M-correction MicroQR"))
          },
          EncodingModes::Kanji => match data.len() {
            0..=4 => Ok(3),
            5..=8 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 8 kanji in an M-correction MicroQR"))
          },
        }
      },
      CorrectionLevels::Q => {
        match mode {
          EncodingModes::Numeric => match data.len() {
            0..=21 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 21 numeric characters in an Q-correction MicroQR"))
          },
          EncodingModes::AlphaNumeric => match data.len() {
            0..=13 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 13 alphanumeric characters in an Q-correction MicroQR"))
          },
          EncodingModes::Byte | EncodingModes::Dynamic => match data.len() {
            0..=9 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 9 bytes in an Q-correction MicroQR"))
          },
          EncodingModes::Kanji => match data.len() {
            0..=5 => Ok(4),
            _ => Err(EncodingError::new("Can't encode more than 5 kanji in an Q-correction MicroQR"))
          },
        }
      }
      CorrectionLevels::H => Err(EncodingError::new("Can't use H-level error correction in a MicroQR"))
    }
    }

    fn calculate_standard_version(
        correction: &CorrectionLevels,
        mode: EncodingModes,
        data: &String,
    ) -> Result<u32, EncodingError> {
        match mode {
            EncodingModes::Numeric => Self::calculate_standard_numeric_version(correction, data),
            EncodingModes::AlphaNumeric => {
                Self::calculate_standard_alphanumeric_version(correction, data)
            }
            EncodingModes::Byte | EncodingModes::Dynamic => {
                Self::calculate_standard_byte_version(correction, data)
            }
            EncodingModes::Kanji => Self::calculate_standard_kanji_version(correction, data),
        }
    }

    fn calculate_standard_numeric_version(
        correction: &CorrectionLevels,
        data: &String,
    ) -> Result<u32, EncodingError> {
        match (correction, data.len()) {
            (CorrectionLevels::L, 1..=41)
            | (CorrectionLevels::M, 1..=34)
            | (CorrectionLevels::Q, 1..=27)
            | (CorrectionLevels::H, 1..=17) => Ok(1),
            (CorrectionLevels::L, 42..=77)
            | (CorrectionLevels::M, 35..=63)
            | (CorrectionLevels::Q, 28..=48)
            | (CorrectionLevels::H, 18..=34) => Ok(2),
            (CorrectionLevels::L, 78..=127)
            | (CorrectionLevels::M, 64..=101)
            | (CorrectionLevels::Q, 49..=77)
            | (CorrectionLevels::H, 35..=58) => Ok(3),
            (CorrectionLevels::L, 128..=187)
            | (CorrectionLevels::M, 102..=149)
            | (CorrectionLevels::Q, 78..=111)
            | (CorrectionLevels::H, 59..=82) => Ok(4),
            (CorrectionLevels::L, 188..=255)
            | (CorrectionLevels::M, 150..=202)
            | (CorrectionLevels::Q, 112..=144)
            | (CorrectionLevels::H, 83..=106) => Ok(5),
            (CorrectionLevels::L, 256..=322)
            | (CorrectionLevels::M, 203..=255)
            | (CorrectionLevels::Q, 145..=178)
            | (CorrectionLevels::H, 107..=139) => Ok(6),
            (CorrectionLevels::L, 323..=370)
            | (CorrectionLevels::M, 256..=293)
            | (CorrectionLevels::Q, 179..=207)
            | (CorrectionLevels::H, 140..=154) => Ok(7),
            (CorrectionLevels::L, 371..=461)
            | (CorrectionLevels::M, 294..=365)
            | (CorrectionLevels::Q, 208..=259)
            | (CorrectionLevels::H, 155..=202) => Ok(8),
            (CorrectionLevels::L, 462..=552)
            | (CorrectionLevels::M, 366..=432)
            | (CorrectionLevels::Q, 260..=312)
            | (CorrectionLevels::H, 203..=235) => Ok(9),
            (CorrectionLevels::L, 553..=652)
            | (CorrectionLevels::M, 433..=513)
            | (CorrectionLevels::Q, 313..=364)
            | (CorrectionLevels::H, 236..=288) => Ok(10),
            (CorrectionLevels::L, 653..=772)
            | (CorrectionLevels::M, 514..=604)
            | (CorrectionLevels::Q, 365..=427)
            | (CorrectionLevels::H, 289..=331) => Ok(11),
            (CorrectionLevels::L, 773..=883)
            | (CorrectionLevels::M, 605..=691)
            | (CorrectionLevels::Q, 428..=489)
            | (CorrectionLevels::H, 332..=374) => Ok(12),
            (CorrectionLevels::L, 884..=1022)
            | (CorrectionLevels::M, 692..=796)
            | (CorrectionLevels::Q, 490..=580)
            | (CorrectionLevels::H, 375..=427) => Ok(13),
            (CorrectionLevels::L, 1023..=1101)
            | (CorrectionLevels::M, 797..=871)
            | (CorrectionLevels::Q, 581..=621)
            | (CorrectionLevels::H, 428..=468) => Ok(14),
            (CorrectionLevels::L, 1102..=1250)
            | (CorrectionLevels::M, 872..=991)
            | (CorrectionLevels::Q, 622..=703)
            | (CorrectionLevels::H, 469..=530) => Ok(15),
            (CorrectionLevels::L, 1251..=1408)
            | (CorrectionLevels::M, 992..=1082)
            | (CorrectionLevels::Q, 704..=775)
            | (CorrectionLevels::H, 531..=602) => Ok(16),
            (CorrectionLevels::L, 1409..=1548)
            | (CorrectionLevels::M, 1083..=1212)
            | (CorrectionLevels::Q, 776..=876)
            | (CorrectionLevels::H, 603..=674) => Ok(17),
            (CorrectionLevels::L, 1549..=1725)
            | (CorrectionLevels::M, 1213..=1346)
            | (CorrectionLevels::Q, 877..=948)
            | (CorrectionLevels::H, 675..=746) => Ok(18),
            (CorrectionLevels::L, 1726..=1903)
            | (CorrectionLevels::M, 1347..=1500)
            | (CorrectionLevels::Q, 949..=1063)
            | (CorrectionLevels::H, 747..=813) => Ok(19),
            (CorrectionLevels::L, 1904..=2061)
            | (CorrectionLevels::M, 1501..=1600)
            | (CorrectionLevels::Q, 1064..=1159)
            | (CorrectionLevels::H, 814..=919) => Ok(20),
            (CorrectionLevels::L, 2062..=2232)
            | (CorrectionLevels::M, 1601..=1708)
            | (CorrectionLevels::Q, 1160..=1224)
            | (CorrectionLevels::H, 920..=969) => Ok(21),
            (CorrectionLevels::L, 2233..=2409)
            | (CorrectionLevels::M, 1709..=1872)
            | (CorrectionLevels::Q, 1225..=1358)
            | (CorrectionLevels::H, 970..=1056) => Ok(22),
            (CorrectionLevels::L, 2410..=2620)
            | (CorrectionLevels::M, 1873..=2059)
            | (CorrectionLevels::Q, 1359..=1468)
            | (CorrectionLevels::H, 1057..=1108) => Ok(23),
            (CorrectionLevels::L, 2621..=2812)
            | (CorrectionLevels::M, 2060..=2188)
            | (CorrectionLevels::Q, 1469..=1588)
            | (CorrectionLevels::H, 1109..=1228) => Ok(24),
            (CorrectionLevels::L, 2813..=3057)
            | (CorrectionLevels::M, 2189..=2395)
            | (CorrectionLevels::Q, 1589..=1718)
            | (CorrectionLevels::H, 1229..=1286) => Ok(25),
            (CorrectionLevels::L, 3058..=3283)
            | (CorrectionLevels::M, 2396..=2544)
            | (CorrectionLevels::Q, 1719..=1804)
            | (CorrectionLevels::H, 1287..=1425) => Ok(26),
            (CorrectionLevels::L, 3284..=3517)
            | (CorrectionLevels::M, 2545..=2701)
            | (CorrectionLevels::Q, 1805..=1933)
            | (CorrectionLevels::H, 1426..=1501) => Ok(27),
            (CorrectionLevels::L, 3518..=3669)
            | (CorrectionLevels::M, 2702..=2857)
            | (CorrectionLevels::Q, 1934..=2085)
            | (CorrectionLevels::H, 1502..=1581) => Ok(28),
            (CorrectionLevels::L, 3670..=3909)
            | (CorrectionLevels::M, 2858..=3035)
            | (CorrectionLevels::Q, 2086..=2181)
            | (CorrectionLevels::H, 1582..=1667) => Ok(29),
            (CorrectionLevels::L, 3910..=4158)
            | (CorrectionLevels::M, 3036..=3289)
            | (CorrectionLevels::Q, 2182..=2358)
            | (CorrectionLevels::H, 1668..=1782) => Ok(30),
            (CorrectionLevels::L, 4159..=4417)
            | (CorrectionLevels::M, 3290..=3486)
            | (CorrectionLevels::Q, 2359..=2473)
            | (CorrectionLevels::H, 1783..=1897) => Ok(31),
            (CorrectionLevels::L, 4418..=4686)
            | (CorrectionLevels::M, 3487..=3693)
            | (CorrectionLevels::Q, 2474..=2670)
            | (CorrectionLevels::H, 1898..=2022) => Ok(32),
            (CorrectionLevels::L, 4687..=4965)
            | (CorrectionLevels::M, 3694..=3909)
            | (CorrectionLevels::Q, 2671..=2805)
            | (CorrectionLevels::H, 2023..=2157) => Ok(33),
            (CorrectionLevels::L, 4966..=5253)
            | (CorrectionLevels::M, 3910..=4134)
            | (CorrectionLevels::Q, 2806..=2949)
            | (CorrectionLevels::H, 2158..=2301) => Ok(34),
            (CorrectionLevels::L, 5254..=5529)
            | (CorrectionLevels::M, 4135..=4343)
            | (CorrectionLevels::Q, 2950..=3081)
            | (CorrectionLevels::H, 2302..=2361) => Ok(35),
            (CorrectionLevels::L, 5530..=5836)
            | (CorrectionLevels::M, 4344..=4588)
            | (CorrectionLevels::Q, 3082..=3244)
            | (CorrectionLevels::H, 2362..=2524) => Ok(36),
            (CorrectionLevels::L, 5837..=6153)
            | (CorrectionLevels::M, 4589..=4775)
            | (CorrectionLevels::Q, 3245..=3417)
            | (CorrectionLevels::H, 2525..=2625) => Ok(37),
            (CorrectionLevels::L, 6154..=6479)
            | (CorrectionLevels::M, 4776..=5039)
            | (CorrectionLevels::Q, 3418..=3599)
            | (CorrectionLevels::H, 2626..=2735) => Ok(38),
            (CorrectionLevels::L, 6480..=6743)
            | (CorrectionLevels::M, 5040..=5313)
            | (CorrectionLevels::Q, 3600..=3791)
            | (CorrectionLevels::H, 2736..=2927) => Ok(39),
            (CorrectionLevels::L, 6744..=7089)
            | (CorrectionLevels::M, 5314..=5596)
            | (CorrectionLevels::Q, 3792..=3993)
            | (CorrectionLevels::H, 2928..=3057) => Ok(40),
            (CorrectionLevels::DetectionOnly, _) => Err(EncodingError::new(
                "Can't simply detect errors in a Standard QR code",
            )),
            _ => Err(EncodingError::new(
                "Too much data for Numeric mode and error correction level",
            )),
        }
    }

    fn calculate_standard_alphanumeric_version(
        correction: &CorrectionLevels,
        data: &String,
    ) -> Result<u32, EncodingError> {
        match (correction, data.len()) {
            (CorrectionLevels::L, 1..=25)
            | (CorrectionLevels::M, 1..=20)
            | (CorrectionLevels::Q, 1..=16)
            | (CorrectionLevels::H, 1..=10) => Ok(1),
            (CorrectionLevels::L, 26..=47)
            | (CorrectionLevels::M, 21..=38)
            | (CorrectionLevels::Q, 17..=29)
            | (CorrectionLevels::H, 11..=20) => Ok(2),
            (CorrectionLevels::L, 48..=77)
            | (CorrectionLevels::M, 39..=61)
            | (CorrectionLevels::Q, 30..=47)
            | (CorrectionLevels::H, 21..=35) => Ok(3),
            (CorrectionLevels::L, 78..=114)
            | (CorrectionLevels::M, 62..=90)
            | (CorrectionLevels::Q, 48..=67)
            | (CorrectionLevels::H, 36..=50) => Ok(4),
            (CorrectionLevels::L, 115..=154)
            | (CorrectionLevels::M, 91..=122)
            | (CorrectionLevels::Q, 68..=87)
            | (CorrectionLevels::H, 51..=64) => Ok(5),
            (CorrectionLevels::L, 155..=195)
            | (CorrectionLevels::M, 123..=154)
            | (CorrectionLevels::Q, 88..=108)
            | (CorrectionLevels::H, 65..=84) => Ok(6),
            (CorrectionLevels::L, 196..=224)
            | (CorrectionLevels::M, 155..=178)
            | (CorrectionLevels::Q, 109..=125)
            | (CorrectionLevels::H, 85..=93) => Ok(7),
            (CorrectionLevels::L, 225..=279)
            | (CorrectionLevels::M, 179..=221)
            | (CorrectionLevels::Q, 126..=157)
            | (CorrectionLevels::H, 94..=122) => Ok(8),
            (CorrectionLevels::L, 280..=335)
            | (CorrectionLevels::M, 222..=262)
            | (CorrectionLevels::Q, 158..=189)
            | (CorrectionLevels::H, 123..=143) => Ok(9),
            (CorrectionLevels::L, 336..=395)
            | (CorrectionLevels::M, 263..=311)
            | (CorrectionLevels::Q, 190..=221)
            | (CorrectionLevels::H, 144..=174) => Ok(10),
            (CorrectionLevels::L, 396..=468)
            | (CorrectionLevels::M, 312..=366)
            | (CorrectionLevels::Q, 222..=259)
            | (CorrectionLevels::H, 175..=200) => Ok(11),
            (CorrectionLevels::L, 469..=535)
            | (CorrectionLevels::M, 367..=419)
            | (CorrectionLevels::Q, 260..=296)
            | (CorrectionLevels::H, 201..=227) => Ok(12),
            (CorrectionLevels::L, 536..=619)
            | (CorrectionLevels::M, 420..=483)
            | (CorrectionLevels::Q, 297..=352)
            | (CorrectionLevels::H, 228..=259) => Ok(13),
            (CorrectionLevels::L, 620..=667)
            | (CorrectionLevels::M, 484..=528)
            | (CorrectionLevels::Q, 353..=376)
            | (CorrectionLevels::H, 260..=283) => Ok(14),
            (CorrectionLevels::L, 668..=758)
            | (CorrectionLevels::M, 529..=600)
            | (CorrectionLevels::Q, 377..=426)
            | (CorrectionLevels::H, 284..=321) => Ok(15),
            (CorrectionLevels::L, 759..=854)
            | (CorrectionLevels::M, 601..=656)
            | (CorrectionLevels::Q, 427..=470)
            | (CorrectionLevels::H, 322..=365) => Ok(16),
            (CorrectionLevels::L, 855..=938)
            | (CorrectionLevels::M, 657..=734)
            | (CorrectionLevels::Q, 471..=531)
            | (CorrectionLevels::H, 366..=408) => Ok(17),
            (CorrectionLevels::L, 939..=1046)
            | (CorrectionLevels::M, 735..=816)
            | (CorrectionLevels::Q, 532..=574)
            | (CorrectionLevels::H, 409..=452) => Ok(18),
            (CorrectionLevels::L, 1045..=1153)
            | (CorrectionLevels::M, 817..=909)
            | (CorrectionLevels::Q, 575..=644)
            | (CorrectionLevels::H, 453..=493) => Ok(19),
            (CorrectionLevels::L, 1154..=1249)
            | (CorrectionLevels::M, 910..=970)
            | (CorrectionLevels::Q, 645..=702)
            | (CorrectionLevels::H, 494..=557) => Ok(20),
            (CorrectionLevels::L, 1250..=1352)
            | (CorrectionLevels::M, 971..=1035)
            | (CorrectionLevels::Q, 703..=742)
            | (CorrectionLevels::H, 558..=587) => Ok(21),
            (CorrectionLevels::L, 1353..=1460)
            | (CorrectionLevels::M, 1036..=1134)
            | (CorrectionLevels::Q, 743..=823)
            | (CorrectionLevels::H, 588..=640) => Ok(22),
            (CorrectionLevels::L, 1461..=1588)
            | (CorrectionLevels::M, 1135..=1248)
            | (CorrectionLevels::Q, 824..=890)
            | (CorrectionLevels::H, 641..=672) => Ok(23),
            (CorrectionLevels::L, 1589..=1704)
            | (CorrectionLevels::M, 1259..=1326)
            | (CorrectionLevels::Q, 891..=963)
            | (CorrectionLevels::H, 673..=744) => Ok(24),
            (CorrectionLevels::L, 1705..=1853)
            | (CorrectionLevels::M, 1327..=1451)
            | (CorrectionLevels::Q, 964..=1041)
            | (CorrectionLevels::H, 745..=779) => Ok(25),
            (CorrectionLevels::L, 1854..=1990)
            | (CorrectionLevels::M, 1452..=1542)
            | (CorrectionLevels::Q, 1042..=1094)
            | (CorrectionLevels::H, 780..=864) => Ok(26),
            (CorrectionLevels::L, 1991..=2132)
            | (CorrectionLevels::M, 1543..=1637)
            | (CorrectionLevels::Q, 1095..=1172)
            | (CorrectionLevels::H, 865..=910) => Ok(27),
            (CorrectionLevels::L, 2133..=2223)
            | (CorrectionLevels::M, 1638..=1732)
            | (CorrectionLevels::Q, 1173..=1263)
            | (CorrectionLevels::H, 911..=958) => Ok(28),
            (CorrectionLevels::L, 2224..=2369)
            | (CorrectionLevels::M, 1733..=1839)
            | (CorrectionLevels::Q, 1264..=1322)
            | (CorrectionLevels::H, 959..=1016) => Ok(29),
            (CorrectionLevels::L, 2370..=2520)
            | (CorrectionLevels::M, 1840..=1994)
            | (CorrectionLevels::Q, 1323..=1429)
            | (CorrectionLevels::H, 1017..=1080) => Ok(30),
            (CorrectionLevels::L, 2521..=2677)
            | (CorrectionLevels::M, 1995..=2113)
            | (CorrectionLevels::Q, 1430..=1499)
            | (CorrectionLevels::H, 1081..=1150) => Ok(31),
            (CorrectionLevels::L, 2678..=2840)
            | (CorrectionLevels::M, 2114..=2238)
            | (CorrectionLevels::Q, 1500..=1618)
            | (CorrectionLevels::H, 1151..=1226) => Ok(32),
            (CorrectionLevels::L, 2841..=3009)
            | (CorrectionLevels::M, 2239..=2369)
            | (CorrectionLevels::Q, 1619..=1700)
            | (CorrectionLevels::H, 1227..=1307) => Ok(33),
            (CorrectionLevels::L, 3010..=3183)
            | (CorrectionLevels::M, 2370..=2506)
            | (CorrectionLevels::Q, 1701..=1787)
            | (CorrectionLevels::H, 1308..=1394) => Ok(34),
            (CorrectionLevels::L, 3184..=3351)
            | (CorrectionLevels::M, 2507..=2632)
            | (CorrectionLevels::Q, 1788..=1867)
            | (CorrectionLevels::H, 1395..=1431) => Ok(35),
            (CorrectionLevels::L, 3352..=3537)
            | (CorrectionLevels::M, 2633..=2780)
            | (CorrectionLevels::Q, 1868..=1966)
            | (CorrectionLevels::H, 1432..=1530) => Ok(36),
            (CorrectionLevels::L, 3538..=3729)
            | (CorrectionLevels::M, 2781..=2894)
            | (CorrectionLevels::Q, 1967..=2071)
            | (CorrectionLevels::H, 1531..=1591) => Ok(37),
            (CorrectionLevels::L, 3730..=3927)
            | (CorrectionLevels::M, 2895..=3054)
            | (CorrectionLevels::Q, 2072..=2181)
            | (CorrectionLevels::H, 1592..=1658) => Ok(38),
            (CorrectionLevels::L, 3928..=4087)
            | (CorrectionLevels::M, 3055..=3220)
            | (CorrectionLevels::Q, 2182..=2298)
            | (CorrectionLevels::H, 1659..=1774) => Ok(39),
            (CorrectionLevels::L, 4088..=4296)
            | (CorrectionLevels::M, 3221..=3391)
            | (CorrectionLevels::Q, 2299..=2420)
            | (CorrectionLevels::H, 1775..=1852) => Ok(40),
            (CorrectionLevels::DetectionOnly, _) => Err(EncodingError::new(
                "Can't simply detect errors in a Standard QR code",
            )),
            _ => Err(EncodingError::new(
                "Too much data for Numeric mode and error correction level",
            )),
        }
    }

    fn calculate_standard_byte_version(
        correction: &CorrectionLevels,
        data: &String,
    ) -> Result<u32, EncodingError> {
        match (correction, data.len()) {
            (CorrectionLevels::L, 1..=17)
            | (CorrectionLevels::M, 1..=14)
            | (CorrectionLevels::Q, 1..=11)
            | (CorrectionLevels::H, 1..=7) => Ok(1),
            (CorrectionLevels::L, 18..=32)
            | (CorrectionLevels::M, 15..=26)
            | (CorrectionLevels::Q, 12..=20)
            | (CorrectionLevels::H, 8..=14) => Ok(2),
            (CorrectionLevels::L, 33..=53)
            | (CorrectionLevels::M, 27..=42)
            | (CorrectionLevels::Q, 21..=32)
            | (CorrectionLevels::H, 15..=24) => Ok(3),
            (CorrectionLevels::L, 54..=78)
            | (CorrectionLevels::M, 43..=62)
            | (CorrectionLevels::Q, 33..=46)
            | (CorrectionLevels::H, 25..=34) => Ok(4),
            (CorrectionLevels::L, 79..=106)
            | (CorrectionLevels::M, 63..=84)
            | (CorrectionLevels::Q, 47..=60)
            | (CorrectionLevels::H, 35..=44) => Ok(5),
            (CorrectionLevels::L, 107..=134)
            | (CorrectionLevels::M, 85..=106)
            | (CorrectionLevels::Q, 61..=74)
            | (CorrectionLevels::H, 45..=58) => Ok(6),
            (CorrectionLevels::L, 135..=154)
            | (CorrectionLevels::M, 107..=122)
            | (CorrectionLevels::Q, 75..=86)
            | (CorrectionLevels::H, 59..=64) => Ok(7),
            (CorrectionLevels::L, 155..=192)
            | (CorrectionLevels::M, 123..=152)
            | (CorrectionLevels::Q, 87..=108)
            | (CorrectionLevels::H, 65..=84) => Ok(8),
            (CorrectionLevels::L, 193..=230)
            | (CorrectionLevels::M, 153..=180)
            | (CorrectionLevels::Q, 109..=130)
            | (CorrectionLevels::H, 85..=98) => Ok(9),
            (CorrectionLevels::L, 231..=271)
            | (CorrectionLevels::M, 181..=213)
            | (CorrectionLevels::Q, 131..=151)
            | (CorrectionLevels::H, 99..=119) => Ok(10),
            (CorrectionLevels::L, 272..=321)
            | (CorrectionLevels::M, 214..=251)
            | (CorrectionLevels::Q, 152..=177)
            | (CorrectionLevels::H, 120..=137) => Ok(11),
            (CorrectionLevels::L, 322..=367)
            | (CorrectionLevels::M, 252..=287)
            | (CorrectionLevels::Q, 178..=203)
            | (CorrectionLevels::H, 138..=155) => Ok(12),
            (CorrectionLevels::L, 368..=425)
            | (CorrectionLevels::M, 288..=331)
            | (CorrectionLevels::Q, 204..=241)
            | (CorrectionLevels::H, 156..=177) => Ok(13),
            (CorrectionLevels::L, 426..=458)
            | (CorrectionLevels::M, 332..=362)
            | (CorrectionLevels::Q, 242..=258)
            | (CorrectionLevels::H, 178..=194) => Ok(14),
            (CorrectionLevels::L, 459..=520)
            | (CorrectionLevels::M, 363..=412)
            | (CorrectionLevels::Q, 259..=292)
            | (CorrectionLevels::H, 195..=220) => Ok(15),
            (CorrectionLevels::L, 521..=586)
            | (CorrectionLevels::M, 413..=450)
            | (CorrectionLevels::Q, 293..=322)
            | (CorrectionLevels::H, 221..=250) => Ok(16),
            (CorrectionLevels::L, 587..=644)
            | (CorrectionLevels::M, 451..=504)
            | (CorrectionLevels::Q, 323..=364)
            | (CorrectionLevels::H, 251..=280) => Ok(17),
            (CorrectionLevels::L, 645..=718)
            | (CorrectionLevels::M, 505..=560)
            | (CorrectionLevels::Q, 365..=394)
            | (CorrectionLevels::H, 281..=310) => Ok(18),
            (CorrectionLevels::L, 719..=792)
            | (CorrectionLevels::M, 561..=624)
            | (CorrectionLevels::Q, 395..=442)
            | (CorrectionLevels::H, 311..=338) => Ok(19),
            (CorrectionLevels::L, 793..=858)
            | (CorrectionLevels::M, 625..=666)
            | (CorrectionLevels::Q, 443..=482)
            | (CorrectionLevels::H, 339..=382) => Ok(20),
            (CorrectionLevels::L, 869..=929)
            | (CorrectionLevels::M, 667..=711)
            | (CorrectionLevels::Q, 483..=509)
            | (CorrectionLevels::H, 383..=403) => Ok(21),
            (CorrectionLevels::L, 930..=1003)
            | (CorrectionLevels::M, 712..=779)
            | (CorrectionLevels::Q, 510..=565)
            | (CorrectionLevels::H, 404..=439) => Ok(22),
            (CorrectionLevels::L, 1004..=1091)
            | (CorrectionLevels::M, 780..=857)
            | (CorrectionLevels::Q, 566..=611)
            | (CorrectionLevels::H, 440..=461) => Ok(23),
            (CorrectionLevels::L, 1092..=1171)
            | (CorrectionLevels::M, 858..=911)
            | (CorrectionLevels::Q, 612..=661)
            | (CorrectionLevels::H, 462..=511) => Ok(24),
            (CorrectionLevels::L, 1172..=1273)
            | (CorrectionLevels::M, 912..=997)
            | (CorrectionLevels::Q, 662..=715)
            | (CorrectionLevels::H, 512..=535) => Ok(25),
            (CorrectionLevels::L, 1274..=1367)
            | (CorrectionLevels::M, 998..=1059)
            | (CorrectionLevels::Q, 716..=751)
            | (CorrectionLevels::H, 536..=593) => Ok(26),
            (CorrectionLevels::L, 1368..=1465)
            | (CorrectionLevels::M, 1060..=1125)
            | (CorrectionLevels::Q, 752..=805)
            | (CorrectionLevels::H, 594..=625) => Ok(27),
            (CorrectionLevels::L, 1466..=1528)
            | (CorrectionLevels::M, 1126..=1190)
            | (CorrectionLevels::Q, 806..=868)
            | (CorrectionLevels::H, 626..=658) => Ok(28),
            (CorrectionLevels::L, 1529..=1628)
            | (CorrectionLevels::M, 1191..=1264)
            | (CorrectionLevels::Q, 869..=908)
            | (CorrectionLevels::H, 659..=698) => Ok(29),
            (CorrectionLevels::L, 1629..=1732)
            | (CorrectionLevels::M, 1265..=1370)
            | (CorrectionLevels::Q, 909..=982)
            | (CorrectionLevels::H, 699..=742) => Ok(30),
            (CorrectionLevels::L, 1733..=1840)
            | (CorrectionLevels::M, 1371..=1452)
            | (CorrectionLevels::Q, 983..=1030)
            | (CorrectionLevels::H, 743..=790) => Ok(31),
            (CorrectionLevels::L, 1841..=1952)
            | (CorrectionLevels::M, 1453..=1538)
            | (CorrectionLevels::Q, 1031..=1112)
            | (CorrectionLevels::H, 791..=842) => Ok(32),
            (CorrectionLevels::L, 1953..=2068)
            | (CorrectionLevels::M, 1539..=1628)
            | (CorrectionLevels::Q, 1113..=1168)
            | (CorrectionLevels::H, 843..=898) => Ok(33),
            (CorrectionLevels::L, 2069..=2188)
            | (CorrectionLevels::M, 1629..=1722)
            | (CorrectionLevels::Q, 1169..=1228)
            | (CorrectionLevels::H, 899..=958) => Ok(34),
            (CorrectionLevels::L, 2189..=2303)
            | (CorrectionLevels::M, 1723..=1809)
            | (CorrectionLevels::Q, 1229..=1283)
            | (CorrectionLevels::H, 959..=983) => Ok(35),
            (CorrectionLevels::L, 2304..=2431)
            | (CorrectionLevels::M, 1810..=1911)
            | (CorrectionLevels::Q, 1284..=1351)
            | (CorrectionLevels::H, 984..=1051) => Ok(36),
            (CorrectionLevels::L, 2432..=2563)
            | (CorrectionLevels::M, 1912..=1989)
            | (CorrectionLevels::Q, 1352..=1423)
            | (CorrectionLevels::H, 1052..=1093) => Ok(37),
            (CorrectionLevels::L, 2564..=2699)
            | (CorrectionLevels::M, 1990..=2099)
            | (CorrectionLevels::Q, 1424..=1499)
            | (CorrectionLevels::H, 1094..=1139) => Ok(38),
            (CorrectionLevels::L, 2700..=2809)
            | (CorrectionLevels::M, 2100..=2213)
            | (CorrectionLevels::Q, 1500..=1579)
            | (CorrectionLevels::H, 1140..=1219) => Ok(39),
            (CorrectionLevels::L, 2810..=2953)
            | (CorrectionLevels::M, 2214..=2331)
            | (CorrectionLevels::Q, 1580..=1663)
            | (CorrectionLevels::H, 1220..=1273) => Ok(40),
            (CorrectionLevels::DetectionOnly, _) => Err(EncodingError::new(
                "Can't simply detect errors in a Standard QR code",
            )),
            _ => Err(EncodingError::new(
                "Too much data for Numeric mode and error correction level",
            )),
        }
    }

    fn calculate_standard_kanji_version(
        _correction: &CorrectionLevels,
        _data: &String,
    ) -> Result<u32, EncodingError> {
        Err(EncodingError::new("Kanji not yet supported"))
        // match (correction, data.len()) {
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(1),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(2),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(3),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(4),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(5),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(6),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(7),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(8),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(9),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(10),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(11),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(12),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(13),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(14),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(15),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(16),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(17),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(18),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(19),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(20),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(21),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(22),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(23),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(24),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(25),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(26),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(27),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(28),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(29),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(30),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(31),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(32),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(33),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(34),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(35),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(36),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(37),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(38),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(39),
        //   (CorrectionLevels::L, ..=) | (CorrectionLevels::M, ..=) | (CorrectionLevels::Q, ..=) | (CorrectionLevels::H, ..=) => Ok(40),
        //   (CorrectionLevels::DetectionOnly, _) => Err(EncodingError::new("Can't simply detect errors in a Standard QR code")),
        //   _ => Err(EncodingError::new("Too much data for Numeric mode and error correction level"))
        // }
    }
}

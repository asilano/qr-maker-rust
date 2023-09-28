mod encoder;
mod error_correction;
mod qr_errors;
mod qr_types;
mod sizer;
use encoder::{Encoder, EncodingModes};
use error_correction::CorrectionLevels;
use image::{imageops, GrayImage, ImageBuffer, Luma};
use qr_errors::QRError;
use qr_types::{FinderLocations, QRFactory};
use sizer::Sizer;
pub use qr_types::QRSymbolTypes;

#[derive(Default)]
pub struct Options {
    pub filepath: Option<String>,
    pub qr_type: Option<QRSymbolTypes>,
    pub version: Option<u32>,
    pub mode: Option<EncodingModes>,
    pub correction_level: Option<CorrectionLevels>,
}

pub struct QRGenerator {
    pub options: Options,
}
impl QRGenerator {
    pub fn default() -> Self {
        Self {
            options: Options {
                correction_level: Some(CorrectionLevels::Q),
                ..Default::default()
            },
        }
    }

    pub fn make_qr_code(&mut self, data: String) -> Result<String, QRError> {
        // Unless specified, assume a QRCode (not a MicroQR)
        if self.options.qr_type.is_none() {
            self.options.qr_type = Some(QRSymbolTypes::QRCode);
        }

        // Unless specified, assume Q-level correction
        if self.options.correction_level.is_none() {
            self.options.correction_level = Some(CorrectionLevels::Q);
        }

        // Work out how large the QR code needs to be
        if self.options.version.is_none() {
          self.options.version = Some(Sizer::calculate_version(&self.options, &data)?);
        }

        let mut encoder = Encoder::new(&self, data);
        encoder.encode_data_into_byte_stream()?;
        let data_codewords = &encoder.output_data;
        println!("{:?}", data_codewords);

        // let data_blocks: Vec<Vec<u8>> = split_data_into_blocks(&data_codewords);
        // let err_correct_blocks: Vec<Vec<u8>> = generate_err_correction(&data_blocks);
        // let message_sequence: Vec<u8> = interleave_data_and_err_correct(data_blocks, err_correct_blocks);
        let message_sequence: Vec<u8> = vec![];
        let unmasked_image: GrayImage = self.build_qr_image(message_sequence);
        // let masked_image: GrayImage = mask_qr_image(unmasked_image);
        // generate_format_and_version_info(...);
        self.save_qr_image(&"./qr_code.png".to_string(), unmasked_image)?;
        Ok("./qr_code.png".to_string())
    }

    fn build_qr_image(&self, message: Vec<u8>) -> GrayImage {
        let qr_code = QRFactory::build_code(QRSymbolTypes::QRCode, 2);

        let dimension = qr_code.module_width();
        let mut loud_region: GrayImage = ImageBuffer::from_pixel(dimension, dimension, Luma([128]));

        Self::add_timing_patterns(&mut loud_region, qr_code.timing_coord());
        Self::add_finder_patterns(&mut loud_region, qr_code.finder_locations());
        Self::add_alignment_patterns(&mut loud_region, qr_code.alignment_locations());
        loud_region
    }

    fn save_qr_image(&self, filepath: &String, loud_region: GrayImage) -> Result<(), QRError> {
        let dimension = loud_region.width();
        let quiet_width = 4;
        let full_dimension = dimension + quiet_width * 2;
        let mut full_image: GrayImage =
            ImageBuffer::from_fn(full_dimension, full_dimension, |x, y| {
                if x < quiet_width || x >= dimension || y < quiet_width || y >= dimension {
                    Luma([255])
                } else {
                    Luma([128])
                }
            });

        imageops::overlay(
            &mut full_image,
            &loud_region,
            quiet_width as i64,
            quiet_width as i64,
        );
        let scale_factor = 5;
        let scaled_size = full_dimension * scale_factor;
        let scaled_image = imageops::resize(
            &full_image,
            scaled_size,
            scaled_size,
            imageops::FilterType::Nearest,
        );
        scaled_image.save(filepath)?;

        Ok(())
    }

    fn add_timing_patterns(buffer: &mut GrayImage, timing_coord: u32) {
        let horiz: GrayImage = ImageBuffer::from_fn(buffer.width(), 1, |x, _| {
            if x % 2 == 0 {
                Luma([0])
            } else {
                Luma([255])
            }
        });

        imageops::overlay(buffer, &horiz, 0, timing_coord as i64);
        imageops::overlay(buffer, &imageops::rotate90(&horiz), timing_coord as i64, 0);
    }

    fn add_finder_patterns(buffer: &mut GrayImage, locations: Vec<FinderLocations>) {
        for location in locations {
            match location {
                FinderLocations::TopLeft => Self::add_finder_pattern(buffer, 0, 0),
                FinderLocations::BottomLeft => {
                    Self::add_finder_pattern(buffer, 0, buffer.height() as i64 - 7)
                }
                FinderLocations::TopRight => {
                    Self::add_finder_pattern(buffer, buffer.width() as i64 - 7, 0)
                }
            }
        }
    }

    fn add_finder_pattern(buffer: &mut GrayImage, left: i64, top: i64) {
        let finder: GrayImage = ImageBuffer::from_fn(9, 9, |x, y| {
            if x == 0 || y == 0 || x == 8 || y == 8 {
                Luma([255])
            } else if x == 1 || y == 1 || x == 7 || y == 7 {
                Luma([0])
            } else if x == 2 || y == 2 || x == 6 || y == 6 {
                Luma([255])
            } else {
                Luma([0])
            }
        });

        imageops::overlay(buffer, &finder, left - 1, top - 1);
    }

    fn add_alignment_patterns(buffer: &mut GrayImage, locations: Vec<(u32, u32)>) {
        for (cx, cy) in locations {
            let five: GrayImage = ImageBuffer::from_pixel(5, 5, Luma([0]));
            let three: GrayImage = ImageBuffer::from_pixel(3, 3, Luma([255]));
            imageops::overlay(buffer, &five, (cx as i64) - 2, (cy as i64) - 2);
            imageops::overlay(buffer, &three, (cx as i64) - 1, (cy as i64) - 1);
            buffer.put_pixel(cx, cy, Luma([0]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

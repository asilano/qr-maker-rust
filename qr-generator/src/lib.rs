mod encoder;
mod error_correction;
mod qr_errors;
mod qr_types;
mod sizer;
mod image_builder;
use encoder::Encoder;
pub use encoder::EncodingModes;
pub use error_correction::CorrectionLevels;
use image::{imageops, GrayImage, ImageBuffer, Luma};
use qr_errors::QRError;
pub use qr_types::QRSymbolTypes;
use sizer::Sizer;

use crate::{error_correction::ErrorCorrector, image_builder::ImageBuilder};

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
        let data_bitstream = &encoder.output_data;
        let data_codewords = data_bitstream.clone().into_vec();

        let mut error_corrector = ErrorCorrector::from(&Sizer::error_correction_shape(
            self.options.qr_type.as_ref().unwrap(),
            self.options.version.unwrap(),
            self.options.correction_level.as_ref().unwrap(),
        ));
        error_corrector.fill_data_into_blocks(data_codewords)?;
        error_corrector.generate_error_correction();

        let message_sequence: Vec<u8> = error_corrector.interleave().collect();

        let mut image_builder = ImageBuilder::new(self.options.qr_type.unwrap(), self.options.version.unwrap(), &message_sequence, self.options.correction_level.unwrap());
        image_builder.build_qr_image();

        self.save_qr_image(&"./qr_code.png".to_string(), image_builder.get_image())?;
        Ok("./qr_code.png".to_string())
    }

    fn save_qr_image(&self, filepath: &String, loud_region: &GrayImage) -> Result<(), QRError> {
        let dimension = loud_region.width();
        let quiet_width = 4;
        let full_dimension = dimension + quiet_width * 2;
        let mut full_image: GrayImage =
            ImageBuffer::from_pixel(full_dimension, full_dimension, Luma([255]));

        imageops::overlay(
            &mut full_image,
            loud_region,
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
}

#[cfg(test)]
mod tests {
    use super::*;
}

mod qr_types;
use std::error::Error;
use image::{ImageBuffer, GrayImage, imageops};
use qr_types::{QRFactory, QRSymbolTypes};

pub fn save_qr_image(filepath: &String) -> Result<(), Box<dyn Error>> {
    let qr_code = QRFactory::build_code(QRSymbolTypes::QRCode, 1);

    let dimension = qr_code.module_width();
    let mut loud_region: GrayImage = ImageBuffer::from_pixel(dimension, dimension, image::Luma([0]));
    add_finder_patterns(&mut loud_region);

    let quiet_width = 4;
    let full_dimension = dimension + quiet_width * 2;
    let mut full_image: GrayImage = ImageBuffer::from_fn(full_dimension, full_dimension, |x, y| {
        if x < quiet_width || x >= dimension || y < quiet_width || y >= dimension {
            image::Luma([255])
        } else {
            image::Luma([128])
        }
    });

    imageops::overlay(&mut full_image, &loud_region, quiet_width as i64, quiet_width as i64);
    let scale_factor = 5;
    let scaled_size = full_dimension * scale_factor;
    let scaled_image = imageops::resize(&full_image, scaled_size, scaled_size, imageops::FilterType::Nearest);
    scaled_image.save(filepath)?;

    Ok(())
}

fn add_finder_patterns(buffer: &mut GrayImage) {
    add_finder_pattern(buffer, 0, 0);
    add_finder_pattern(buffer, 0, buffer.height() as i64 - 7);
    add_finder_pattern(buffer, buffer.width() as i64 - 7, 0);
}

fn add_finder_pattern(buffer: &mut GrayImage, left: i64, top: i64) {
    let finder: GrayImage = ImageBuffer::from_fn(9, 9, |x, y| {
        if x == 0 || y == 0 || x == 8 || y == 8 {
            image::Luma([255])
        }
        else if x == 1 || y == 1 || x == 7 || y == 7 {
            image::Luma([0])
        }
        else if x == 2 || y == 2 || x == 6 || y == 6 {
            image::Luma([255])
        }
        else {
            image::Luma([0])
        }
    });

    imageops::overlay(buffer, &finder, left - 1, top - 1);
}


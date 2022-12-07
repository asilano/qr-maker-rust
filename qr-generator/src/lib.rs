mod qr_types;
use std::error::Error;
use image::{ImageBuffer, GrayImage, Luma, imageops};
use qr_types::{QRFactory, QRSymbolTypes, FinderLocations};

pub fn save_qr_image(filepath: &String) -> Result<(), Box<dyn Error>> {
    let qr_code = QRFactory::build_code(QRSymbolTypes::QRCode, 2);

    let dimension = qr_code.module_width();
    let mut loud_region: GrayImage = ImageBuffer::from_pixel(dimension, dimension, Luma([128]));

    add_timing_patterns(&mut loud_region, qr_code.timing_coord());
    add_finder_patterns(&mut loud_region, qr_code.finder_locations());
    add_alignment_patterns(&mut loud_region, qr_code.alignment_locations());

    let quiet_width = 4;
    let full_dimension = dimension + quiet_width * 2;
    let mut full_image: GrayImage = ImageBuffer::from_fn(full_dimension, full_dimension, |x, y| {
        if x < quiet_width || x >= dimension || y < quiet_width || y >= dimension {
            Luma([255])
        } else {
            Luma([128])
        }
    });

    imageops::overlay(&mut full_image, &loud_region, quiet_width as i64, quiet_width as i64);
    let scale_factor = 5;
    let scaled_size = full_dimension * scale_factor;
    let scaled_image = imageops::resize(&full_image, scaled_size, scaled_size, imageops::FilterType::Nearest);
    scaled_image.save(filepath)?;

    Ok(())
}

fn add_timing_patterns(buffer: &mut GrayImage, timing_coord: u32) {
    let horiz: GrayImage = ImageBuffer::from_fn(buffer.width(), 1, |x, _| {
        if x % 2 == 0 { Luma([0]) } else { Luma([255]) }
    });

    imageops::overlay(buffer, &horiz, 0, timing_coord as i64);
    imageops::overlay(buffer, &imageops::rotate90(&horiz), timing_coord as i64, 0);
}

fn add_finder_patterns(buffer: &mut GrayImage, locations: Vec<FinderLocations>) {
    for location in locations {
        match location {
            FinderLocations::TopLeft => add_finder_pattern(buffer, 0, 0),
            FinderLocations::BottomLeft => add_finder_pattern(buffer, 0, buffer.height() as i64 - 7),
            FinderLocations::TopRight => add_finder_pattern(buffer, buffer.width() as i64 - 7, 0)
        }
    }
}

fn add_finder_pattern(buffer: &mut GrayImage, left: i64, top: i64) {
    let finder: GrayImage = ImageBuffer::from_fn(9, 9, |x, y| {
        if x == 0 || y == 0 || x == 8 || y == 8 {
            Luma([255])
        }
        else if x == 1 || y == 1 || x == 7 || y == 7 {
            Luma([0])
        }
        else if x == 2 || y == 2 || x == 6 || y == 6 {
            Luma([255])
        }
        else {
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

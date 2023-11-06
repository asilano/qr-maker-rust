use image::{GrayImage, ImageBuffer, Luma, imageops};
use polynomial_arithmetic::{Polynomial, IntMod, One, Zero};

use crate::{
    QRSymbolTypes,
    qr_types::{QRSymbol, QRFactory, FinderLocations}, error_correction::CorrectionLevels
};
pub struct ImageBuilder<'a> {
    qr_code: Box<dyn QRSymbol>,
    message: &'a Vec<u8>,
    loud_region: Option<GrayImage>,
    correction_level: CorrectionLevels
}

impl<'a> ImageBuilder<'a> {
    pub fn new(qr_type: QRSymbolTypes, version: u32, message: &'a Vec<u8>, correction_level: CorrectionLevels) -> Self {
        Self {
            qr_code: QRFactory::build_code(qr_type, version),
            message,
            loud_region: None,
            correction_level
        }
    }

    pub fn build_qr_image(&mut self) {
        let dimension = self.qr_code.module_width();
        self.loud_region = Some(ImageBuffer::from_pixel(dimension, dimension, Luma([128])));

        self.add_timing_patterns(self.qr_code.timing_coord());
        self.add_finder_patterns(self.qr_code.finder_locations());
        self.add_alignment_patterns(self.qr_code.alignment_locations());
        self.reserve_format_and_version_space(self.qr_code.finder_locations(), self.qr_code.include_version_locations());
        self.add_message_stream();
        let chosen_mask = self.mask_data_area();
        self.add_format_information(chosen_mask, self.qr_code.finder_locations());
        if self.qr_code.include_version_locations() {
            self.add_version_information();
        }
        self.recolour_function_pixels();
    }

    pub fn get_image(&self) -> &GrayImage {
        self.loud_region.as_ref().unwrap()
    }

    fn white() -> Luma<u8> { Luma([255]) }
    fn black() -> Luma<u8> { Luma([0]) }
    fn fn_white() -> Luma<u8> { Luma([200]) }
    fn fn_black() -> Luma<u8> { Luma([50]) }

    fn add_timing_patterns(&mut self, timing_coord: u32) {
        let buffer = self.loud_region.as_mut().unwrap();
        let horiz: GrayImage = ImageBuffer::from_fn(buffer.width(), 1, |x, _| {
            if x % 2 == 0 {
                Self::fn_black()
            } else {
                Self::fn_white()
            }
        });

        imageops::overlay(buffer, &horiz, 0, timing_coord as i64);
        imageops::overlay(buffer, &imageops::rotate90(&horiz), timing_coord as i64, 0);
    }

    fn add_finder_patterns(&mut self, locations: Vec<FinderLocations>) {
        let buffer = self.loud_region.as_mut().unwrap();
        for location in locations {
            match location {
                FinderLocations::TopLeft => {
                    Self::add_finder_pattern(buffer, 0, 0, Self::fn_white(), Self::fn_black())
                }
                FinderLocations::BottomLeft => {
                    Self::add_finder_pattern(buffer, 0, buffer.height() as i64 - 7, Self::fn_white(), Self::fn_black())
                }
                FinderLocations::TopRight => {
                    Self::add_finder_pattern(buffer, buffer.width() as i64 - 7, 0, Self::fn_white(), Self::fn_black())
                }
            }
        }
    }

    fn add_finder_pattern(buffer: &mut GrayImage, left: i64, top: i64, white: Luma<u8>, black: Luma<u8>) {
        let finder: GrayImage = ImageBuffer::from_fn(9, 9, |x, y| {
            if x == 0 || y == 0 || x == 8 || y == 8 {
                white
            } else if x == 1 || y == 1 || x == 7 || y == 7 {
                black
            } else if x == 2 || y == 2 || x == 6 || y == 6 {
                white
            } else {
                black
            }
        });

        imageops::overlay(buffer, &finder, left - 1, top - 1);
    }

    fn add_alignment_patterns(&mut self, locations: Vec<(u32, u32)>) {
        let buffer = self.loud_region.as_mut().unwrap();
        for (cx, cy) in locations {
            let five: GrayImage = ImageBuffer::from_pixel(5, 5, Self::fn_black());
            let three: GrayImage = ImageBuffer::from_pixel(3, 3, Self::fn_white());
            imageops::overlay(buffer, &five, (cx as i64) - 2, (cy as i64) - 2);
            imageops::overlay(buffer, &three, (cx as i64) - 1, (cy as i64) - 1);
            buffer.put_pixel(cx, cy, Self::fn_black());
        }
    }

    fn reserve_format_and_version_space(&mut self, locations: Vec<FinderLocations>, include_versions: bool) {
        let buffer = self.loud_region.as_mut().unwrap();
        for location in locations {
            match location {
                FinderLocations::TopLeft => {
                    for n in 0..9 {
                        buffer.put_pixel(8, n, Self::fn_black());
                        buffer.put_pixel(n, 8, Self::fn_black());
                    }
                },
                FinderLocations::BottomLeft => {
                    for n in 0..8 {
                        buffer.put_pixel(8, buffer.height() - 1 - n, Self::fn_black());
                    }
                }
                FinderLocations::TopRight => {
                    for n in 0..8 {
                        buffer.put_pixel(buffer.width() - 1 - n, 8, Self::fn_black());
                    }
                }
            }
        }

        if include_versions {
            let tr_region: GrayImage = ImageBuffer::from_pixel(3, 6, Self::fn_black());
            let bl_region: GrayImage = ImageBuffer::from_pixel(6, 3, Self::fn_black());
            imageops::overlay(buffer, &tr_region, buffer.width() as i64 - 11, 0);
            imageops::overlay(buffer, &bl_region, 0, buffer.height() as i64 - 11);
        }
    }

    fn add_message_stream(&mut self) {
        let bits = self.message.iter().flat_map(|n| {
            (0..8).map(move |b| (n >> (7 - b)) % 2)
        });
        let loud_copy = self.loud_region.as_ref().unwrap().clone();
        let cells = MessageCells::new(&loud_copy, self.qr_code.timing_coord());
        for (bit, (x, y)) in bits.zip(cells) {
            let colour = if bit == 1 { Self::black() } else { Self::white() };
            self.loud_region.as_mut().unwrap().put_pixel(x, y, colour);
        }
    }

    // Returns mask identifier, MSB first
    fn mask_data_area(&mut self) -> Vec<u8> {
        let mask_candidates = self.qr_code.mask_functions();

        let (mask_number, mask) = mask_candidates.iter().enumerate().max_by_key(|(_, mask)| {
            let mut loud_copy = self.loud_region.as_ref().unwrap().clone();
            Self::apply_mask(&mut loud_copy, mask);
            self.qr_code.score_masked_image(&loud_copy)
        }).unwrap();

        // Found best mask; apply it to the real image
        Self::apply_mask(self.loud_region.as_mut().unwrap(), mask);

        if mask_candidates.len() > 4 {
            vec![(mask_number as u8 >> 2) % 2,
                (mask_number as u8 >> 1) % 2,
                mask_number as u8 % 2]
        } else {
            vec![(mask_number as u8 >> 1) % 2,
                mask_number as u8 % 2]
        }
    }

    fn apply_mask(image: &mut GrayImage, mask_fn: &dyn Fn(u32, u32) -> bool) {
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            if *pixel == Self::white() || *pixel == Self::black() {
                // Not a function pixel
                if (*pixel == Self::black()) == mask_fn(x, y) {
                    *pixel = Self::white();
                } else {
                    *pixel = Self::black();
                }
            }
        }
    }

    fn add_format_information(&mut self, mask_bits: Vec<u8>, locations: Vec<FinderLocations>) {
        let mut format_bits = self.qr_code.ec_level_bits(self.correction_level);
        format_bits.extend(mask_bits.iter());
        assert!(format_bits.len() == 5);

        let one = IntMod::<2>::one();
        let zero = IntMod::<2>::zero();
        let mut format_poly = Polynomial::<IntMod<2>>::from(format_bits.iter().rev().map(|&b| IntMod::<2>::from(b as u32)).collect::<Vec<IntMod<2>>>());
        let mut x10 = vec![zero; 10];
        x10.push(one);
        format_poly = format_poly * Polynomial::<IntMod<2>>::from(x10);

        let ec_generator = Polynomial::<IntMod<2>>::from(vec![one, one, one, zero, one, one, zero, zero, one, zero, one]);
        let ec_poly = format_poly % ec_generator;
        let ec_len = ec_poly.coefficients.len();
        for _ in 0..(10 - ec_len) {
            format_bits.push(0);
        }
        format_bits.extend(ec_poly.coefficients.iter().rev().map(|bit| bit.value as u8));

        let mask = self.qr_code.format_mask();
        for (data, mask) in format_bits.iter_mut().zip(mask.iter()) {
            if *data == *mask {
                *data = 0;
            } else {
                *data = 1;
            }
        }

        // Apply the final format info into the reserved areas
        let timing_coord = self.qr_code.timing_coord();
        let buffer = self.loud_region.as_mut().unwrap();
        for location in locations {
            let mut format_iter = format_bits.iter().rev();
            match location {
                FinderLocations::TopLeft => {
                    for n in 0..9 {
                        if n != timing_coord {
                            let pixel = match format_iter.next() {
                                Some(0) => Self::white(),
                                Some(1) => Self::black(),
                                _ => unreachable!()
                            };
                            buffer.put_pixel(8, n, pixel);
                        }
                    }
                    for n in 1..9 {
                        if 8 - n != timing_coord {
                            let pixel = match format_iter.next() {
                                Some(0) => Self::white(),
                                Some(1) => Self::black(),
                                _ => unreachable!()
                            };
                            buffer.put_pixel(8 - n, 8, pixel);
                        }
                    }
                },
                FinderLocations::BottomLeft => {
                    let mut second_half = format_iter.skip(8);
                    for n in 0..7 {
                        let pixel = match second_half.next() {
                            Some(0) => Self::white(),
                            Some(1) => Self::black(),
                            _ => unreachable!()
                        };
                        buffer.put_pixel(8, buffer.height() - 7 + n, pixel);
                    }
                }
                FinderLocations::TopRight => {
                    for n in 0..8 {
                        let pixel = match format_iter.next() {
                            Some(0) => Self::white(),
                            Some(1) => Self::black(),
                            _ => unreachable!()
                        };
                        buffer.put_pixel(buffer.width() - 1 - n, 8, pixel);
                    }
                }
            }
        }

    }

    fn add_version_information(&mut self) {
        let version = self.qr_code.version();
        let version_bits = (0..6).map(|b| IntMod::<2>::from((version >> b) % 2)).collect::<Vec<IntMod<2>>>();
        let mut version_poly = Polynomial::<IntMod<2>>::from(version_bits.clone());

        let one = IntMod::<2>::one();
        let zero = IntMod::<2>::zero();
        let mut x12 = vec![zero; 12];
        x12.push(one);
        version_poly = version_poly * Polynomial::<IntMod<2>>::from(x12);

        let ec_generator = Polynomial::<IntMod<2>>::from(vec![one, zero, one, zero, zero, one, zero, zero, one, one, one, one, one]);
        let ec_poly = &version_poly % &ec_generator;
        version_poly = version_poly + ec_poly;

        let mut version_area: GrayImage = ImageBuffer::from_pixel(3, 6, Luma([255]));
        for (index, bit) in version_poly.coefficients.iter().enumerate() {
            if bit.value == 1 {
                version_area.put_pixel(index as u32 % 3, index as u32 / 3, Luma([0]));
            }
        }

        let buffer = self.loud_region.as_mut().unwrap();
        imageops::overlay(buffer, &version_area, buffer.width() as i64 - 11, 0);
        version_area = imageops::rotate270(&version_area);
        imageops::flip_vertical_in_place(&mut version_area);
        imageops::overlay(buffer, &version_area, 0, buffer.height() as i64 - 11);
    }

    fn recolour_function_pixels(&mut self) {
        for pixel in self.loud_region.as_mut().unwrap().pixels_mut() {
            *pixel = if pixel.0[0] < 128 {
                Luma([0])
            } else {
                Luma([255])
            }
        }
    }
}

struct MessageCells<'a> {
    loud_region: &'a GrayImage,
    timing_column: u32,
    left_col_x_index: u32,
    y_index: u32,
    left: bool,
    up: bool,
    first: bool
}
impl<'a> MessageCells<'a> {
    pub fn new(loud_region: &'a GrayImage, timing_column: u32) -> Self {
        let left_col_x_index = loud_region.width() - 2;
        let y_index = loud_region.height() - 1;
        Self {
            loud_region,
            timing_column,
            left_col_x_index,
            y_index,
            left: false,
            up: true,
            first: true
        }
    }

    fn advance_cell(&mut self) -> Option<()> {
        if self.first {
            self.first = false;
            return Some(());
        }

        if !self.left {
            self.left = true;
            return Some(());
        }

        self.left = false;
        match (self.up, self.y_index, self.loud_region.height() - 1 - self.y_index) {
            (true, 0, _) => {
                // Currently at left-top going up. Move one column-pair left, right cell, downwards
                if self.left_col_x_index == 0 {
                    None // Nowhere to go left!
                } else {
                    self.left_col_x_index -= 2;
                    if self.left_col_x_index + 1 == self.timing_column {
                        self.left_col_x_index -= 1;
                    }
                    self.up = false;
                    Some(())
                }
            },
            (true, _, _) => {
                // Room to move up one, into right cell
                self.y_index -= 1;
                Some(())
            },
            (false, _, 0) => {
                // Currently at left-bottom going down. Move one column-pair left, right cell, upwards
                if self.left_col_x_index == 0 {
                    None // Nowhere to go left!
                } else {
                    self.left_col_x_index -= 2;
                    if self.left_col_x_index + 1 == self.timing_column {
                        self.left_col_x_index -= 1;
                    }
                    self.up = true;
                    Some(())
                }
            },
            (false, _, _) => {
                // Room to move down one, into right cell
                self.y_index += 1;
                Some(())
            },
        }
    }
}
impl<'a> Iterator for MessageCells<'a> {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        self.advance_cell()?;

        let mut x = self.left_col_x_index + if self.left { 0 } else { 1 };
        while self.loud_region.get_pixel(x, self.y_index) != &Luma([128]) {
            self.advance_cell()?;
            x = self.left_col_x_index + if self.left { 0 } else { 1 };
        }

        Some((x, self.y_index))
    }
}
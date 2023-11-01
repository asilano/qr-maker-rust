use image::{GrayImage, ImageBuffer, Luma, imageops};

use crate::{
    QRSymbolTypes,
    qr_types::{QRSymbol, QRFactory, FinderLocations}
};
pub struct ImageBuilder<'a> {
    qr_code: Box<dyn QRSymbol>,
    message: &'a Vec<u8>,
    loud_region: Option<GrayImage>,
    white: Luma<u8>,
    black: Luma<u8>,
    fn_white: Luma<u8>,
    fn_black: Luma<u8>
}

impl<'a> ImageBuilder<'a> {
    pub fn new(qr_type: QRSymbolTypes, version: u32, message: &'a Vec<u8>) -> Self {
        Self {
            qr_code: QRFactory::build_code(qr_type, version),
            message,
            loud_region: None,
            white: Luma([255]),
            black: Luma([0]),
            fn_white: Luma([200]),
            fn_black: Luma([50])
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
    }

    pub fn get_image(&self) -> &GrayImage {
        self.loud_region.as_ref().unwrap()
    }

    fn add_timing_patterns(&mut self, timing_coord: u32) {
        let buffer = self.loud_region.as_mut().unwrap();
        let horiz: GrayImage = ImageBuffer::from_fn(buffer.width(), 1, |x, _| {
            if x % 2 == 0 {
                self.fn_black
            } else {
                self.fn_white
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
                    Self::add_finder_pattern(buffer, 0, 0, self.fn_white, self.fn_black)
                }
                FinderLocations::BottomLeft => {
                    Self::add_finder_pattern(buffer, 0, buffer.height() as i64 - 7, self.fn_white, self.fn_black)
                }
                FinderLocations::TopRight => {
                    Self::add_finder_pattern(buffer, buffer.width() as i64 - 7, 0, self.fn_white, self.fn_black)
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
            let five: GrayImage = ImageBuffer::from_pixel(5, 5, self.fn_black);
            let three: GrayImage = ImageBuffer::from_pixel(3, 3, self.fn_white);
            imageops::overlay(buffer, &five, (cx as i64) - 2, (cy as i64) - 2);
            imageops::overlay(buffer, &three, (cx as i64) - 1, (cy as i64) - 1);
            buffer.put_pixel(cx, cy, self.fn_black);
        }
    }

    fn reserve_format_and_version_space(&mut self, locations: Vec<FinderLocations>, include_versions: bool) {
        let buffer = self.loud_region.as_mut().unwrap();
        for location in locations {
            match location {
                FinderLocations::TopLeft => {
                    for n in 0..9 {
                        buffer.put_pixel(8, n, self.fn_black);
                        buffer.put_pixel(n, 8, self.fn_black);
                    }
                },
                FinderLocations::BottomLeft => {
                    for n in 0..8 {
                        buffer.put_pixel(8, buffer.height() - 1 - n, self.fn_black);
                    }
                }
                FinderLocations::TopRight => {
                    for n in 0..8 {
                        buffer.put_pixel(buffer.width() - 1 - n, 8, self.fn_black);
                    }
                }
            }
        }

        if include_versions {
            let tr_region: GrayImage = ImageBuffer::from_pixel(3, 6, self.fn_black);
            let bl_region: GrayImage = ImageBuffer::from_pixel(6, 3, self.fn_black);
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
            let colour = if bit == 1 { self.black } else { self.white };
            self.loud_region.as_mut().unwrap().put_pixel(x, y, colour);
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
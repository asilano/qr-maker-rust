#[derive(Debug, PartialEq)]
pub enum FinderLocations {
    TopLeft,
    TopRight,
    BottomLeft,
}
use itertools::Itertools;
use FinderLocations::*;

pub trait QRSymbol {
    fn module_width(&self) -> u32;
    fn timing_coord(&self) -> u32;
    fn finder_locations(&self) -> Vec<FinderLocations>;
    fn alignment_locations(&self) -> Vec<(u32, u32)>;
    fn format_locations(&self) -> Vec<FinderLocations>;
    fn include_version_locations(&self) -> bool;
}

pub struct QRCode {
    version: u32,
}
impl QRCode {
    fn alignment_coords(&self) -> Vec<u32> {
        match self.version {
            1 => vec![],
            2 => vec![6, 18],
            3 => vec![6, 22],
            4 => vec![6, 26],
            5 => vec![6, 30],
            6 => vec![6, 34],
            7 => vec![6, 22, 38],
            8 => vec![6, 24, 42],
            9 => vec![6, 26, 46],
            10 => vec![6, 28, 50],
            11 => vec![6, 30, 54],
            12 => vec![6, 32, 58],
            13 => vec![6, 34, 62],
            14 => vec![6, 26, 46, 66],
            15 => vec![6, 26, 48, 70],
            16 => vec![6, 26, 50, 74],
            17 => vec![6, 30, 54, 78],
            18 => vec![6, 30, 56, 82],
            19 => vec![6, 30, 58, 86],
            20 => vec![6, 34, 62, 90],
            21 => vec![6, 28, 50, 72, 94],
            22 => vec![6, 26, 50, 74, 98],
            23 => vec![6, 30, 54, 76, 102],
            24 => vec![6, 28, 54, 78, 106],
            25 => vec![6, 32, 58, 80, 110],
            26 => vec![6, 30, 58, 82, 114],
            27 => vec![6, 34, 62, 84, 118],
            28 => vec![6, 26, 50, 74, 98, 122],
            29 => vec![6, 30, 54, 78, 102, 126],
            30 => vec![6, 26, 52, 78, 104, 130],
            31 => vec![6, 30, 56, 82, 108, 134],
            32 => vec![6, 34, 60, 86, 112, 138],
            33 => vec![6, 30, 58, 86, 114, 142],
            34 => vec![6, 34, 62, 90, 118, 146],
            35 => vec![6, 30, 54, 78, 102, 126, 150],
            36 => vec![6, 24, 50, 76, 102, 128, 154],
            37 => vec![6, 28, 54, 80, 106, 132, 158],
            38 => vec![6, 32, 58, 84, 110, 136, 162],
            39 => vec![6, 26, 54, 82, 110, 138, 166],
            40 => vec![6, 30, 58, 86, 114, 142, 170],
            _ => unreachable!(),
        }
    }
}
impl QRSymbol for QRCode {
    fn module_width(&self) -> u32 {
        21 + 4 * (self.version - 1)
    }
    fn timing_coord(&self) -> u32 {
        6
    }
    fn finder_locations(&self) -> Vec<FinderLocations> {
        vec![TopLeft, TopRight, BottomLeft]
    }
    fn alignment_locations(&self) -> Vec<(u32, u32)> {
        let coords = self.alignment_coords();
        if coords.is_empty() {
            return vec![];
        }

        let min = coords[0];
        let max = *coords.last().unwrap();

        coords
            .iter()
            .cartesian_product(coords.iter())
            .filter_map(|(x, y)| {
                if (*x == min && *y == min) || (*x == min && *y == max) || (*x == max && *y == min)
                {
                    None
                } else {
                    Some((*x, *y))
                }
            })
            .collect()
    }
    fn format_locations(&self) -> Vec<FinderLocations> {
        vec![TopLeft, TopRight, BottomLeft]
    }
    fn include_version_locations(&self) -> bool {
        self.version >= 7
    }
}

pub struct MicroQRCode {
    version: u32,
}
impl QRSymbol for MicroQRCode {
    fn module_width(&self) -> u32 {
        11 + 2 * (self.version - 1)
    }
    fn timing_coord(&self) -> u32 {
        0
    }
    fn finder_locations(&self) -> Vec<FinderLocations> {
        vec![TopLeft]
    }
    fn alignment_locations(&self) -> Vec<(u32, u32)> {
        Vec::new()
    }
    fn format_locations(&self) -> Vec<FinderLocations> {
        vec![TopLeft]
    }
    fn include_version_locations(&self) -> bool {
        false
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum QRSymbolTypes {
    QRCode,
    MicroQRCode,
}
pub struct QRFactory;
impl QRFactory {
    pub fn build_code(qr_type: QRSymbolTypes, version: u32) -> Box<dyn QRSymbol> {
        match qr_type {
            QRSymbolTypes::QRCode => Box::new(QRCode { version }),
            QRSymbolTypes::MicroQRCode => Box::new(MicroQRCode { version }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qr_code_has_correct_width_v1() {
        assert_eq!(QRCode { version: 1 }.module_width(), 21);
    }
    #[test]
    fn qr_code_has_correct_width_v7() {
        assert_eq!(QRCode { version: 7 }.module_width(), 45);
    }
    #[test]
    fn qr_code_has_correct_width_v40() {
        assert_eq!(QRCode { version: 40 }.module_width(), 177);
    }

    #[test]
    fn micro_qr_code_has_correct_width_v1() {
        assert_eq!(MicroQRCode { version: 1 }.module_width(), 11);
    }
    #[test]
    fn micro_qr_code_has_correct_width_v2() {
        assert_eq!(MicroQRCode { version: 2 }.module_width(), 13);
    }
    #[test]
    fn micro_qr_code_has_correct_width_v4() {
        assert_eq!(MicroQRCode { version: 4 }.module_width(), 17);
    }

    #[test]
    fn qr_code_timing_in_correct_place() {
        assert_eq!(QRCode { version: 5 }.timing_coord(), 6);
    }
    #[test]
    fn micro_qr_code_timing_in_correct_place() {
        assert_eq!(MicroQRCode { version: 3 }.timing_coord(), 0);
    }

    #[test]
    fn qr_code_three_finders() {
        assert_eq!(
            QRCode { version: 15 }.finder_locations(),
            vec![TopLeft, TopRight, BottomLeft]
        );
    }
    #[test]
    fn micro_qr_code_one_finder() {
        assert_eq!(MicroQRCode { version: 1 }.finder_locations(), vec![TopLeft]);
    }

    #[test]
    fn qr_code_v1_has_no_alignment() {
        assert!(QRCode { version: 1 }.alignment_locations().is_empty());
    }
    #[test]
    fn qr_code_v2_has_one_alignment() {
        assert_eq!(
            QRCode { version: 2 }.alignment_locations().iter().count(),
            1
        );
    }
    #[test]
    fn qr_code_v9_has_six_alignments() {
        assert_eq!(
            QRCode { version: 9 }.alignment_locations().iter().count(),
            6
        );
    }
    #[test]
    fn qr_code_v27_has_22_alignments() {
        assert_eq!(
            QRCode { version: 27 }.alignment_locations().iter().count(),
            22
        );
    }
    #[test]
    fn qr_code_v40_has_46_alignments() {
        assert_eq!(
            QRCode { version: 40 }.alignment_locations().iter().count(),
            46
        );
    }
    #[test]
    fn micro_qr_code_v4_has_no_alignments() {
        assert!(MicroQRCode { version: 4 }.alignment_locations().is_empty());
    }
}

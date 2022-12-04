#[derive(Debug, PartialEq)]
pub enum FinderLocations {
    TopLeft,
    TopRight,
    BottomLeft
}
use FinderLocations::*;

pub trait QRSymbol {
    fn module_width(&self) -> u32;
    fn timing_coord(&self) -> u32;
    fn finder_locations(&self) -> Vec<FinderLocations>;
}

pub struct QRCode {
    version: u32,
}
impl QRSymbol for QRCode {
    fn module_width(&self) -> u32 {
        21 + 4 * (self.version - 1)
    }
    fn timing_coord(&self) -> u32 { 6 }
    fn finder_locations(&self) -> Vec<FinderLocations> {
        vec![TopLeft, TopRight, BottomLeft]
    }
}

pub struct MicroQRCode {
    version: u32,
}
impl QRSymbol for MicroQRCode {
    fn module_width(&self) -> u32 {
        11 + 2 * (self.version - 1)
    }
    fn timing_coord(&self) -> u32 { 0 }
    fn finder_locations(&self) -> Vec<FinderLocations> {
        vec![TopLeft]
    }
}

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
        assert_eq!(QRCode {version: 1}.module_width(), 21);
    }
    #[test]
    fn qr_code_has_correct_width_v7() {
        assert_eq!(QRCode {version: 7}.module_width(), 45);
    }
    #[test]
    fn qr_code_has_correct_width_v40() {
        assert_eq!(QRCode {version: 40}.module_width(), 177);
    }

    #[test]
    fn micro_qr_code_has_correct_width_v1() {
        assert_eq!(MicroQRCode {version: 1}.module_width(), 11);
    }
    #[test]
    fn micro_qr_code_has_correct_width_v2() {
        assert_eq!(MicroQRCode {version: 2}.module_width(), 13);
    }
    #[test]
    fn micro_qr_code_has_correct_width_v4() {
        assert_eq!(MicroQRCode {version: 4}.module_width(), 17);
    }

    #[test]
    fn qr_code_timing_in_correct_place() {
        assert_eq!(QRCode {version: 5}.timing_coord(), 6);
    }
    #[test]
    fn micro_qr_code_timing_in_correct_place() {
        assert_eq!(MicroQRCode {version: 3}.timing_coord(), 0);
    }

    #[test]
    fn qr_code_three_finders() {
        assert_eq!(QRCode {version: 15}.finder_locations(), vec![TopLeft, TopRight, BottomLeft]);
    }
    #[test]
    fn micro_qr_code_one_finder() {
        assert_eq!(MicroQRCode {version: 1}.finder_locations(), vec![TopLeft]);
    }
}

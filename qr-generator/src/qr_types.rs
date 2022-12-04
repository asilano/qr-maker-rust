pub trait QRSymbol {
    fn module_width(&self) -> u32;
}

pub struct QRCode {
    version: u32,
}
impl QRSymbol for QRCode {
    fn module_width(&self) -> u32 {
        21 + 4 * (self.version - 1)
    }
}

pub struct MicroQRCode {
    version: u32,
}
impl QRSymbol for MicroQRCode {
    fn module_width(&self) -> u32 {
        11 + 2 * (self.version - 1)
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
}

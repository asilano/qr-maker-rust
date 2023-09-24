use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum QRError {
    EncodingError(EncodingError),
    ImageError(image::ImageError),
}

impl fmt::Display for QRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EncodingError(msg) => write!(f, "{}", msg),
            Self::ImageError(img_err) => write!(f, "{}", img_err),
        }
    }
}

impl Error for QRError {}

impl From<image::ImageError> for QRError {
    fn from(err: image::ImageError) -> Self {
        QRError::ImageError(err)
    }
}

#[derive(Debug)]
pub struct EncodingError {
    msg: String,
}
impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl EncodingError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
impl From<EncodingError> for QRError {
    fn from(err: EncodingError) -> Self {
        QRError::EncodingError(err)
    }
}

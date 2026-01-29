use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeshCodeError {
    InvalidFormat(String),
    InvalidDigit { position: usize, digit: char },
    InvalidLevel(usize),
    OutOfRange,
}

impl fmt::Display for MeshCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshCodeError::InvalidFormat(msg) => write!(f, "Invalid mesh code format: {}", msg),
            MeshCodeError::InvalidDigit { position, digit } => {
                write!(f, "Invalid digit '{}' at position {}", digit, position)
            }
            MeshCodeError::InvalidLevel(len) => {
                write!(f, "Invalid mesh code level: length {}", len)
            }
            MeshCodeError::OutOfRange => write!(f, "Mesh code out of valid range"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MeshCodeError {}

#[derive(Debug, Clone, PartialEq)]
pub enum CoordinateError {
    InvalidLatitude(f64),
    InvalidLongitude(f64),
    OutOfJapanRange,
}

impl fmt::Display for CoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoordinateError::InvalidLatitude(lat) => {
                write!(f, "Invalid latitude: {} (must be between -90 and 90)", lat)
            }
            CoordinateError::InvalidLongitude(lon) => {
                write!(
                    f,
                    "Invalid longitude: {} (must be between -180 and 180)",
                    lon
                )
            }
            CoordinateError::OutOfJapanRange => {
                write!(f, "Coordinate is outside of Japan's mesh code range")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CoordinateError {}

pub type Result<T> = core::result::Result<T, MeshCodeError>;
pub type CoordResult<T> = core::result::Result<T, CoordinateError>;

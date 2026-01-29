use crate::error::{MeshCodeError, Result};

/// メッシュのレベル（次数）を表す列挙型
///
/// JIS X 0410で定義されている各メッシュレベルに対応します。
/// レベルが大きいほど、より細かい地域を表します。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MeshLevel {
    /// 1次メッシュ（約80km四方、4桁）
    First = 1,
    /// 2次メッシュ（約10km四方、6桁）
    Second = 2,
    /// 3次メッシュ（約1km四方、8桁）
    Third = 3,
    /// 4次メッシュ（2分の1地域メッシュ、約500m四方、9桁）
    FourthHalf = 4,
    /// 4次メッシュ（4分の1地域メッシュ、約250m四方、10桁）
    FourthQuarter = 5,
    /// 4次メッシュ（8分の1地域メッシュ、約125m四方、11桁）
    FourthEighth = 6,
    /// 5次メッシュ（約100m四方、10桁）
    Fifth = 7,
}

impl MeshLevel {
    /// メッシュコードの桁数からメッシュレベルを判定する
    ///
    /// # 引数
    /// * `len` - メッシュコードの桁数
    ///
    /// # 戻り値
    /// 対応するメッシュレベル、または無効な桁数の場合はエラー
    pub fn from_code_length(len: usize) -> Result<Self> {
        match len {
            4 => Ok(MeshLevel::First),
            6 => Ok(MeshLevel::Second),
            8 => Ok(MeshLevel::Third),
            9 => Ok(MeshLevel::FourthHalf),
            10 => Ok(MeshLevel::FourthQuarter),
            11 => Ok(MeshLevel::FourthEighth),
            _ => Err(MeshCodeError::InvalidLevel(len)),
        }
    }

    /// メッシュコード文字列からメッシュレベルを判定する
    ///
    /// 10桁のメッシュコードの場合、4次メッシュ（4分の1）と5次メッシュを区別します。
    ///
    /// # 引数
    /// * `code_str` - メッシュコード文字列
    ///
    /// # 戻り値
    /// 対応するメッシュレベル、または無効な形式の場合はエラー
    pub fn from_code_string(code_str: &str) -> Result<Self> {
        let len = code_str.len();
        if len == 10 {
            let ninth_digit = code_str.chars().nth(8).unwrap();
            if ('1'..='4').contains(&ninth_digit) {
                return Ok(MeshLevel::FourthQuarter);
            } else {
                return Ok(MeshLevel::Fifth);
            }
        }
        Self::from_code_length(len)
    }

    /// このメッシュレベルのコード桁数を返す
    pub fn code_length(self) -> usize {
        match self {
            MeshLevel::First => 4,
            MeshLevel::Second => 6,
            MeshLevel::Third => 8,
            MeshLevel::FourthHalf => 9,
            MeshLevel::FourthQuarter => 10,
            MeshLevel::FourthEighth => 11,
            MeshLevel::Fifth => 10,
        }
    }

    /// このメッシュレベルの緯度方向のサイズを度数で返す
    pub fn lat_size_degrees(self) -> f64 {
        match self {
            MeshLevel::First => 40.0 / 60.0,
            MeshLevel::Second => 5.0 / 60.0,
            MeshLevel::Third => 30.0 / 3600.0,
            MeshLevel::FourthHalf => 15.0 / 3600.0,
            MeshLevel::FourthQuarter => 7.5 / 3600.0,
            MeshLevel::FourthEighth => 3.75 / 3600.0,
            MeshLevel::Fifth => 3.0 / 3600.0,
        }
    }

    /// このメッシュレベルの経度方向のサイズを度数で返す
    pub fn lon_size_degrees(self) -> f64 {
        match self {
            MeshLevel::First => 1.0,
            MeshLevel::Second => 7.5 / 60.0,
            MeshLevel::Third => 45.0 / 3600.0,
            MeshLevel::FourthHalf => 22.5 / 3600.0,
            MeshLevel::FourthQuarter => 11.25 / 3600.0,
            MeshLevel::FourthEighth => 5.625 / 3600.0,
            MeshLevel::Fifth => 4.5 / 3600.0,
        }
    }

    /// このメッシュレベルのおおよそのサイズをメートルで返す
    pub fn approximate_size_meters(self) -> f64 {
        match self {
            MeshLevel::First => 80000.0,
            MeshLevel::Second => 10000.0,
            MeshLevel::Third => 1000.0,
            MeshLevel::FourthHalf => 500.0,
            MeshLevel::FourthQuarter => 250.0,
            MeshLevel::FourthEighth => 125.0,
            MeshLevel::Fifth => 100.0,
        }
    }

    /// このメッシュレベルの親レベルを返す（1次メッシュの場合はNone）
    pub fn parent(self) -> Option<Self> {
        match self {
            MeshLevel::First => None,
            MeshLevel::Second => Some(MeshLevel::First),
            MeshLevel::Third => Some(MeshLevel::Second),
            MeshLevel::FourthHalf => Some(MeshLevel::Third),
            MeshLevel::FourthQuarter => Some(MeshLevel::Third),
            MeshLevel::FourthEighth => Some(MeshLevel::Third),
            MeshLevel::Fifth => Some(MeshLevel::Third),
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            1 => Ok(MeshLevel::First),
            2 => Ok(MeshLevel::Second),
            3 => Ok(MeshLevel::Third),
            4 => Ok(MeshLevel::FourthHalf),
            5 => Ok(MeshLevel::FourthQuarter),
            6 => Ok(MeshLevel::FourthEighth),
            7 => Ok(MeshLevel::Fifth),
            _ => Err(MeshCodeError::InvalidLevel(value as usize)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_code_length() {
        assert_eq!(MeshLevel::from_code_length(4).unwrap(), MeshLevel::First);
        assert_eq!(MeshLevel::from_code_length(6).unwrap(), MeshLevel::Second);
        assert_eq!(MeshLevel::from_code_length(8).unwrap(), MeshLevel::Third);
        assert!(MeshLevel::from_code_length(3).is_err());
    }

    #[test]
    fn test_lat_lon_size() {
        assert!((MeshLevel::First.lat_size_degrees() - 40.0 / 60.0).abs() < 1e-10);
        assert!((MeshLevel::First.lon_size_degrees() - 1.0).abs() < 1e-10);
        assert!((MeshLevel::Third.lat_size_degrees() - 30.0 / 3600.0).abs() < 1e-10);
    }

    #[test]
    fn test_parent() {
        assert_eq!(MeshLevel::Third.parent(), Some(MeshLevel::Second));
        assert_eq!(MeshLevel::Second.parent(), Some(MeshLevel::First));
        assert_eq!(MeshLevel::First.parent(), None);
    }
}

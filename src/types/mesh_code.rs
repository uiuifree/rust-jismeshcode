use crate::error::{MeshCodeError, Result};
use crate::types::mesh_level::MeshLevel;
use alloc::format;
use alloc::string::{String, ToString};
use core::fmt;

/// メッシュコードを表す型
///
/// 内部表現としてu64を使用し、上位8ビットにメッシュレベル、
/// 下位56ビットにメッシュコード値を格納します。
/// Copy traitを実装しているため、効率的な値渡しが可能です。
///
/// # シリアライズ（`serde`フィーチャー）
///
/// `serde`フィーチャー有効時は、メッシュコード文字列（例: `"53394611"`）として
/// シリアライズ・デシリアライズされます。デシリアライズは[`MeshCode::from_str`]と
/// 同じ規則でレベルを判定するため、10桁コードの曖昧さ（4分の1メッシュと
/// 5次メッシュ）に注意してください。
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// // 文字列からメッシュコードを作成
/// let mesh = MeshCode::from_str("5339").unwrap();
/// assert_eq!(mesh.level(), MeshLevel::First);
/// assert_eq!(mesh.as_string(), "5339");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String", try_from = "String"))]
pub struct MeshCode {
    value: u64,
}

impl MeshCode {
    /// メッシュレベルとコード値から新しいメッシュコードを作成する
    ///
    /// コード値がレベルの桁数・番号規則に反する場合は
    /// [`MeshCodeError::OutOfRange`]を返します。
    ///
    /// # 引数
    /// * `level` - メッシュレベル
    /// * `code` - メッシュコード値
    pub fn new(level: MeshLevel, code: u64) -> Result<Self> {
        Self::validate(level, code)?;
        let level_bits = (level.as_u8() as u64) << 56;
        let value = level_bits | (code & 0x00FF_FFFF_FFFF_FFFF);
        Ok(MeshCode { value })
    }

    /// コード値がメッシュレベルの規則に適合するか検証する
    fn validate(level: MeshLevel, code: u64) -> Result<()> {
        let len = level.code_length() as u32;
        if code >= 10u64.pow(len) {
            return Err(MeshCodeError::OutOfRange);
        }

        // 左からi番目（0始まり）の桁を取り出す
        let digit = |i: u32| (code / 10u64.pow(len - 1 - i)) % 10;

        // 2次メッシュの緯度・経度番号（5〜6桁目）は0〜7
        if len >= 6 && (digit(4) > 7 || digit(5) > 7) {
            return Err(MeshCodeError::OutOfRange);
        }

        // 分割地域メッシュの番号（9桁目以降）は1〜4
        let subdivision_digits: &[u32] = match level {
            MeshLevel::FourthHalf => &[8],
            MeshLevel::FourthQuarter => &[8, 9],
            MeshLevel::FourthEighth => &[8, 9, 10],
            _ => &[],
        };
        for &i in subdivision_digits {
            if !(1..=4).contains(&digit(i)) {
                return Err(MeshCodeError::OutOfRange);
            }
        }

        Ok(())
    }

    /// 文字列からメッシュコードをパースする
    ///
    /// # 引数
    /// * `s` - メッシュコード文字列（例: "5339", "533946", "53394611"）
    ///
    /// # 戻り値
    /// パースされたメッシュコード、または無効な形式の場合はエラー
    ///
    /// # 例
    ///
    /// ```
    /// use jismeshcode::prelude::*;
    ///
    /// let mesh = MeshCode::from_str("53394611").unwrap();
    /// assert_eq!(mesh.level(), MeshLevel::Third);
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Err(MeshCodeError::InvalidFormat("Empty string".to_string()));
        }

        for (i, c) in s.chars().enumerate() {
            if !c.is_ascii_digit() {
                return Err(MeshCodeError::InvalidDigit {
                    position: i,
                    digit: c,
                });
            }
        }

        let level = MeshLevel::from_code_string(s)?;

        let code = s.parse::<u64>().map_err(|_| {
            MeshCodeError::InvalidFormat("Failed to parse numeric code".to_string())
        })?;

        Self::new(level, code)
    }

    /// このメッシュコードのレベルを返す
    pub fn level(&self) -> MeshLevel {
        let level_byte = (self.value >> 56) as u8;
        MeshLevel::from_u8(level_byte).expect("Invalid level stored in MeshCode")
    }

    /// このメッシュコードの数値表現を返す
    pub fn code(&self) -> u64 {
        self.value & 0x00FF_FFFF_FFFF_FFFF
    }

    /// このメッシュコードを文字列表現に変換する
    ///
    /// 先頭のゼロを含む適切な桁数の文字列を返します。
    pub fn as_string(&self) -> String {
        let level = self.level();
        let code = self.code();
        let width = level.code_length();
        format!("{code:0width$}")
    }
}

impl core::str::FromStr for MeshCode {
    type Err = MeshCodeError;

    fn from_str(s: &str) -> Result<Self> {
        MeshCode::from_str(s)
    }
}

impl From<MeshCode> for String {
    fn from(mesh: MeshCode) -> Self {
        mesh.as_string()
    }
}

impl TryFrom<String> for MeshCode {
    type Error = MeshCodeError;

    fn try_from(s: String) -> Result<Self> {
        MeshCode::from_str(&s)
    }
}

impl fmt::Debug for MeshCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeshCode")
            .field("level", &self.level())
            .field("code", &self.as_string())
            .finish()
    }
}

impl fmt::Display for MeshCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_code_creation() {
        let mesh = MeshCode::new(MeshLevel::First, 5339).unwrap();
        assert_eq!(mesh.level(), MeshLevel::First);
        assert_eq!(mesh.code(), 5339);
        assert_eq!(mesh.as_string(), "5339");
    }

    #[test]
    fn test_mesh_code_from_str() {
        let mesh = MeshCode::from_str("5339").unwrap();
        assert_eq!(mesh.level(), MeshLevel::First);
        assert_eq!(mesh.code(), 5339);

        let mesh = MeshCode::from_str("53393599").unwrap();
        assert_eq!(mesh.level(), MeshLevel::Third);
        assert_eq!(mesh.code(), 53393599);
    }

    #[test]
    fn test_invalid_mesh_code() {
        assert!(MeshCode::from_str("").is_err());
        assert!(MeshCode::from_str("abc").is_err());
        assert!(MeshCode::from_str("12345").is_err());
    }

    #[test]
    fn test_mesh_code_display() {
        let mesh = MeshCode::from_str("0001").unwrap();
        assert_eq!(mesh.as_string(), "0001");
        assert_eq!(format!("{mesh}"), "0001");
    }
}

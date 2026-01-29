use crate::error::{CoordinateError, CoordResult};

/// 地理座標（緯度経度）を表す型
///
/// 日本の範囲内（緯度20-46度、経度122-154度）の座標のみを受け付けます。
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// // 東京駅の座標
/// let coord = Coordinate::new(35.6812, 139.7671).unwrap();
/// assert_eq!(coord.lat(), 35.6812);
/// assert_eq!(coord.lon(), 139.7671);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    lat: f64,
    lon: f64,
}

impl Coordinate {
    /// 新しい座標を作成する
    ///
    /// # 引数
    /// * `lat` - 緯度（-90.0〜90.0）
    /// * `lon` - 経度（-180.0〜180.0）
    ///
    /// # 戻り値
    /// 座標オブジェクト、または範囲外の場合はエラー
    ///
    /// # 例
    ///
    /// ```
    /// use jismeshcode::prelude::*;
    ///
    /// let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    /// ```
    pub fn new(lat: f64, lon: f64) -> CoordResult<Self> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err(CoordinateError::InvalidLatitude(lat));
        }
        if !(-180.0..=180.0).contains(&lon) {
            return Err(CoordinateError::InvalidLongitude(lon));
        }

        if !Self::is_in_japan_range(lat, lon) {
            return Err(CoordinateError::OutOfJapanRange);
        }

        Ok(Coordinate { lat, lon })
    }

    /// 範囲チェックなしで新しい座標を作成する
    ///
    /// 内部使用のため、範囲バリデーションをスキップします。
    pub fn new_unchecked(lat: f64, lon: f64) -> Self {
        Coordinate { lat, lon }
    }

    /// 緯度を返す
    pub fn lat(&self) -> f64 {
        self.lat
    }

    /// 経度を返す
    pub fn lon(&self) -> f64 {
        self.lon
    }

    fn is_in_japan_range(lat: f64, lon: f64) -> bool {
        (20.0..=46.0).contains(&lat) && (122.0..=154.0).contains(&lon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_coordinate() {
        let coord = Coordinate::new(35.6812, 139.7671).unwrap();
        assert_eq!(coord.lat(), 35.6812);
        assert_eq!(coord.lon(), 139.7671);
    }

    #[test]
    fn test_invalid_latitude() {
        assert!(Coordinate::new(91.0, 139.0).is_err());
        assert!(Coordinate::new(-91.0, 139.0).is_err());
    }

    #[test]
    fn test_invalid_longitude() {
        assert!(Coordinate::new(35.0, 181.0).is_err());
        assert!(Coordinate::new(35.0, -181.0).is_err());
    }

    #[test]
    fn test_out_of_japan_range() {
        assert!(Coordinate::new(0.0, 0.0).is_err());
        assert!(Coordinate::new(50.0, 100.0).is_err());
    }
}

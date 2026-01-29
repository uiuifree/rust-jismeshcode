use crate::types::Coordinate;

/// 地球の半径（メートル）
const EARTH_RADIUS_METERS: f64 = 6371000.0;

/// 2点間の距離をHaversine公式で計算する
///
/// Haversine公式を使用して、地球上の2点間の大円距離を計算します。
/// 日本国内の比較的短い距離での使用に適しています。
///
/// # 引数
/// * `coord1` - 1つ目の座標
/// * `coord2` - 2つ目の座標
///
/// # 戻り値
/// 2点間の距離（メートル単位）
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
/// let yokohama = Coordinate::new(35.4437, 139.6380).unwrap();
/// let distance = haversine_distance(tokyo, yokohama);
/// println!("東京-横浜間の距離: {:.2}km", distance / 1000.0);
/// ```
pub fn haversine_distance(coord1: Coordinate, coord2: Coordinate) -> f64 {
    let lat1 = coord1.lat().to_radians();
    let lat2 = coord2.lat().to_radians();
    let lon1 = coord1.lon().to_radians();
    let lon2 = coord2.lon().to_radians();

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    // Haversine公式
    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_METERS * c
}

/// 指定距離に対応する緯度経度のオフセットを計算する
///
/// 半径検索のためのBoundingBox作成に使用します。
/// 経度のオフセットは緯度によって変化するため、緯度でコサイン補正を行います。
///
/// # 引数
/// * `center` - 中心座標
/// * `radius_meters` - 半径（メートル）
///
/// # 戻り値
/// (緯度オフセット, 経度オフセット) のタプル（度単位）
pub(crate) fn calculate_bbox_offsets(center: Coordinate, radius_meters: f64) -> (f64, f64) {
    // 緯度1度 ≈ 111,320メートル（ほぼ一定）
    let lat_offset = radius_meters / 111320.0;

    // 経度1度の距離は緯度により変わる（極に近いほど短くなる）
    // cos(緯度)で補正
    let lon_offset = radius_meters / (111320.0 * center.lat().to_radians().cos());

    (lat_offset, lon_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_distance_same_point() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
        let distance = haversine_distance(tokyo, tokyo);
        assert!(distance.abs() < 0.01, "同一座標の距離は0に近い値");
    }

    #[test]
    fn test_haversine_distance_tokyo_yokohama() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
        let yokohama = Coordinate::new(35.4437, 139.6380).unwrap();
        let distance = haversine_distance(tokyo, yokohama);

        // 東京-横浜間の距離は約28km
        assert!(distance > 27000.0 && distance < 29000.0, "距離は約28km");
    }

    #[test]
    fn test_haversine_distance_symmetric() {
        let coord1 = Coordinate::new(35.6812, 139.7671).unwrap();
        let coord2 = Coordinate::new(35.4437, 139.6380).unwrap();

        let dist1 = haversine_distance(coord1, coord2);
        let dist2 = haversine_distance(coord2, coord1);

        assert!((dist1 - dist2).abs() < 0.01, "距離計算は対称");
    }

    #[test]
    fn test_calculate_bbox_offsets_positive() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
        let (lat_offset, lon_offset) = calculate_bbox_offsets(tokyo, 1000.0);

        // 1000mの緯度オフセットは約0.009度
        assert!(lat_offset > 0.008 && lat_offset < 0.01);

        // 経度オフセットは緯度により異なるが、東京付近では緯度オフセットより大きい
        assert!(lon_offset > lat_offset);
    }

    #[test]
    fn test_calculate_bbox_offsets_zero_radius() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
        let (lat_offset, lon_offset) = calculate_bbox_offsets(tokyo, 0.0);

        assert_eq!(lat_offset, 0.0);
        assert_eq!(lon_offset, 0.0);
    }
}

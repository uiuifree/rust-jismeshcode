use crate::convert::mesh_to_center;
use crate::spatial::range::MeshCodeIterator;
use crate::types::{BoundingBox, Coordinate, MeshCode, MeshLevel};
use crate::utils::distance::{calculate_bbox_offsets, haversine_distance};

/// 半径検索でメッシュコードを遅延評価で列挙するイテレータ
///
/// 指定座標からの距離が指定半径以内のメッシュコードを列挙します。
/// 内部的にはBoundingBoxで範囲を絞り込み、Haversine公式で実距離を判定します。
pub struct MeshCodeRadiusIterator {
    bbox_iter: MeshCodeIterator,
    center: Coordinate,
    radius_meters: f64,
    // 半径0の場合の中心メッシュ（遅延初期化）
    center_mesh_for_zero_radius: Option<Option<MeshCode>>,
    level: MeshLevel,
}

impl MeshCodeRadiusIterator {
    /// 新しいRadiusIteratorを作成する
    ///
    /// # 引数
    /// * `center` - 中心座標
    /// * `radius_meters` - 検索半径（メートル）
    /// * `level` - 目的のメッシュレベル
    pub fn new(center: Coordinate, radius_meters: f64, level: MeshLevel) -> Self {
        // 負の半径の場合は空のイテレータを返す
        if radius_meters < 0.0 {
            let empty_bbox = BoundingBox::new(center, center);
            return MeshCodeRadiusIterator {
                bbox_iter: MeshCodeIterator::new(empty_bbox, level),
                center,
                radius_meters: 0.0,
                center_mesh_for_zero_radius: Some(None),
                level,
            };
        }

        // 半径からBoundingBoxを作成
        let (lat_offset, lon_offset) = if radius_meters == 0.0 {
            // 半径0の場合は空のBoundingBoxで良い（特殊処理で対応）
            (0.0, 0.0)
        } else {
            calculate_bbox_offsets(center, radius_meters)
        };

        let min_lat = (center.lat() - lat_offset).max(20.0);
        let max_lat = (center.lat() + lat_offset).min(46.0);
        let min_lon = (center.lon() - lon_offset).max(122.0);
        let max_lon = (center.lon() + lon_offset).min(154.0);

        let sw = Coordinate::new_unchecked(min_lat, min_lon);
        let ne = Coordinate::new_unchecked(max_lat, max_lon);
        let bbox = BoundingBox::new(sw, ne);

        MeshCodeRadiusIterator {
            bbox_iter: MeshCodeIterator::new(bbox, level),
            center,
            radius_meters,
            center_mesh_for_zero_radius: None,
            level,
        }
    }
}

impl Iterator for MeshCodeRadiusIterator {
    type Item = MeshCode;

    fn next(&mut self) -> Option<Self::Item> {
        // 半径0の場合は、中心座標を含むメッシュのみを返す特殊処理
        if self.radius_meters == 0.0 {
            // 遅延初期化: 最初の呼び出し時に中心メッシュを計算
            if self.center_mesh_for_zero_radius.is_none() {
                let center_mesh = crate::convert::coord_to_mesh(self.center, self.level).ok();
                self.center_mesh_for_zero_radius = Some(center_mesh);
                return center_mesh;
            }
            // 2回目以降の呼び出しではNoneを返す
            return None;
        }

        // 通常の半径検索
        loop {
            let mesh = self.bbox_iter.next()?;
            let mesh_center = mesh_to_center(mesh);
            let distance = haversine_distance(self.center, mesh_center);

            if distance <= self.radius_meters {
                return Some(mesh);
            }
            // 距離範囲外のメッシュはスキップして次へ
        }
    }
}

/// 指定座標から指定距離内のメッシュコードをイテレータで取得する
///
/// 指定座標を中心として、指定半径以内のメッシュコードを列挙します。
/// 各メッシュの中心座標との距離で判定します。
///
/// # 引数
/// * `center` - 中心座標
/// * `radius_meters` - 検索半径（メートル）
/// * `level` - 目的のメッシュレベル
///
/// # 戻り値
/// メッシュコードを列挙するイテレータ
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
/// let meshes: Vec<_> = mesh_codes_in_radius(tokyo, 1000.0, MeshLevel::Third).collect();
/// println!("1000m以内のメッシュ数: {}", meshes.len());
/// ```
pub fn mesh_codes_in_radius(
    center: Coordinate,
    radius_meters: f64,
    level: MeshLevel,
) -> MeshCodeRadiusIterator {
    MeshCodeRadiusIterator::new(center, radius_meters, level)
}

/// メッシュコードから指定距離内のメッシュコードをイテレータで取得する
///
/// 指定メッシュの中心座標を基準として、指定半径以内のメッシュコードを列挙します。
/// メッシュコードベースで半径検索を行う最も一般的なAPIです。
///
/// # 引数
/// * `mesh` - 中心メッシュコード
/// * `radius_meters` - 検索半径（メートル）
///
/// # 戻り値
/// メッシュコードを列挙するイテレータ（中心メッシュと同じレベル）
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let mesh = MeshCode::from_str("53394611").unwrap();
/// let nearby: Vec<_> = mesh_codes_in_radius_from_mesh(mesh, 1000.0).collect();
/// println!("1000m以内のメッシュ数: {}", nearby.len());
///
/// // 中心メッシュ自身も含まれる
/// assert!(nearby.contains(&mesh));
/// ```
pub fn mesh_codes_in_radius_from_mesh(
    mesh: MeshCode,
    radius_meters: f64,
) -> MeshCodeRadiusIterator {
    let center = mesh_to_center(mesh);
    let level = mesh.level();
    MeshCodeRadiusIterator::new(center, radius_meters, level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::coord_to_mesh;

    #[test]
    fn test_mesh_codes_in_radius_zero() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
        let meshes: Vec<_> = mesh_codes_in_radius(tokyo, 0.0, MeshLevel::Third).collect();

        // 半径0mでは中心座標を含むメッシュのみ
        assert_eq!(meshes.len(), 1);
        let expected = coord_to_mesh(tokyo, MeshLevel::Third).unwrap();
        assert_eq!(meshes[0], expected);
    }

    #[test]
    fn test_mesh_codes_in_radius_positive() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
        let meshes: Vec<_> = mesh_codes_in_radius(tokyo, 1000.0, MeshLevel::Third).collect();

        // 1000m以内には複数のメッシュがある
        assert!(meshes.len() > 1);

        // すべてのメッシュが実際に1000m以内か検証
        for mesh in &meshes {
            let mesh_center = mesh_to_center(*mesh);
            let distance = haversine_distance(tokyo, mesh_center);
            assert!(
                distance <= 1000.0,
                "メッシュ {} の距離 {:.2}m は1000m以内",
                mesh,
                distance
            );
        }
    }

    #[test]
    fn test_mesh_codes_in_radius_negative() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
        let meshes: Vec<_> = mesh_codes_in_radius(tokyo, -100.0, MeshLevel::Third).collect();

        // 負の半径では空のイテレータ
        assert_eq!(meshes.len(), 0);
    }

    #[test]
    fn test_mesh_codes_in_radius_from_mesh() {
        let mesh = coord_to_mesh(Coordinate::new(35.6812, 139.7671).unwrap(), MeshLevel::Third)
            .unwrap();
        let nearby: Vec<_> = mesh_codes_in_radius_from_mesh(mesh, 1000.0).collect();

        // 中心メッシュ自身を含む
        assert!(nearby.contains(&mesh));

        // 複数のメッシュがある
        assert!(nearby.len() > 1);

        // すべて同じレベル
        assert!(nearby.iter().all(|m| m.level() == mesh.level()));
    }

    #[test]
    fn test_mesh_codes_in_radius_increasing_radius() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();

        let meshes_500: Vec<_> = mesh_codes_in_radius(tokyo, 500.0, MeshLevel::Third).collect();
        let meshes_1000: Vec<_> = mesh_codes_in_radius(tokyo, 1000.0, MeshLevel::Third).collect();
        let meshes_2000: Vec<_> = mesh_codes_in_radius(tokyo, 2000.0, MeshLevel::Third).collect();

        // 半径が大きいほどメッシュ数が増える
        assert!(meshes_500.len() < meshes_1000.len());
        assert!(meshes_1000.len() < meshes_2000.len());
    }

    #[test]
    fn test_mesh_codes_in_radius_different_levels() {
        let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();

        let first_level: Vec<_> = mesh_codes_in_radius(tokyo, 10000.0, MeshLevel::First).collect();
        let second_level: Vec<_> =
            mesh_codes_in_radius(tokyo, 10000.0, MeshLevel::Second).collect();
        let third_level: Vec<_> = mesh_codes_in_radius(tokyo, 10000.0, MeshLevel::Third).collect();

        // 細かいメッシュほど数が多い
        assert!(first_level.len() < second_level.len());
        assert!(second_level.len() < third_level.len());

        // レベルが正しい
        assert!(first_level.iter().all(|m| m.level() == MeshLevel::First));
        assert!(second_level
            .iter()
            .all(|m| m.level() == MeshLevel::Second));
        assert!(third_level.iter().all(|m| m.level() == MeshLevel::Third));
    }
}

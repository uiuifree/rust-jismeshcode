use crate::convert::coord_to_mesh;
use crate::types::{BoundingBox, MeshCode, MeshLevel};

/// 範囲内のメッシュコードを遅延評価で列挙するイテレータ
///
/// 大量のメッシュコードを扱う場合でも、メモリ効率的に処理できます。
pub struct MeshCodeIterator {
    bbox: BoundingBox,
    level: MeshLevel,
    current_lat: f64,
    current_lon: f64,
    lat_step: f64,
    lon_step: f64,
}

impl MeshCodeIterator {
    pub fn new(bbox: BoundingBox, level: MeshLevel) -> Self {
        let lat_step = level.lat_size_degrees();
        let lon_step = level.lon_size_degrees();

        MeshCodeIterator {
            bbox,
            level,
            current_lat: bbox.min_lat(),
            current_lon: bbox.min_lon(),
            lat_step,
            lon_step,
        }
    }
}

impl Iterator for MeshCodeIterator {
    type Item = MeshCode;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_lat <= self.bbox.max_lat() {
            while self.current_lon <= self.bbox.max_lon() {
                let coord =
                    crate::types::Coordinate::new_unchecked(self.current_lat, self.current_lon);

                self.current_lon += self.lon_step;

                if let Ok(mesh) = coord_to_mesh(coord, self.level) {
                    return Some(mesh);
                }
            }

            self.current_lat += self.lat_step;
            self.current_lon = self.bbox.min_lon();
        }

        None
    }
}

/// 指定された境界ボックス内のメッシュコードをイテレータで取得する
///
/// # 引数
/// * `bbox` - 検索範囲を表す境界ボックス
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
/// let sw = Coordinate::new(35.6, 139.7).unwrap();
/// let ne = Coordinate::new(35.7, 139.8).unwrap();
/// let bbox = BoundingBox::new(sw, ne);
///
/// let meshes: Vec<_> = mesh_codes_in_bbox(bbox, MeshLevel::Third).collect();
/// println!("メッシュ数: {}", meshes.len());
/// ```
pub fn mesh_codes_in_bbox(bbox: BoundingBox, level: MeshLevel) -> MeshCodeIterator {
    MeshCodeIterator::new(bbox, level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Coordinate;

    #[test]
    fn test_mesh_codes_in_bbox() {
        let sw = Coordinate::new(35.6, 139.7).unwrap();
        let ne = Coordinate::new(35.7, 139.8).unwrap();
        let bbox = BoundingBox::new(sw, ne);

        let meshes: Vec<_> = mesh_codes_in_bbox(bbox, MeshLevel::Third).collect();
        assert!(meshes.len() > 0);
        assert!(meshes.iter().all(|m| m.level() == MeshLevel::Third));
    }
}

use crate::convert::{coord_to_mesh, mesh_to_center};
use crate::types::{Direction, MeshCode};

/// 指定された方向の隣接メッシュを取得する
///
/// # 引数
/// * `mesh` - 対象のメッシュコード
/// * `direction` - 方向（北、北東、東、南東、南、南西、西、北西）
///
/// # 戻り値
/// 隣接メッシュコード、または範囲外の場合はNone
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let mesh = MeshCode::from_str("53394611").unwrap();
/// let north = neighbor(mesh, Direction::North);
/// assert!(north.is_some());
/// ```
pub fn neighbor(mesh: MeshCode, direction: Direction) -> Option<MeshCode> {
    let center = mesh_to_center(mesh);
    let level = mesh.level();

    let lat_size = level.lat_size_degrees();
    let lon_size = level.lon_size_degrees();

    let (dx, dy) = direction.offset();

    let new_lat = center.lat() + dy as f64 * lat_size;
    let new_lon = center.lon() + dx as f64 * lon_size;

    if !(20.0..=46.0).contains(&new_lat) || !(122.0..=154.0).contains(&new_lon) {
        return None;
    }

    let new_coord = crate::types::Coordinate::new_unchecked(new_lat, new_lon);
    coord_to_mesh(new_coord, level).ok()
}

/// すべての方向の隣接メッシュを取得する
///
/// 8方向（北、北東、東、南東、南、南西、西、北西）の隣接メッシュを返します。
/// 範囲外の隣接メッシュは結果に含まれません。
///
/// # 引数
/// * `mesh` - 対象のメッシュコード
///
/// # 戻り値
/// 隣接メッシュコードのベクター（最大8個）
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let mesh = MeshCode::from_str("53394611").unwrap();
/// let all_neighbors = neighbors(mesh);
/// println!("隣接メッシュ数: {}", all_neighbors.len());
/// ```
pub fn neighbors(mesh: MeshCode) -> Vec<MeshCode> {
    Direction::ALL
        .iter()
        .filter_map(|&dir| neighbor(mesh, dir))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbor_north() {
        let mesh = MeshCode::from_str("53393599").unwrap();
        let north = neighbor(mesh, Direction::North);
        assert!(north.is_some());
    }

    #[test]
    fn test_neighbors() {
        let mesh = MeshCode::from_str("53393599").unwrap();
        let all_neighbors = neighbors(mesh);
        assert!(all_neighbors.len() <= 8);
        assert!(all_neighbors.len() > 0);
    }
}

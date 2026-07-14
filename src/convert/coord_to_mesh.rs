use crate::error::Result;
use crate::types::{Coordinate, MeshCode, MeshLevel};
use crate::utils::math;
use alloc::format;

/// 地理座標からメッシュコードに変換する
///
/// JIS X 0410に基づいて、指定された座標を含むメッシュコードを計算します。
///
/// # 引数
/// * `coord` - 変換する座標
/// * `level` - 目的のメッシュレベル
///
/// # 戻り値
/// 計算されたメッシュコード
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let coord = Coordinate::new(35.6812, 139.7671).unwrap();
/// let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();
/// println!("メッシュコード: {}", mesh);
/// ```
pub fn coord_to_mesh(coord: Coordinate, level: MeshLevel) -> Result<MeshCode> {
    let lat = coord.lat();
    let lon = coord.lon();

    let first_code = calc_first_mesh(lat, lon);

    match level {
        MeshLevel::First => MeshCode::new(level, first_code),
        MeshLevel::Second => {
            let second_code = calc_second_mesh(lat, lon, first_code);
            MeshCode::new(level, second_code)
        }
        MeshLevel::Third => {
            let second_code = calc_second_mesh(lat, lon, first_code);
            let third_code = calc_third_mesh(lat, lon, second_code);
            MeshCode::new(level, third_code)
        }
        MeshLevel::FourthHalf => {
            let second_code = calc_second_mesh(lat, lon, first_code);
            let third_code = calc_third_mesh(lat, lon, second_code);
            let fourth_code = calc_fourth_half_mesh(lat, lon, third_code);
            MeshCode::new(level, fourth_code)
        }
        MeshLevel::FourthQuarter => {
            let second_code = calc_second_mesh(lat, lon, first_code);
            let third_code = calc_third_mesh(lat, lon, second_code);
            let fourth_code = calc_fourth_quarter_mesh(lat, lon, third_code);
            MeshCode::new(level, fourth_code)
        }
        MeshLevel::FourthEighth => {
            let second_code = calc_second_mesh(lat, lon, first_code);
            let third_code = calc_third_mesh(lat, lon, second_code);
            let fourth_code = calc_fourth_eighth_mesh(lat, lon, third_code);
            MeshCode::new(level, fourth_code)
        }
        MeshLevel::Fifth => {
            let second_code = calc_second_mesh(lat, lon, first_code);
            let third_code = calc_third_mesh(lat, lon, second_code);
            let fifth_code = calc_fifth_mesh(lat, lon, third_code);
            MeshCode::new(level, fifth_code)
        }
    }
}

fn calc_first_mesh(lat: f64, lon: f64) -> u64 {
    let lat_times_1_5 = math::floor(lat * 1.5) as i32;
    let p = lat_times_1_5 / 10;
    let q = lat_times_1_5 % 10;

    let lon_minus_100 = math::floor(lon - 100.0) as i32;
    let r = lon_minus_100 / 10;
    let s = lon_minus_100 % 10;

    (p * 1000 + q * 100 + r * 10 + s) as u64
}

fn calc_second_mesh(lat: f64, lon: f64, first_code: u64) -> u64 {
    let first_str = format!("{first_code:04}");
    let p = first_str[0..1].parse::<f64>().unwrap();
    let q = first_str[1..2].parse::<f64>().unwrap();
    let r = first_str[2..3].parse::<f64>().unwrap();
    let s = first_str[3..4].parse::<f64>().unwrap();

    let first_lat = (p * 10.0 + q) / 1.5;
    let first_lon = r * 10.0 + s + 100.0;

    let lat_in_mesh = lat - first_lat;
    let lon_in_mesh = lon - first_lon;

    // 浮動小数点誤差でメッシュ境界を越えないよう0〜7に制限する
    let t = (math::floor(lat_in_mesh / (40.0 / 60.0) * 8.0) as i32).clamp(0, 7);
    let u = (math::floor(lon_in_mesh * 8.0) as i32).clamp(0, 7);

    first_code * 100 + (t * 10 + u) as u64
}

fn calc_third_mesh(lat: f64, lon: f64, second_code: u64) -> u64 {
    let second_str = format!("{second_code:06}");
    let first_str = &second_str[0..4];
    let p = first_str[0..1].parse::<f64>().unwrap();
    let q = first_str[1..2].parse::<f64>().unwrap();
    let r = first_str[2..3].parse::<f64>().unwrap();
    let s = first_str[3..4].parse::<f64>().unwrap();
    let t = second_str[4..5].parse::<f64>().unwrap();
    let u = second_str[5..6].parse::<f64>().unwrap();

    let first_lat = (p * 10.0 + q) / 1.5;
    let first_lon = r * 10.0 + s + 100.0;
    let second_lat = first_lat + t * (40.0 / 60.0) / 8.0;
    let second_lon = first_lon + u / 8.0;

    let lat_in_mesh = lat - second_lat;
    let lon_in_mesh = lon - second_lon;

    // 浮動小数点誤差でメッシュ境界を越えないよう0〜9に制限する
    let v = (math::floor(lat_in_mesh / (5.0 / 60.0) * 10.0) as i32).clamp(0, 9);
    let w = (math::floor(lon_in_mesh / (7.5 / 60.0) * 10.0) as i32).clamp(0, 9);

    second_code * 100 + (v * 10 + w) as u64
}

/// 分割地域メッシュの番号を計算する（JIS X 0410）
///
/// メッシュを南北・東西に2等分し、南西=1、南東=2、北西=3、北東=4を割り当てます。
/// 戻り値は（番号、分割後メッシュ内の残余緯度、残余経度）です。
fn subdivision_index(
    lat_in_mesh: f64,
    lon_in_mesh: f64,
    lat_size: f64,
    lon_size: f64,
) -> (u64, f64, f64) {
    let half_lat = lat_size / 2.0;
    let half_lon = lon_size / 2.0;
    let north = lat_in_mesh >= half_lat;
    let east = lon_in_mesh >= half_lon;

    let index = 1 + 2 * (north as u64) + (east as u64);
    let lat_rem = if north {
        lat_in_mesh - half_lat
    } else {
        lat_in_mesh
    };
    let lon_rem = if east {
        lon_in_mesh - half_lon
    } else {
        lon_in_mesh
    };

    (index, lat_rem, lon_rem)
}

const THIRD_LAT_SIZE: f64 = 30.0 / 3600.0;
const THIRD_LON_SIZE: f64 = 45.0 / 3600.0;

fn calc_fourth_half_mesh(lat: f64, lon: f64, third_code: u64) -> u64 {
    let third_str = format!("{third_code:08}");
    let lat_in_mesh = lat - extract_lat_from_third(&third_str);
    let lon_in_mesh = lon - extract_lon_from_third(&third_str);

    let (index, _, _) = subdivision_index(lat_in_mesh, lon_in_mesh, THIRD_LAT_SIZE, THIRD_LON_SIZE);

    third_code * 10 + index
}

fn calc_fourth_quarter_mesh(lat: f64, lon: f64, third_code: u64) -> u64 {
    let third_str = format!("{third_code:08}");
    let lat_in_mesh = lat - extract_lat_from_third(&third_str);
    let lon_in_mesh = lon - extract_lon_from_third(&third_str);

    let (half, lat_rem, lon_rem) =
        subdivision_index(lat_in_mesh, lon_in_mesh, THIRD_LAT_SIZE, THIRD_LON_SIZE);
    let (quarter, _, _) =
        subdivision_index(lat_rem, lon_rem, THIRD_LAT_SIZE / 2.0, THIRD_LON_SIZE / 2.0);

    third_code * 100 + half * 10 + quarter
}

fn calc_fourth_eighth_mesh(lat: f64, lon: f64, third_code: u64) -> u64 {
    let third_str = format!("{third_code:08}");
    let lat_in_mesh = lat - extract_lat_from_third(&third_str);
    let lon_in_mesh = lon - extract_lon_from_third(&third_str);

    let (half, lat_rem, lon_rem) =
        subdivision_index(lat_in_mesh, lon_in_mesh, THIRD_LAT_SIZE, THIRD_LON_SIZE);
    let (quarter, lat_rem, lon_rem) =
        subdivision_index(lat_rem, lon_rem, THIRD_LAT_SIZE / 2.0, THIRD_LON_SIZE / 2.0);
    let (eighth, _, _) =
        subdivision_index(lat_rem, lon_rem, THIRD_LAT_SIZE / 4.0, THIRD_LON_SIZE / 4.0);

    third_code * 1000 + half * 100 + quarter * 10 + eighth
}

fn calc_fifth_mesh(lat: f64, lon: f64, third_code: u64) -> u64 {
    let third_str = format!("{third_code:08}");
    let lat_in_mesh = lat - extract_lat_from_third(&third_str);
    let lon_in_mesh = lon - extract_lon_from_third(&third_str);

    // 3次メッシュを緯度・経度方向に10等分し、南から北へ0〜9、西から東へ0〜9を割り当てる
    let lat_no = (math::floor(lat_in_mesh / (3.0 / 3600.0)) as i32).clamp(0, 9);
    let lon_no = (math::floor(lon_in_mesh / (4.5 / 3600.0)) as i32).clamp(0, 9);

    third_code * 100 + (lat_no * 10 + lon_no) as u64
}

fn extract_lat_from_third(third_str: &str) -> f64 {
    let p = third_str[0..1].parse::<f64>().unwrap();
    let q = third_str[1..2].parse::<f64>().unwrap();
    let t = third_str[4..5].parse::<f64>().unwrap();
    let v = third_str[6..7].parse::<f64>().unwrap();

    let first_lat = (p * 10.0 + q) / 1.5;
    let second_lat = first_lat + t * (40.0 / 60.0) / 8.0;

    second_lat + v * (5.0 / 60.0) / 10.0
}

fn extract_lon_from_third(third_str: &str) -> f64 {
    let r = third_str[2..3].parse::<f64>().unwrap();
    let s = third_str[3..4].parse::<f64>().unwrap();
    let u = third_str[5..6].parse::<f64>().unwrap();
    let w = third_str[7..8].parse::<f64>().unwrap();

    let first_lon = r * 10.0 + s + 100.0;
    let second_lon = first_lon + u / 8.0;

    second_lon + w * (7.5 / 60.0) / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokyo_station_first_mesh() {
        let coord = Coordinate::new(35.6812, 139.7671).unwrap();
        let mesh = coord_to_mesh(coord, MeshLevel::First).unwrap();
        assert_eq!(mesh.as_string(), "5339");
    }

    #[test]
    fn test_tokyo_station_third_mesh() {
        let coord = Coordinate::new(35.6812, 139.7671).unwrap();
        let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();
        assert_eq!(mesh.as_string(), "53394611");
    }

    #[test]
    fn test_subdivision_index_jis_numbering() {
        // 南西=1、南東=2、北西=3、北東=4
        let (sw, _, _) = subdivision_index(0.0, 0.0, 1.0, 1.0);
        let (se, _, _) = subdivision_index(0.0, 0.6, 1.0, 1.0);
        let (nw, _, _) = subdivision_index(0.6, 0.0, 1.0, 1.0);
        let (ne, _, _) = subdivision_index(0.6, 0.6, 1.0, 1.0);
        assert_eq!((sw, se, nw, ne), (1, 2, 3, 4));
    }
}

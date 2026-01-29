use crate::error::Result;
use crate::types::{Coordinate, MeshCode, MeshLevel};

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
    let lat_times_1_5 = lat * 1.5;
    let p = (lat_times_1_5.floor() as i32) / 10;
    let q = (lat_times_1_5.floor() as i32) % 10;

    let lon_minus_100 = lon - 100.0;
    let r = (lon_minus_100.floor() as i32) / 10;
    let s = (lon_minus_100.floor() as i32) % 10;

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

    let t = (lat_in_mesh / (40.0 / 60.0) * 8.0).floor() as i32;
    let u = (lon_in_mesh / 1.0 * 8.0).floor() as i32;

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

    let v = (lat_in_mesh / (5.0 / 60.0) * 10.0).floor() as i32;
    let w = (lon_in_mesh / (7.5 / 60.0) * 10.0).floor() as i32;

    second_code * 100 + (v * 10 + w) as u64
}

fn calc_fourth_half_mesh(lat: f64, lon: f64, third_code: u64) -> u64 {
    let third_str = format!("{third_code:08}");
    let third_lat = extract_lat_from_third(&third_str);
    let third_lon = extract_lon_from_third(&third_str);

    let lat_in_mesh = lat - third_lat;
    let lon_in_mesh = lon - third_lon;

    let lat_half = lat_in_mesh / (30.0 / 3600.0);
    let lon_half = lon_in_mesh / (45.0 / 3600.0);

    let index = if lat_half >= 0.5 {
        if lon_half >= 0.5 {
            1
        } else {
            3
        }
    } else if lon_half >= 0.5 {
        2
    } else {
        4
    };

    third_code * 10 + index
}

fn calc_fourth_quarter_mesh(lat: f64, lon: f64, third_code: u64) -> u64 {
    let third_str = format!("{third_code:08}");
    let third_lat = extract_lat_from_third(&third_str);
    let third_lon = extract_lon_from_third(&third_str);

    let lat_in_mesh = lat - third_lat;
    let lon_in_mesh = lon - third_lon;

    let lat_quarter = (lat_in_mesh / (7.5 / 3600.0)).floor() as i32;
    let lon_quarter = (lon_in_mesh / (11.25 / 3600.0)).floor() as i32;

    let index = lat_quarter * 4 + lon_quarter + 1;

    third_code * 100 + index as u64
}

fn calc_fourth_eighth_mesh(lat: f64, lon: f64, third_code: u64) -> u64 {
    let third_str = format!("{third_code:08}");
    let third_lat = extract_lat_from_third(&third_str);
    let third_lon = extract_lon_from_third(&third_str);

    let lat_in_mesh = lat - third_lat;
    let lon_in_mesh = lon - third_lon;

    let lat_eighth = (lat_in_mesh / (3.75 / 3600.0)).floor() as i32;
    let lon_eighth = (lon_in_mesh / (5.625 / 3600.0)).floor() as i32;

    let index = lat_eighth * 8 + lon_eighth + 1;

    third_code * 1000 + index as u64
}

fn calc_fifth_mesh(lat: f64, lon: f64, third_code: u64) -> u64 {
    let third_str = format!("{third_code:08}");
    let third_lat = extract_lat_from_third(&third_str);
    let third_lon = extract_lon_from_third(&third_str);

    let lat_in_mesh = lat - third_lat;
    let lon_in_mesh = lon - third_lon;

    let lat_fifth = (lat_in_mesh / (3.0 / 3600.0)).floor() as i32;
    let lon_fifth = (lon_in_mesh / (4.5 / 3600.0)).floor() as i32;

    let index = lat_fifth * 10 + lon_fifth + 1;

    third_code * 100 + index as u64
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
}

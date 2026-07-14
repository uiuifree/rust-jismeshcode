use crate::types::{BoundingBox, Coordinate, MeshCode, MeshLevel};

pub fn mesh_to_bounds(mesh: MeshCode) -> BoundingBox {
    let level = mesh.level();
    let code_str = mesh.as_string();

    let (sw_lat, sw_lon) = match level {
        MeshLevel::First => calc_first_mesh_sw(&code_str),
        MeshLevel::Second => calc_second_mesh_sw(&code_str),
        MeshLevel::Third => calc_third_mesh_sw(&code_str),
        MeshLevel::FourthHalf => calc_fourth_half_mesh_sw(&code_str),
        MeshLevel::FourthQuarter => calc_fourth_quarter_mesh_sw(&code_str),
        MeshLevel::FourthEighth => calc_fourth_eighth_mesh_sw(&code_str),
        MeshLevel::Fifth => calc_fifth_mesh_sw(&code_str),
    };

    let lat_size = level.lat_size_degrees();
    let lon_size = level.lon_size_degrees();

    let sw = Coordinate::new_unchecked(sw_lat, sw_lon);
    let ne = Coordinate::new_unchecked(sw_lat + lat_size, sw_lon + lon_size);

    BoundingBox::new(sw, ne)
}

pub fn mesh_to_center(mesh: MeshCode) -> Coordinate {
    let bounds = mesh_to_bounds(mesh);
    bounds.center()
}

fn calc_first_mesh_sw(code_str: &str) -> (f64, f64) {
    let p = code_str[0..1].parse::<f64>().unwrap();
    let q = code_str[1..2].parse::<f64>().unwrap();
    let r = code_str[2..3].parse::<f64>().unwrap();
    let s = code_str[3..4].parse::<f64>().unwrap();

    let lat = (p * 10.0 + q) / 1.5;
    let lon = r * 10.0 + s + 100.0;

    (lat, lon)
}

fn calc_second_mesh_sw(code_str: &str) -> (f64, f64) {
    let (first_lat, first_lon) = calc_first_mesh_sw(&code_str[0..4]);

    let t = code_str[4..5].parse::<f64>().unwrap();
    let u = code_str[5..6].parse::<f64>().unwrap();

    let lat = first_lat + t * (40.0 / 60.0) / 8.0;
    let lon = first_lon + u / 8.0;

    (lat, lon)
}

fn calc_third_mesh_sw(code_str: &str) -> (f64, f64) {
    let (second_lat, second_lon) = calc_second_mesh_sw(&code_str[0..6]);

    let v = code_str[6..7].parse::<f64>().unwrap();
    let w = code_str[7..8].parse::<f64>().unwrap();

    let lat = second_lat + v * (5.0 / 60.0) / 10.0;
    let lon = second_lon + w * (7.5 / 60.0) / 10.0;

    (lat, lon)
}

/// 分割地域メッシュの番号（1〜4）から南西端オフセットの単位を返す
///
/// JIS X 0410の番号付け（南西=1、南東=2、北西=3、北東=4）に対応します。
/// 戻り値は（緯度方向、経度方向）で、各値は0または1です。
fn subdivision_offset_units(digit: u32) -> (f64, f64) {
    let index = digit - 1;
    ((index / 2) as f64, (index % 2) as f64)
}

const THIRD_LAT_SIZE: f64 = 30.0 / 3600.0;
const THIRD_LON_SIZE: f64 = 45.0 / 3600.0;

fn calc_fourth_half_mesh_sw(code_str: &str) -> (f64, f64) {
    let (third_lat, third_lon) = calc_third_mesh_sw(&code_str[0..8]);

    let digit = code_str[8..9].parse::<u32>().unwrap();
    let (lat_units, lon_units) = subdivision_offset_units(digit);

    let lat = third_lat + lat_units * (THIRD_LAT_SIZE / 2.0);
    let lon = third_lon + lon_units * (THIRD_LON_SIZE / 2.0);

    (lat, lon)
}

fn calc_fourth_quarter_mesh_sw(code_str: &str) -> (f64, f64) {
    let (half_lat, half_lon) = calc_fourth_half_mesh_sw(&code_str[0..9]);

    let digit = code_str[9..10].parse::<u32>().unwrap();
    let (lat_units, lon_units) = subdivision_offset_units(digit);

    let lat = half_lat + lat_units * (THIRD_LAT_SIZE / 4.0);
    let lon = half_lon + lon_units * (THIRD_LON_SIZE / 4.0);

    (lat, lon)
}

fn calc_fourth_eighth_mesh_sw(code_str: &str) -> (f64, f64) {
    let (quarter_lat, quarter_lon) = calc_fourth_quarter_mesh_sw(&code_str[0..10]);

    let digit = code_str[10..11].parse::<u32>().unwrap();
    let (lat_units, lon_units) = subdivision_offset_units(digit);

    let lat = quarter_lat + lat_units * (THIRD_LAT_SIZE / 8.0);
    let lon = quarter_lon + lon_units * (THIRD_LON_SIZE / 8.0);

    (lat, lon)
}

fn calc_fifth_mesh_sw(code_str: &str) -> (f64, f64) {
    let (third_lat, third_lon) = calc_third_mesh_sw(&code_str[0..8]);

    // 9桁目が緯度方向番号（0〜9）、10桁目が経度方向番号（0〜9）
    let lat_no = code_str[8..9].parse::<f64>().unwrap();
    let lon_no = code_str[9..10].parse::<f64>().unwrap();

    let lat = third_lat + lat_no * (3.0 / 3600.0);
    let lon = third_lon + lon_no * (4.5 / 3600.0);

    (lat, lon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_mesh_bounds() {
        let mesh = MeshCode::from_str("5339").unwrap();
        let bounds = mesh_to_bounds(mesh);

        let expected_lat = (5.0 * 10.0 + 3.0) / 1.5;
        let expected_lon = 3.0 * 10.0 + 9.0 + 100.0;

        assert!((bounds.min_lat() - expected_lat).abs() < 1e-10);
        assert!((bounds.min_lon() - expected_lon).abs() < 1e-10);
    }

    #[test]
    fn test_third_mesh_center() {
        let mesh = MeshCode::from_str("53393599").unwrap();
        let center = mesh_to_center(mesh);

        assert!(center.lat() >= 35.0 && center.lat() <= 36.0);
        assert!(center.lon() >= 139.0 && center.lon() <= 140.0);
    }
}

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

fn calc_fourth_half_mesh_sw(code_str: &str) -> (f64, f64) {
    let (third_lat, third_lon) = calc_third_mesh_sw(&code_str[0..8]);

    let index = code_str[8..9].parse::<i32>().unwrap();

    let (lat_offset, lon_offset) = match index {
        1 => (0.5, 0.5),
        2 => (0.0, 0.5),
        3 => (0.5, 0.0),
        4 => (0.0, 0.0),
        _ => (0.0, 0.0),
    };

    let lat = third_lat + lat_offset * (30.0 / 3600.0);
    let lon = third_lon + lon_offset * (45.0 / 3600.0);

    (lat, lon)
}

fn calc_fourth_quarter_mesh_sw(code_str: &str) -> (f64, f64) {
    let (third_lat, third_lon) = calc_third_mesh_sw(&code_str[0..8]);

    let index = code_str[8..10].parse::<i32>().unwrap() - 1;
    let lat_index = index / 4;
    let lon_index = index % 4;

    let lat = third_lat + lat_index as f64 * (7.5 / 3600.0);
    let lon = third_lon + lon_index as f64 * (11.25 / 3600.0);

    (lat, lon)
}

fn calc_fourth_eighth_mesh_sw(code_str: &str) -> (f64, f64) {
    let (third_lat, third_lon) = calc_third_mesh_sw(&code_str[0..8]);

    let index = code_str[8..11].parse::<i32>().unwrap() - 1;
    let lat_index = index / 8;
    let lon_index = index % 8;

    let lat = third_lat + lat_index as f64 * (3.75 / 3600.0);
    let lon = third_lon + lon_index as f64 * (5.625 / 3600.0);

    (lat, lon)
}

fn calc_fifth_mesh_sw(code_str: &str) -> (f64, f64) {
    let (third_lat, third_lon) = calc_third_mesh_sw(&code_str[0..8]);

    let index = code_str[8..10].parse::<i32>().unwrap() - 1;
    let lat_index = index / 10;
    let lon_index = index % 10;

    let lat = third_lat + lat_index as f64 * (3.0 / 3600.0);
    let lon = third_lon + lon_index as f64 * (4.5 / 3600.0);

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

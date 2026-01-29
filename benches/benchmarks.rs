use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jismeshcode::prelude::*;

fn bench_coord_to_mesh(c: &mut Criterion) {
    c.bench_function("coord_to_mesh_third", |b| {
        let coord = Coordinate::new(35.6812, 139.7671).unwrap();
        b.iter(|| coord_to_mesh(black_box(coord), black_box(MeshLevel::Third)))
    });
}

fn bench_mesh_to_bounds(c: &mut Criterion) {
    c.bench_function("mesh_to_bounds", |b| {
        let mesh = MeshCode::from_str("53393599").unwrap();
        b.iter(|| mesh_to_bounds(black_box(mesh)))
    });
}

fn bench_neighbors(c: &mut Criterion) {
    c.bench_function("neighbors", |b| {
        let mesh = MeshCode::from_str("53393599").unwrap();
        b.iter(|| neighbors(black_box(mesh)))
    });
}

criterion_group!(
    benches,
    bench_coord_to_mesh,
    bench_mesh_to_bounds,
    bench_neighbors
);
criterion_main!(benches);

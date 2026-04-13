use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sovereign_hdc::{Hypervector, Bundle};

fn bench_holographic_ops(c: &mut Criterion) {
    let hv1 = Hypervector::random();
    let hv2 = Hypervector::random();

    let mut group = c.benchmark_group("Holographic_Substrate");

    group.bench_function("Binding (XOR)", |b| {
        b.iter(|| hv1.bind(black_box(&hv2)))
    });

    group.bench_function("Similarity (Hamming)", |b| {
        b.iter(|| hv1.similarity(black_box(&hv2)))
    });

    group.bench_function("Permutation (Rotate)", |b| {
        b.iter(|| hv1.rotate(black_box(42)))
    });

    group.bench_function("Bundling (100 vectors)", |b| {
        let hvs: Vec<Hypervector> = (0..100).map(|_| Hypervector::random()).collect();
        b.iter(|| {
            let mut bundle = Bundle::new();
            for hv in &hvs {
                bundle.add(hv);
            }
            bundle.finalize()
        })
    });

    group.finish();
}

criterion_group!(benches, bench_holographic_ops);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use libsodium_clone::hash_bytes;
use sodiumoxide::crypto::hash::sha256;

fn bench_hash(c: &mut Criterion) {
    let data = b"benchmark input repeated to have some workbench";

    c.bench_function("our_sha256", |b| b.iter(|| {
        let _ = hash_bytes(data);
    }));

    c.bench_function("libsodium_sha256", |b| b.iter(|| {
        let _ = sha256::hash(data);
    }));
}

criterion_group!(benches, bench_hash);
criterion_main!(benches);

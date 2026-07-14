use criterion::{criterion_group, criterion_main, Criterion};
use libsodium_clone::secretbox_encrypt;
use sodiumoxide::crypto::secretbox::{self, Key, Nonce};

fn bench_secretbox(c: &mut Criterion) {
    let key = [7u8; 32];
    let nonce = [11u8; 24];
    let data = b"benchmark plaintext for secretbox";

    c.bench_function("our_secretbox", |b| b.iter(|| {
        let _ = secretbox_encrypt(data, &key, &nonce);
    }));

    c.bench_function("libsodium_secretbox", |b| b.iter(|| {
        let _ = secretbox::seal(data, &Nonce(nonce), &Key(key));
    }));
}

criterion_group!(benches, bench_secretbox);
criterion_main!(benches);

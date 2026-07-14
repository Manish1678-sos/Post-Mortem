use libsodium_clone::{hash_bytes, secretbox_encrypt};
use rand::{rngs::StdRng, Rng, SeedableRng};
use sodiumoxide::crypto::{hash::sha256, secretbox};

#[test]
fn differential_parity_checks() {
    let mut rng = StdRng::seed_from_u64(0xDEADBEEF);
    let mut messages: Vec<Vec<u8>> = vec![
        b"".to_vec(),
        b"short".to_vec(),
        b"some longer message for testing parity".to_vec(),
        b"same parameters for both implementations".to_vec(),
    ];

    for _ in 0..6 {
        let len = rng.gen_range(1..64);
        let mut data = vec![0u8; len];
        rng.fill(&mut data[..]);
        messages.push(data);
    }

    let key = [7u8; 32];
    let nonce = [11u8; 24];

    for m in messages.iter() {
        let msg_ref: &[u8] = m.as_slice();

        let ours_hash = hash_bytes(msg_ref);
        let theirs_hash = sha256::hash(msg_ref).0.to_vec();
        assert_eq!(ours_hash, theirs_hash, "hash mismatch for message: {:?}", msg_ref);

        let ours_ct = secretbox_encrypt(msg_ref, &key, &nonce);
        let theirs_ct = secretbox::seal(msg_ref, &secretbox::Nonce(nonce), &secretbox::Key(key));
        assert_eq!(ours_ct, theirs_ct, "secretbox mismatch for message: {:?}", msg_ref);
    }
}

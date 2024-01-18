use blake2::digest::typenum::U64;
use blake2::{Blake2b, Digest};
use rand::thread_rng;
use tari_crypto::hash_domain;
use tari_crypto::hashing::DomainSeparatedHasher;
use tari_crypto::keys::{PublicKey, SecretKey};
use tari_crypto::ristretto::{RistrettoPublicKey, RistrettoSecretKey};
use tari_crypto::tari_utilities::ByteArray;

hash_domain!(MusigDomain, "musig_demo");

#[allow(non_snake_case)]
fn main() {
    let (k1, X1, r1, R1) = get_keys();
    let (k2, X2, r2, R2) = get_keys();
    let (k3, X3, r3, R3) = get_keys();

    // I'm setting the order here. In general, they'll be sorted
    let l = calculate_challenge(&[X1.as_bytes(), &X2.as_bytes(), &X3.as_bytes()], "L");
    // ai = H(l || p)
    let a1 = calculate_challenge(&[l.as_bytes(), X1.as_bytes()], "a1");
    let a2 = calculate_challenge(&[l.as_bytes(), X2.as_bytes()], "a2");
    let a3 = calculate_challenge(&[l.as_bytes(), X3.as_bytes()], "a3");
    // X = sum( a_i X_i)
    let X = &a1 * X1 + &a2 * X2 + &a3 * X3;

    let m = calculate_challenge(&[b"SomeSharedMultiSigTx"], "message");

    // Calc shared nonce
    let R = R1 + R2 + R3;

    // e = H(R || X || m)
    let e = calculate_challenge(&[R.as_bytes(), X.as_bytes(), m.as_bytes()], "challenge");

    // Signatures
    let s1 = r1 + k1 * a1 * &e;
    let s2 = r2 + k2 * a2 * &e;
    let s3 = r3 + k3 * a3 * &e;
    let s = s1 + s2 + s3;

    //Verify
    let sg = RistrettoPublicKey::from_secret_key(&s);
    let check = R + e * X;
    assert_eq!(sg, check, "The signature is INVALID");
    println!("The signature is correct!")
}

#[allow(non_snake_case)]
fn get_keys() -> (
    RistrettoSecretKey,
    RistrettoPublicKey,
    RistrettoSecretKey,
    RistrettoPublicKey,
) {
    let mut rng = thread_rng();
    let k = RistrettoSecretKey::random(&mut rng);
    let P = RistrettoPublicKey::from_secret_key(&k);
    let r = RistrettoSecretKey::random(&mut rng);
    let R = RistrettoPublicKey::from_secret_key(&r);
    (k, P, r, R)
}

fn calculate_challenge(values: &[&[u8]], label: &'static str) -> RistrettoSecretKey {
    let hash = values
        .iter()
        .fold(
            DomainSeparatedHasher::<Blake2b<U64>, MusigDomain>::new_with_label(label),
            |acc, v| acc.chain_update(v),
        )
        .finalize();
    RistrettoSecretKey::from_uniform_bytes(hash.as_ref()).unwrap()
}

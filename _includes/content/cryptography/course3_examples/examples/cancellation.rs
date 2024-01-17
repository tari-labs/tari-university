use blake2::digest::typenum::U64;
use blake2::{Blake2b, Digest};
use rand::thread_rng;
use tari_crypto::hash_domain;
use tari_crypto::hashing::DomainSeparatedHasher;
use tari_crypto::keys::{PublicKey, SecretKey};
use tari_crypto::ristretto::{RistrettoPublicKey, RistrettoSecretKey};
use tari_crypto::tari_utilities::ByteArray;

// Domain separation prevents hashes accidentally colliding, are letting messages from one application be used in another
hash_domain!(CDomain, "cancellation_demo");

#[allow(non_snake_case)]
fn main() {
    // Alice generates some keys
    let (_ka, Pa, _ra, Ra) = get_keyset();
    // Bob generates some keys as before
    let (kb, Pb, rb, Rb) = get_keyset();
    // ..and then publishes his forged keys
    let Pf = &Pb - &Pa;
    let Rf = &Rb - &Ra;
    // The challenge uses both nonce public keys and private keys
    // e = H(Ra || Rb' || Pa || Pb' || H(m))
    let hash =
        DomainSeparatedHasher::<Blake2b<U64>, CDomain>::new_with_label("aggregated_challenge")
            .chain_update(Ra.as_bytes())
            .chain_update(Rf.as_bytes())
            .chain_update(Pa.as_bytes())
            .chain_update(Pf.as_bytes())
            .chain_update(b"a multisig transaction")
            .finalize();
    let e = RistrettoSecretKey::from_uniform_bytes(hash.as_ref()).unwrap();
    // Bob creates a forged signature
    let s_f = &rb + &e * &kb;
    // Check if it's valid
    let sG = &Ra + &Rf + &e * (&Pa + &Pf);
    assert_eq!(sG, PublicKey::from_secret_key(&s_f));
    println!("Bob successfully forged the aggregate signature!")
}

#[allow(non_snake_case)]
fn get_keyset() -> (
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

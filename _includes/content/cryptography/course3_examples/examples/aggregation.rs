use blake2::digest::typenum::U64;
use blake2::{Blake2b, Digest};
use generic_array;
use rand::thread_rng;
use tari_crypto;
use tari_crypto::hash_domain;
use tari_crypto::hashing::DomainSeparatedHasher;
use tari_crypto::keys::{PublicKey, SecretKey};
use tari_crypto::ristretto::{RistrettoPublicKey, RistrettoSchnorr, RistrettoSecretKey};
use tari_crypto::tari_utilities::hex::{to_hex, Hex};
use tari_crypto::tari_utilities::ByteArray;

#[allow(non_snake_case)]
fn main() {
    let mut rng = thread_rng();
    // Alice generates some keys
    let (ka, Pa) = RistrettoPublicKey::random_keypair(&mut rng);
    // Alice generates a nonce
    let (ra, Ra) = RistrettoPublicKey::random_keypair(&mut rng);

    // Bob generates some keys
    let (kb, Pb) = RistrettoPublicKey::random_keypair(&mut rng);
    // Bob generates a nonce
    let (rb, Rb) = RistrettoPublicKey::random_keypair(&mut rng);
    // // The challenge uses both nonce public keys and private keys
    // // e = H(Ra || Rb || Pa || Pb || H(m))
    let e = calculate_challenge(&Ra, &Rb, &Pa, &Pb, "This is a multisig");
    // Alice calculates her signature
    let sa = ra + ka * &e;
    // Bob calculates his signature
    let sb = rb + kb * &e;
    // Calculate the aggregate signature
    let s_agg = sa + sb;
    // S = s_agg.G
    let S = RistrettoPublicKey::from_secret_key(&s_agg);
    // This should equal Ra + Rb + e(Pa + Pb)
    assert_eq!(S, Ra + Rb + e * (Pa + Pb));
    println!("The aggregate signature is valid!");
}

// Domain separation prevents hashes accidentally colliding, are letting messages from one application be used in another
hash_domain!(ExampleDomain, "aggregation_demo");

#[allow(non_snake_case)]
fn calculate_challenge(
    Ra: &RistrettoPublicKey,
    Rb: &RistrettoPublicKey,
    Pa: &RistrettoPublicKey,
    Pb: &RistrettoPublicKey,
    msg: &str,
) -> RistrettoSecretKey {
    let hash = DomainSeparatedHasher::<Blake2b<U64>, ExampleDomain>::new_with_label(
        "aggregated_challenge",
    )
    .chain_update(Ra.as_bytes())
    .chain_update(Rb.as_bytes())
    .chain_update(Pa.as_bytes())
    .chain_update(Pb.as_bytes())
    .chain_update(msg.as_bytes())
    .finalize();
    RistrettoSecretKey::from_uniform_bytes(hash.as_ref()).unwrap()
}

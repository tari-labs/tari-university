use blake2::digest::typenum::U64;
use blake2::Blake2b;
use rand::thread_rng;
use tari_crypto::hash_domain;
use tari_crypto::hashing::DomainSeparatedHasher;
use tari_crypto::keys::{PublicKey, SecretKey};
use tari_crypto::ristretto::{RistrettoPublicKey, RistrettoSecretKey};
use tari_crypto::tari_utilities::ByteArray;

hash_domain!(ECDH, "ecdh_demo");

#[allow(non_snake_case)]
fn main() {
    let mut rng = thread_rng();
    // Alice creates a public-private keypair
    let k_a = RistrettoSecretKey::random(&mut rng);
    let P_a = RistrettoPublicKey::from_secret_key(&k_a);
    // Bob creates a public-private keypair
    let k_b = RistrettoSecretKey::random(&mut rng);
    let P_b = RistrettoPublicKey::from_secret_key(&k_b);
    // They each calculate the shared secret based only on the other party's public information
    // Alice's version:
    let S_a = hash_pubkey(k_a * P_b);
    println!("Alice's version: {}", S_a.reveal());
    // Bob's version:
    let S_b = hash_pubkey(k_b * P_a);
    println!("Bob's version:   {}", S_b.reveal());

    assert_eq!(S_a, S_b, "The shared secret is not the same!");
    println!("The shared secret is identical")
}

fn hash_pubkey(p: RistrettoPublicKey) -> RistrettoSecretKey {
    let hash = DomainSeparatedHasher::<Blake2b<U64>, ECDH>::new_with_label("ecdh_derive")
        .digest(p.as_bytes());
    RistrettoSecretKey::from_uniform_bytes(hash.as_ref()).unwrap()
}

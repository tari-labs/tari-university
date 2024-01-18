use rand::thread_rng;
use tari_crypto::keys::{PublicKey, SecretKey};
use tari_crypto::ristretto::{RistrettoPublicKey, RistrettoSchnorr, RistrettoSecretKey};
use tari_crypto::signatures::SchnorrSignature;

#[allow(non_snake_case)]
fn main() {
    // Create a random private key
    let mut rng = thread_rng();
    let k = RistrettoSecretKey::random(&mut rng);
    println!("My private key: {}", k.reveal());
    let P = RistrettoPublicKey::from_secret_key(&k);
    // Challenge, e = H(P || m), which effectively random
    let e = RistrettoSecretKey::random(&mut rng);

    // "Signature" without a nonce. DON'T DO THIS!
    let s = &e * &k;

    // Verify the signature
    assert_eq!(RistrettoPublicKey::from_secret_key(&s), &e * P);
    println!("'Signature' is valid... but it's not a signature!");
    // But let's try calculate the private key from known information
    let hacked = s * e.invert().unwrap();
    assert_eq!(k, hacked);
    println!("Hacked key:     {}", k.reveal())
}

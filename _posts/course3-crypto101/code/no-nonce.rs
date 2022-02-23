extern crate libsecp256k1_rs as secp256k1;

use secp256k1::{SecretKey, PublicKey, thread_rng, Message};
use secp256k1::schnorr::{ Challenge};

#[allow(non_snake_case)]
fn main() {
    // Create a random private key
    let mut rng = thread_rng();
    let k = SecretKey::random(&mut rng);
    println!("My private key: {}", k);
    let P = PublicKey::from_secret_key(&k);
    let m = Message::hash(b"Meet me at 12").unwrap();
    // Challenge, e = H(P || m)
    let e = Challenge::new(&[&P, &m]).as_scalar().unwrap();

    // Signature
    let s = e * k;

    // Verify the signature
    assert_eq!(PublicKey::from_secret_key(&s), e*P);
    println!("Signature is valid!");
    // But let's try calculate the private key from known information
    let hacked = s * e.inv();
    assert_eq!(k, hacked);
    println!("Hacked key:     {}", k)
}

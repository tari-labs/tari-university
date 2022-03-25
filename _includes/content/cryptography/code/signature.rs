extern crate libsecp256k1_rs as secp256k1;

use secp256k1::{ SecretKey, PublicKey, thread_rng, Message };
use secp256k1::schnorr::{ Schnorr, Challenge };

#[allow(non_snake_case)]
fn main() {
    let mut rng = thread_rng();
    let k = SecretKey::random(&mut rng);
    let P = PublicKey::from_secret_key(&k);
    let nonce = SecretKey::random(&mut rng);
    let R = PublicKey::from_secret_key(&nonce);
    let m = Message::hash(b"Attack at dawn").unwrap();
    let e = Challenge::new(&[&R, &P, &m]).as_scalar().unwrap();

    // Signature
    let s = nonce + e * k;
    //Verify
    let sg = PublicKey::from_secret_key(&s);
    let check = R + e * P;
    assert_eq!(sg, check, "The signature is INVALID");
    println!("The signature is correct!")
}

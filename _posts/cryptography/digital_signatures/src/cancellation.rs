extern crate libsecp256k1_rs as secp256k1;

use secp256k1::{SecretKey, PublicKey, thread_rng, Message};
use secp256k1::schnorr::{Schnorr, Challenge};

#[allow(non_snake_case)]
fn main() {
    // Alice generates some keys
    let (ka, Pa, ra, Ra) = get_keyset();
    // Bob generates some keys as before
    let (kb, Pb, rb, Rb) = get_keyset();
    // ..and then publishes his forged keys
    let Pf = Pb - Pa;
    let Rf = Rb - Ra;

    let m = Message::hash(b"a multisig transaction").unwrap();
    // The challenge uses both nonce public keys and private keys
    // e = H(Ra || Rb' || Pa || Pb' || H(m))
    let e = Challenge::new(&[&Ra, &Rf, &Pa, &Pf, &m]).as_scalar().unwrap();

    // Bob creates a forged signature
    let s_f = rb + e * kb;
    // Check if it's valid
    let sG = Ra + Rf + e*(Pa + Pf);
    assert_eq!(sG, PublicKey::from_secret_key(&s_f));
    println!("Bob successfully forged the aggregate signature!")
}

#[allow(non_snake_case)]
fn get_keyset() -> (SecretKey, PublicKey, SecretKey, PublicKey) {
    let mut rng = thread_rng();
    let k = SecretKey::random(&mut rng);
    let P = PublicKey::from_secret_key(&k);
    let r = SecretKey::random(&mut rng);
    let R = PublicKey::from_secret_key(&r);
    (k, P, r, R)
}

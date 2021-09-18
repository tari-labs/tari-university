extern crate libsecp256k1_rs as secp256k1;

use secp256k1::{SecretKey, PublicKey, thread_rng, Message};
use secp256k1::schnorr::{Schnorr, Challenge};

#[allow(non_snake_case)]
fn main() {
    // Alice generates some keys
    let (ka, Pa, ra, Ra) = get_keyset();
    // Bob generates some keys
    let (kb, Pb, rb, Rb) = get_keyset();
    let m = Message::hash(b"a multisig transaction").unwrap();
    // The challenge uses both nonce public keys and private keys
    // e = H(Ra || Rb || Pa || Pb || H(m))
    let e = Challenge::new(&[&Ra, &Rb, &Pa, &Pb, &m]).as_scalar().unwrap();
    // Alice calculates her signature
    let sa = ra + ka * e;
    // Bob calculates his signature
    let sb = rb + kb * e;
    // Calculate the aggregate signature
    let s_agg = sa + sb;
    // S = s_agg.G
    let S = PublicKey::from_secret_key(&s_agg);
    // This should equal Ra + Rb + e(Pa + Pb)
    assert_eq!(S, Ra + Rb + e*(Pa + Pb));
    println!("The aggregate signature is valid!")
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

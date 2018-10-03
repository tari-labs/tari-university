extern crate libsecp256k1_rs as secp256k1;

use secp256k1::{ SecretKey, PublicKey, thread_rng, Message };
use secp256k1::schnorr::{ Challenge };

#[allow(non_snake_case)]
fn main() {
    let (k1, X1, r1, R1) = get_keys();
    let (k2, X2, r2, R2) = get_keys();
    let (k3, X3, r3, R3) = get_keys();

    // I'm setting the order here. In general, they'll be sorted
    let l = Challenge::new(&[&X1, &X2, &X3]);
    // ai = H(l || p)
    let a1 = Challenge::new(&[ &l, &X1 ]).as_scalar().unwrap();
    let a2 = Challenge::new(&[ &l, &X2 ]).as_scalar().unwrap();
    let a3 = Challenge::new(&[ &l, &X3 ]).as_scalar().unwrap();
    // X = sum( a_i X_i)
    let X = a1 * X1 + a2 * X2 + a3 * X3;

    let m = Message::hash(b"SomeSharedMultiSigTx").unwrap();

    // Calc shared nonce
    let R = R1 + R2 + R3;

    // e = H(R || X || m)
    let e = Challenge::new(&[&R, &X, &m]).as_scalar().unwrap();


    // Signatures
    let s1 = r1 + k1 * a1 * e;
    let s2 = r2 + k2 * a2 * e;
    let s3 = r3 + k3 * a3 * e;
    let s = s1 + s2 + s3;

    //Verify
    let sg = PublicKey::from_secret_key(&s);
    let check = R + e * X;
    assert_eq!(sg, check, "The signature is INVALID");
    println!("The signature is correct!")
}

#[allow(non_snake_case)]
fn get_keys() -> (SecretKey, PublicKey, SecretKey, PublicKey) {
    let mut rng = thread_rng();
    let k = SecretKey::random(&mut rng);
    let P = PublicKey::from_secret_key(&k);
    let r = SecretKey::random(&mut rng);
    let R = PublicKey::from_secret_key(&r);
    (k, P, r, R)
}
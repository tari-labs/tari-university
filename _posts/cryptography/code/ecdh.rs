extern crate libsecp256k1_rs as secp256k1;

use secp256k1::{ SecretKey, PublicKey, thread_rng, Message };

#[allow(non_snake_case)]
fn main() {
    let mut rng = thread_rng();
    // Alice creates a public-private keypair
    let k_a = SecretKey::random(&mut rng);
    let P_a = PublicKey::from_secret_key(&k_a);
    // Bob creates a public-private keypair
    let k_b = SecretKey::random(&mut rng);
    let P_b = PublicKey::from_secret_key(&k_b);
    // They each calculate the shared secret based only on the other party's public information
    // Alice's version:
    let S_a = k_a * P_b;
    // Bob's version:
    let S_b = k_b * P_a;

    assert_eq!(S_a, S_b, "The shared secret is not the same!");
    println!("The shared secret is identical")
}

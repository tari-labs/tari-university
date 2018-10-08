extern crate libsecp256k1_rs;

use libsecp256k1_rs::{ SecretKey, PublicKey };

#[allow(non_snake_case)]
fn main() {
    // Create the secret key "1"
    let k = SecretKey::from_hex("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    // Generate the public key, P = k.G
    let pub_from_k = PublicKey::from_secret_key(&k);
    let known_pub = PublicKey::from_hex("0479BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8").unwrap();
    // Compare it to the known value
    assert_eq!(pub_from_k, known_pub);
    println!("Ok")
}

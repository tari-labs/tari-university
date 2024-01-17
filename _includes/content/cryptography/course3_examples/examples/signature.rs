use rand::thread_rng;
use tari_crypto::keys::{PublicKey, SecretKey};
use tari_crypto::ristretto::{RistrettoPublicKey, RistrettoSchnorr, RistrettoSecretKey};

#[allow(non_snake_case)]
fn main() {
    let mut rng = thread_rng();
    let k = RistrettoSecretKey::random(&mut rng);
    let P = RistrettoPublicKey::from_secret_key(&k);
    let s = RistrettoSchnorr::sign(&k, "My message", &mut rng).unwrap();
    println!(
        "Signature is (R:{}, s:{})",
        s.get_public_nonce(),
        s.get_signature().reveal()
    );
    //Verify
    let result = s.verify(&P, "My message");
    if result {
        println!("Signature is valid!");
    } else {
        println!("Signature is invalid!");
    }
    let result = s.verify(&P, "A different message");
    if result {
        println!("Oh no! Signature verified on a different message!");
    } else {
        println!("Signature is invalid on a different message, as we expected!");
    }
}

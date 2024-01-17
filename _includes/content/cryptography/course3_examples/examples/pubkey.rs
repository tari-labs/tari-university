use tari_crypto;
use tari_crypto::keys::PublicKey;
use tari_crypto::ristretto::pedersen::RISTRETTO_PEDERSEN_G;
use tari_crypto::ristretto::{RistrettoPublicKey, RistrettoSecretKey};
use tari_crypto::tari_utilities::hex::Hex;

#[allow(non_snake_case)]
fn main() {
    // Create the secret key "1" . Ristretto uses little-endian encoding, so the number is written back to front
    let k = RistrettoSecretKey::from_hex(
        "0100000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    // Generate the public key, P = k.G = 1.G
    let pub_from_k = RistrettoPublicKey::from_secret_key(&k);
    let known_pub = RistrettoPublicKey::from_hex(
        "e2f2ae0a6abc4e71a884a961c500515f58e30b6aa582dd8db6a65945e08d2d76",
    )
    .unwrap();
    // Compare it to the known value
    assert_eq!(pub_from_k, known_pub);
    println!("Ok, P = k.G");
    // But k = 1, so P = 1.G === G
    assert_eq!(pub_from_k.point(), RISTRETTO_PEDERSEN_G);
    println!("Ok, P = 1.G = G");
}

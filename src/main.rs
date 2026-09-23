use rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer};

fn main() {
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(vec![0u8; 16]));
    let r = rustls::crypto::aws_lc_rs::sign::any_supported_type(&key);
    println!("parsed ok: {}", r.is_ok());
}

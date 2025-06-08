use std::fs;
use std::path::Path;
use rcgen::CertifiedKey;

fn main() {
    // `.der` is binary file for the certificate
    if !Path::new("cert.der").exists(){
        let CertifiedKey{cert, key_pair} = rcgen::generate_simple_self_signed(vec!["localhost".into()]).expect("Failed to create cert!!");
        fs::write("key.der", key_pair.serialize_der()).expect("Failed to write key");
        fs::write("cert.der", cert.der()).expect("Failed to write cert");
    }
}
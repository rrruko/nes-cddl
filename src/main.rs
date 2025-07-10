use cddl_cat::validate_cbor_bytes;
use ciborium;
use std::fs;
use std::fs::File;

fn main() {
    let cddl_input = fs::read_to_string("nes.cddl").unwrap();
    let cbor_bytes = fs::read("nes1.cbor").unwrap();
    match validate_cbor_bytes("new_epoch_state", &cddl_input, &cbor_bytes) {
        Ok(()) => println!("valid!"),
        Err(e) => println!("invalid: {}", e)
    }
}

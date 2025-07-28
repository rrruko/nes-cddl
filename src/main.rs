use cddl::validate_cbor_from_slice;
use cddl::validator::Validator;
use ciborium;
use std::fs;
use std::fs::File;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let cddl_filepath = &args[1];
    let cbor_filepath = &args[2];
    let cddl_input = fs::read_to_string(cddl_filepath).unwrap();
    let cbor_bytes = fs::read(cbor_filepath).unwrap();
    match validate_cbor_from_slice(&cddl_input, &cbor_bytes, None) {
        Ok(()) => println!("valid!"),
        Err(e) => println!("invalid: {}", e)
    }
}

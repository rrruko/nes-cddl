use cddl::cddl_from_str;
use cddl::validator::Validator;
use cddl::validator::cbor::CBORValidator;
use ciborium;
use std::fs;
use std::fs::File;

fn main() {
    let input = fs::read_to_string("nes.cddl").unwrap();
    let cddl = cddl_from_str(&input, true).unwrap();
    let cbor_file = File::open("nes1.cbor").unwrap();
    let cbor = ciborium::de::from_reader(cbor_file).unwrap();
    let mut cv = CBORValidator::new(&cddl, cbor, None);
    let v = cv.validate();
    match v {
        Ok(()) => println!("valid!"),
        Err(e) => println!("invalid: {}", e)
    }
}

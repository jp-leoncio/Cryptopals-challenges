mod operations;
mod base64;
use base64::Base64;
use hex;

fn main() {
    let original = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let decoded_data = hex::decode(original).expect("Invalid hex string");
    let decoded_string = String::from_utf8(decoded_data).expect("Invalid UTF-8");

    let mut base64 = Base64::new();
    let encoded = base64.encode(&decoded_string).unwrap();

    println!("Original: {:?}", original);
    println!("Encoded: {:?}", decoded_string);
    println!("Decoded: {:?}", encoded);
}
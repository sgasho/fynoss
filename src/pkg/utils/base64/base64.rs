use base64::prelude::BASE64_STANDARD;
use base64::{Engine};
use std::error::Error;

pub fn decode_to_string(input: &str) -> Result<String, Box<dyn Error>> {
    let decoded_bytes = BASE64_STANDARD.decode(input.replace("\n", "").replace("\r", ""))?;
    let decoded = String::from_utf8(decoded_bytes)?;
    Ok(decoded)
}
use anyhow::Ok;
use base64::{
    engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD, prelude::*,
};

use crate::{get_reader, Base64Format};
pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let data = data.trim();

    let decoded_data = match format {
        Base64Format::Standard => STANDARD.decode(data)?,
        Base64Format::Urlsafe => URL_SAFE_NO_PAD.decode(data)?,
    };
    let decoded_data: String = String::from_utf8(decoded_data)?;
    print!("{}", decoded_data);
    Ok(())
}
pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    let encoded_data = match format {
        Base64Format::Standard => STANDARD.encode(data),
        Base64Format::Urlsafe => URL_SAFE_NO_PAD.encode(data),
    };

    print!("{:?}", encoded_data);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base64_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Urlsafe;
        //process_base64_decode(input,format).unwrap();
        assert!(process_base64_decode(input, format).is_ok())
    }
    #[test]
    fn test_base64_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_base64_encode(input, format).is_ok());
    }
}

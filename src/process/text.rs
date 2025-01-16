use std::{fs, io::Read};

use anyhow::Ok;

use crate::{get_reader, TextSignFormat};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, prelude::*};

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify1 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}
// struct Ed25519Signer {
//     key: [u8; 32],
// }
// struct Ed25519Verifier {
//     key: [u8; 32],
// }
pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;

    let sign = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read_to_string(key)?;
            let signer: Blake3 = Blake3 {
                key: key.as_bytes().try_into()?,
            };
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(&sign);
    println!("{:?}", signed);
    Ok(())
}
pub fn process_verify(input: &str, key: &str, sign: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let sign = URL_SAFE_NO_PAD.decode(sign)?;
    let verify = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read_to_string(key)?;
            let verifier: Blake3 = Blake3 {
                key: key.as_bytes().try_into()?,
            };
            verifier.verify(&mut reader, &sign)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    println!("{:?}", verify);
    Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;

        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}
impl TextVerify1 for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool> {
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::hash(&buf).as_bytes() == sign)
    }
}

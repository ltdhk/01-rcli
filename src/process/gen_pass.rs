use rand::{seq::SliceRandom, Rng};
use zxcvbn::zxcvbn;

use crate::GenPassOpts;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*,.<>?";
pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut pass = Vec::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        pass.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        pass.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if opts.numbers {
        chars.extend_from_slice(NUMBERS);
        pass.push(*NUMBERS.choose(&mut rng).expect("NUMBERS won't be empty"));
    }
    if opts.symbols {
        chars.extend_from_slice(SYMBOLS);
        pass.push(*SYMBOLS.choose(&mut rng).expect("SYMBOLS won't be empty"));
    }
    for _ in 0..(opts.length - pass.len() as u8) {
        let ids = rng.gen_range(0..chars.len());
        pass.push(chars[ids]);
    }
    pass.shuffle(&mut rng);
    let pass = String::from_utf8(pass)?;
    println!("{}", pass);

    let result = zxcvbn(&pass, &[]);
    eprint!("Password strength: {}", result.score());
    Ok(())
}

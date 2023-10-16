use std::env;

use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input character: '{0}'")]
    InvalidCharacter(char),

    #[error("mismatched input sizes")]
    MismatchedInputSizes,

    #[error("insufficient CLI arguments")]
    InsufficientCliArguments,
}

fn read_binary(b_str: &str) -> Result<Vec<bool>, Error> {
    let mut res = Vec::new();
    for ch in b_str.chars().rev() {
        res.push(match ch {
            '0' => false,
            '1' => true,
            _ => {
                return Err(Error::InvalidCharacter(ch));
            }
        });
    }
    Ok(res)
}

fn write_binary(b_bits: &[bool]) -> String {
    let mut res = "".to_string();
    for b in b_bits.iter().rev() {
        res.push(if *b { '1' } else { '0' });
    }
    res
}

fn add(a: &[bool], b: &[bool]) -> Result<Vec<bool>, Error> {
    let mut carry = false;
    let mut res = Vec::new();
    if a.len() != b.len() {
        return Err(Error::MismatchedInputSizes);
    }

    for i in 0..a.len() {
        let (output, new_carry) = match (a[i], b[i], carry) {
            (false, false, false) => (false, false),
            (false, false, true) => (true, false),
            (false, true, false) => (true, false),
            (false, true, true) => (false, true),
            (true, false, false) => (true, false),
            (true, false, true) => (false, true),
            (true, true, false) => (false, true),
            (true, true, true) => (true, true),
        };
        carry = new_carry;
        res.push(output);
    }
    Ok(res)
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Provide two arguments of equal length");
        return Err(Error::InsufficientCliArguments);
    }

    let result = add(&read_binary(&args[1])?, &read_binary(&args[2])?)?;

    println!("{}", write_binary(&result));

    Ok(())
}

use anyhow::{anyhow, Result};
use atoi::FromRadix10;

pub fn atoi_with_rest<I: FromRadix10>(text: &[u8]) -> Result<(I, &[u8])> {
    match I::from_radix_10(text) {
        (_, 0) => Err(anyhow!("invalid int")),
        (n, used) => Ok((n, &text[used..])),
    }
}

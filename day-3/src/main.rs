use std::{env, fs::File};

use anyhow::{Context as _, Result};
use memmap2::Mmap;
use util::atoi_with_rest;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input file arg")?;
    let file = File::open(filename)?;
    let input_all = unsafe { Mmap::map(&file)? };

    let mut input = &input_all[..];
    let mut enabled = true;
    let mut ans1 = 0;
    let mut ans2 = 0;

    while input.len() >= 4 {
        if &input[..4] == b"mul(" {
            if let Some((a, b, rest)) = atoi_with_rest::<u64>(&input[4..]).and_then(|(a, rest)| {
                let (b, rest) = atoi_with_rest::<u64>(&rest[1..])?;
                if rest.is_empty() || rest[0] != b')' {
                    None
                } else {
                    Some((a, b, rest))
                }
            }) {
                let res = a * b;
                ans1 += res;
                if enabled {
                    ans2 += res;
                }
                input = &rest[1..];
            } else {
                input = &input[4..];
            }
        } else if &input[..4] == b"do()" {
            enabled = true;
            input = &input[4..];
        } else if input.len() >= 7 && &input[..7] == b"don't()" {
            enabled = false;
            input = &input[7..];
        } else {
            input = &input[1..];
        }
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

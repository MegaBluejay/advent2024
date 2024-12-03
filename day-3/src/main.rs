use std::{env, fs::File};

use anyhow::{Context, Result};
use memmap2::Mmap;
use regex::bytes::Regex;
use util::atoi_with_rest;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input file arg")?;
    let file = File::open(filename)?;
    let input = unsafe { Mmap::map(&file)? };

    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)")?;

    let mut ans1: u64 = 0;
    let mut ans2: u64 = 0;
    let mut enabled = true;

    for m in re.find_iter(&input) {
        match m.as_bytes() {
            b"do()" => enabled = true,
            b"don't()" => enabled = false,
            s => {
                let (a, s) = atoi_with_rest::<u64>(&s[4..])?;
                let (b, _) = atoi_with_rest::<u64>(&s[1..])?;
                let res = a * b;
                ans1 += res;
                if enabled {
                    ans2 += res;
                }
            }
        }
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

use std::{env, fs::File};

use anyhow::{Context, Result};
use atoi::atoi;
use memmap2::Mmap;
use regex::bytes::Regex;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input file arg")?;
    let file = File::open(filename)?;
    let input = unsafe { Mmap::map(&file)? };

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")?;

    let mut ans1: u64 = 0;
    let mut ans2: u64 = 0;
    let mut enabled = true;

    for m in re.captures_iter(&input) {
        match &m[0] {
            b"do()" => enabled = true,
            b"don't()" => enabled = false,
            _ => {
                let a: u64 = atoi(&m[1]).context("invalid int")?;
                let b: u64 = atoi(&m[2]).context("invalid int")?;
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

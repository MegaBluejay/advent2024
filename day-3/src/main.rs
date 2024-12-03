use std::{env, fs::File};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use memmap2::Mmap;
use util::atoi_with_rest;

fn part1(s: &[u8]) -> u64 {
    let mut ans = 0;

    for i in s.windows(4).positions(|q| q == b"mul(") {
        if let Ok((a, b)) = atoi_with_rest::<u64>(&s[i + 4..]).and_then(|(a, rest)| {
            let (b, rest) = atoi_with_rest::<u64>(&rest[1..])?;
            if rest.is_empty() || rest[0] != ')' as u8 {
                return Err(anyhow!("no )"));
            }
            Ok((a, b))
        }) {
            ans += a * b;
        }
    }

    ans
}

fn part2(mut s: &[u8]) -> u64 {
    let mut ans = 0;

    loop {
        let end = s
            .windows(7)
            .position(|q| q == b"don't()")
            .unwrap_or(s.len());
        ans += part1(&s[..end]);
        s = &s[end + 7..];
        let start = s.windows(4).position(|q| q == b"do()");
        if let Some(start) = start {
            s = &s[start + 4..];
        } else {
            break;
        }
    }

    ans
}

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input file arg")?;
    let file = File::open(filename)?;
    let input = unsafe { Mmap::map(&file)? };

    let ans1 = part1(&input);
    let ans2 = part2(&input);

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

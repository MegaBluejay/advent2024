use std::{env, fs::File};

use anyhow::{Context as _, Result};
use itertools::Itertools as _;
use memmap2::Mmap;
use util::atoi_with_rest;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input file arg")?;

    let file = File::open(filename)?;
    let input_all = unsafe { Mmap::map(&file)? };

    let mut ass: Vec<u64> = vec![];
    let mut bss: Vec<u64> = vec![];
    let mut input = &input_all[..];

    loop {
        if input.is_empty() || input[0] == '\n' as u8 {
            break;
        }
        let (a, rest) = atoi_with_rest(input)?;
        let (b, rest) = atoi_with_rest(&rest[3..])?;
        ass.push(a);
        bss.push(b);
        input = &rest[1..];
    }

    ass.sort();
    bss.sort();

    let ans1: u64 = ass.iter().zip(&bss).map(|(a, b)| a.abs_diff(*b)).sum();

    let mut j = 0;
    let mut ans2 = 0;
    for (count, a) in ass.into_iter().dedup_with_count() {
        let mut c = 0;
        while j < bss.len() && bss[j] < a {
            j += 1;
        }
        while j < bss.len() && bss[j] == a {
            j += 1;
            c += 1;
        }
        ans2 += (count as u64) * a * c;
        if j == bss.len() {
            break;
        }
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

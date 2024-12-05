use std::{env, fs::File};

use anyhow::{Context as _, Result};
use memmap2::Mmap;
use util::atoi_with_rest;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input filename")?;
    let file = File::open(filename)?;
    let input_all = unsafe { Mmap::map(&file)? };

    let mut input = &input_all[..];

    let mut rules = vec![vec![]; 10_000];
    while input[0] != b'\n' {
        let (a, rest) = atoi_with_rest::<u16>(input).context("invalid int")?;
        let (b, rest) = atoi_with_rest::<u16>(&rest[1..]).context("invalid int")?;
        rules[a as usize].push(b);
        input = &rest[1..];
    }

    input = &input[1..];

    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut original = vec![];
    let mut incoming = vec![100_000; 10_000];
    let mut stack = Vec::new();
    let mut out = Vec::new();

    while !input.is_empty() {
        original.clear();
        stack.clear();
        out.clear();
        loop {
            let (x, rest) = atoi_with_rest::<u16>(input).context("invalid int")?;
            original.push(x);
            incoming[x as usize] = 0;
            input = &rest[1..];
            if rest[0] == b'\n' {
                break;
            }
        }
        for a in &original {
            for b in &rules[*a as usize] {
                incoming[*b as usize] += 1;
            }
        }
        for a in &original {
            if incoming[*a as usize] == 0 {
                stack.push(*a);
            }
        }
        while let Some(a) = stack.pop() {
            out.push(a);
            for b in &rules[a as usize] {
                incoming[*b as usize] -= 1;
                if incoming[*b as usize] == 0 {
                    stack.push(*b);
                }
            }
        }

        let mid = out[(out.len() - 1) / 2] as u64;
        if original == out {
            ans1 += mid;
        } else {
            ans2 += mid;
        }

        for a in &original {
            incoming[*a as usize] = 100_000;
        }
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

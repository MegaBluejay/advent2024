use std::{cmp::Ordering, env, fs::File};

use ahash::{HashSet, HashSetExt as _};
use anyhow::{Context as _, Result};
use memmap2::Mmap;
use util::atoi_with_rest;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input filename")?;
    let file = File::open(filename)?;
    let input_all = unsafe { Mmap::map(&file)? };

    let mut input = &input_all[..];

    let mut rules = HashSet::new();
    while input[0] != b'\n' {
        let (a, rest) = atoi_with_rest::<u16>(input).context("invalid int")?;
        let (b, rest) = atoi_with_rest::<u16>(&rest[1..]).context("invalid int")?;
        rules.insert((a, b));
        input = &rest[1..];
    }

    input = &input[1..];

    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut update = vec![];

    while !input.is_empty() {
        update.clear();
        loop {
            let (x, rest) = atoi_with_rest::<u16>(input).context("invalid int")?;
            update.push(x);
            input = &rest[1..];
            if rest[0] == b'\n' {
                break;
            }
        }
        let sorted = update.is_sorted_by(|&a, &b| rules.contains(&(a, b)));
        let mid = update.len() / 2;
        if sorted {
            ans1 += update[mid];
        } else {
            let (_, &mut val, _) = update.select_nth_unstable_by(mid, |&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            ans2 += val;
        }
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

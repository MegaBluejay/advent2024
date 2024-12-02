use std::{env, fs::File, iter::once};

use anyhow::{Context, Result};
use itertools::Itertools;
use memmap2::Mmap;
use util::atoi_with_rest;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input file arg")?;

    let file = File::open(filename)?;
    let input_all = unsafe { Mmap::map(&file)? };

    let mut dists: Vec<i64> = Vec::new();
    let mut input = &input_all[..];
    let mut ans1 = 0;
    let mut ans2 = 0;

    loop {
        if input.is_empty() || input[0] == '\n' as u8 {
            break;
        }

        let mut prev: Option<i64> = None;
        loop {
            let (level, rest) = atoi_with_rest(input)?;
            if let Some(prev) = prev {
                dists.push(level - prev);
            }
            prev = Some(level);
            input = &rest[1..];
            if rest[0] == '\n' as u8 {
                break;
            }
        }

        #[derive(PartialEq, Eq, Clone, Copy)]
        enum Comp {
            Asc,
            Desc,
            Invalid,
        }

        impl From<i64> for Comp {
            fn from(value: i64) -> Self {
                if (1..=3).contains(&value) {
                    Self::Asc
                } else if (-3..=-1).contains(&value) {
                    Self::Desc
                } else {
                    Self::Invalid
                }
            }
        }

        fn check<I: IntoIterator<Item = i64>>(it: I) -> bool {
            matches!(
                it.into_iter().map(Comp::from).all_equal_value(),
                Ok(Comp::Asc | Comp::Desc) | Err(None)
            )
        }

        let safe1 = check(dists.iter().copied());

        fn merge<'a>(dists: &'a [i64], i: usize) -> impl Iterator<Item = i64> + 'a {
            dists[..i]
                .iter()
                .copied()
                .chain(once(dists[i] + dists[i + 1]))
                .chain(dists[i + 2..].iter().copied())
        }

        fn get_safe2(dists: &[i64]) -> bool {
            let first = Comp::from(dists[0]);
            let rm_first = check(dists[1..].iter().copied());
            let Some((i, _)) = dists
                .iter()
                .copied()
                .map(Comp::from)
                .find_position(|c| *c != first)
            else {
                return rm_first;
            };
            let before = check(merge(dists, i - 1));
            let after = if i == dists.len() - 1 {
                check(dists[..dists.len() - 1].iter().copied())
            } else {
                check(merge(dists, i))
            };
            rm_first || before || after
        }

        if safe1 {
            ans1 += 1;
        }

        if safe1 || get_safe2(&dists) {
            ans2 += 1;
        }

        dists.clear();
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

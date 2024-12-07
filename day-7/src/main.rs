use std::{env, fs::File};

use anyhow::{Context as _, Result};
use memmap2::Mmap;
use util::atoi_with_rest;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input filename")?;
    let file = File::open(filename)?;
    let input_all = unsafe { Mmap::map(&file)? };

    let mut input = &input_all[..];
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut line = vec![];
    let mut stack = vec![];

    while !input.is_empty() {
        line.clear();
        let (res, rest) = atoi_with_rest::<usize>(input).context("invalid int")?;
        input = &rest[2..];
        loop {
            let (a, rest) = atoi_with_rest::<usize>(input).context("invalid int")?;
            line.push(a);
            input = &rest[1..];
            if rest[0] == b'\n' {
                break;
            }
        }
        let n = line.len();

        stack.push((n - 1, res, false));
        let mut ok1 = false;
        let mut ok2 = false;
        while let Some((i, q, has_concat)) = stack.pop() {
            let a = line[i];

            if i == 0 {
                if a == q {
                    ok2 = true;
                    if !has_concat {
                        ok1 = true;
                    }
                }
                continue;
            }

            if q >= a {
                stack.push((i - 1, q - a, has_concat));
            }

            if q % a == 0 {
                stack.push((i - 1, q / a, has_concat));
            }

            let pow = 10usize.pow(a.ilog10() + 1);
            if q % pow == a {
                stack.push((i - 1, q / pow, true));
            }
        }

        if ok1 {
            ans1 += res;
        }

        if ok2 {
            ans2 += res;
        }
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

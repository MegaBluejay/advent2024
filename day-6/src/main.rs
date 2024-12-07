use std::{env, fs::File, iter};

use ahash::{HashSet, HashSetExt as _};
use anyhow::{anyhow, Context as _, Result};
use itertools::Itertools as _;
use memmap2::Mmap;

type Idx = usize;

fn next(maxi: Idx, maxj: Idx, i: Idx, j: Idx, dir: u8) -> Option<(Idx, Idx)> {
    Some(match dir {
        0 => {
            if i == 0 {
                return None;
            }
            (i - 1, j)
        }
        1 => {
            if j == maxj {
                return None;
            }
            (i, j + 1)
        }
        2 => {
            if i == maxi {
                return None;
            }
            (i + 1, j)
        }
        3 => {
            if j == 0 {
                return None;
            }
            (i, j - 1)
        }
        _ => unreachable!(),
    })
}

fn path<'a>(
    map: &'a HashSet<(Idx, Idx)>,
    maxi: Idx,
    maxj: Idx,
    extra: Option<(Idx, Idx)>,
    i: Idx,
    j: Idx,
    dir: u8,
) -> impl Iterator<Item = (Idx, Idx, u8)> + 'a {
    iter::successors(Some((i, j, dir)), move |&(i, j, mut dir)| loop {
        let (ni, nj) = next(maxi, maxj, i, j, dir)?;
        if map.contains(&(ni, nj)) || extra == Some((ni, nj)) {
            dir = (dir + 1) % 4;
        } else {
            return Some((ni, nj, dir));
        }
    })
}

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input filename")?;
    let file = File::open(filename)?;
    let input = unsafe { Mmap::map(&file)? };

    let mut pos = None;
    let mut maxi = 0;
    let mut maxj = 0;
    let mut map = HashSet::new();
    for (i, line) in input
        .strip_suffix(b"\n")
        .unwrap_or(&input)
        .split(|&c| c == b'\n')
        .enumerate()
    {
        maxi = i;
        for (j, &c) in line.iter().enumerate() {
            maxj = j;
            match c {
                b'#' => {
                    map.insert((i, j));
                }
                b'.' => {}
                b'^' => {
                    if let Some((pi, pj)) = pos {
                        return Err(anyhow!(
                            "multiple starting positions: {pi},{pj} and {i},{j}"
                        ));
                    }
                    pos = Some((i, j));
                }
                _ => return Err(anyhow!("invalid char at {i},{j}")),
            }
        }
    }
    let (starti, startj) = pos.ok_or_else(|| anyhow!("no starting position"))?;

    let mut seen = HashSet::new();
    seen.insert((starti, startj));
    let mut loop_seen = HashSet::new();
    let mut ans2 = 0;

    for ((pi, pj, pdir), (i, j, _dir)) in
        path(&map, maxi, maxj, None, starti, startj, 0).tuple_windows()
    {
        if !seen.insert((i, j)) {
            continue;
        }

        loop_seen.clear();
        for (li, lj, dir) in path(&map, maxi, maxj, Some((i, j)), pi, pj, pdir) {
            if !loop_seen.insert((li, lj, dir)) {
                ans2 += 1;
                break;
            }
        }
    }

    let ans1 = seen.len();

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

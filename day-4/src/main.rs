use std::{env, fs::File, mem};

use anyhow::{Context as _, Result};
use memmap2::Mmap;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("no input file arg")?;
    let file = File::open(filename)?;
    let input = unsafe { Mmap::map(&file)? };

    let ncols = input
        .iter()
        .copied()
        .position(|c| c == b'\n')
        .context("no newline")
        .unwrap();

    #[derive(Clone, Copy)]
    struct State1 {
        index: u8,
        forward: bool,
        backward: bool,
    }

    fn update1(prev: Option<State1>, cur: Option<u8>, ans: &mut u64) -> Option<State1> {
        let index = cur?;
        let forward = index == 0 || prev.map_or(false, |p| p.forward && p.index == index - 1);
        let backward = index == 3 || prev.map_or(false, |p| p.backward && p.index == index + 1);
        if forward && index == 3 {
            *ans += 1;
        }
        if backward && index == 0 {
            *ans += 1;
        }
        Some(State1 {
            index,
            forward,
            backward,
        })
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    enum MS {
        M,
        S,
    }

    fn parse_ms(char: u8) -> Option<MS> {
        match char {
            b'M' => Some(MS::M),
            b'S' => Some(MS::S),
            _ => None,
        }
    }

    #[derive(Clone, Copy, Default)]
    struct State2 {
        left: Option<MS>,
        mid: Option<(MS, MS)>,
        right: Option<MS>,
    }

    fn update2(state: &mut State2, left: Option<MS>, mid: bool, right: Option<MS>) -> bool {
        let x = match (state.mid, left, right) {
            (Some((ltop, rtop)), Some(lbot), Some(rbot)) if ltop != rbot && rtop != lbot => true,
            _ => false,
        };
        *state = State2 {
            left,
            mid: state.left.zip(state.right).filter(|_| mid),
            right,
        };
        x
    }

    let mut prev_row_state1 = vec![[None; 3]; ncols];
    let mut row_state1 = prev_row_state1.clone();
    let mut ans1 = 0;

    let mut row_state2 = vec![Default::default(); ncols - 2];
    let mut ans2 = 0;

    for line in input.chunks(ncols + 1) {
        let mut col_state = None;
        for (i, c) in line[..ncols].iter().copied().enumerate() {
            let cur = match c {
                b'X' => Some(0),
                b'M' => Some(1),
                b'A' => Some(2),
                b'S' => Some(3),
                _ => None,
            };
            for (j, prev) in prev_row_state1[i].iter().copied().enumerate() {
                let new = update1(prev, cur, &mut ans1);
                if i + 1 >= j && i + 1 < row_state1.len() + j {
                    row_state1[i + 1 - j][j] = new;
                }
            }
            col_state = update1(col_state, cur, &mut ans1);

            if i < ncols - 2 {
                let left = parse_ms(c);
                let mid = line[i + 1] == b'A';
                let right = parse_ms(line[i + 2]);
                if update2(&mut row_state2[i], left, mid, right) {
                    ans2 += 1;
                }
            }
        }
        mem::swap(&mut prev_row_state1, &mut row_state1);
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

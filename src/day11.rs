use rustc_hash::FxHashSet;
use std::num::NonZeroUsize;

/*const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
*/

const INPUT: &str = include_str!("inputs/11.txt");

pub fn part1() -> usize {
    let width =
        NonZeroUsize::new(INPUT.as_bytes().iter().position(|&b| b == b'\n').unwrap() + 1).unwrap();
    let mut populated_rows = FxHashSet::default();
    let mut populated_cols = FxHashSet::default();
    let galaxies: Vec<_> = INPUT
        .bytes()
        .enumerate()
        .filter(|&(_, b)| b == b'#')
        .map(|(i, _)| (i % width, i / width))
        .collect();
    for &(x, y) in galaxies.iter() {
        populated_rows.insert(y);
        populated_cols.insert(x);
    }
    let mut sum = 0;
    for (i, &from) in galaxies.iter().enumerate() {
        for (j, &to) in galaxies.iter().enumerate().skip(i) {
            if i == j {
                continue;
            }
            let mut path = 0;
            let (start_x, end_x) = if from.0 < to.0 {
                (from.0, to.0)
            } else {
                (to.0, from.0)
            };
            for x in start_x..end_x {
                if !populated_cols.contains(&x) {
                    path += 2
                } else {
                    path += 1
                }
            }

            let (start_y, end_y) = if from.1 < to.1 {
                (from.1, to.1)
            } else {
                (to.1, from.1)
            };
            for y in start_y..end_y {
                if !populated_rows.contains(&y) {
                    path += 2
                } else {
                    path += 1
                }
            }
            sum += path;
        }
    }
    sum
}
pub fn part2() -> usize {
    let width =
        NonZeroUsize::new(INPUT.as_bytes().iter().position(|&b| b == b'\n').unwrap() + 1).unwrap();
    let mut populated_rows = FxHashSet::default();
    let mut populated_cols = FxHashSet::default();
    let galaxies: Vec<_> = INPUT
        .bytes()
        .enumerate()
        .filter(|&(_, b)| b == b'#')
        .map(|(i, _)| (i % width, i / width))
        .collect();
    for &(x, y) in galaxies.iter() {
        populated_rows.insert(y);
        populated_cols.insert(x);
    }
    let mut sum = 0;
    for (i, &from) in galaxies.iter().enumerate() {
        for (j, &to) in galaxies.iter().enumerate().skip(i) {
            if i == j {
                continue;
            }
            let mut path = 0;
            let (start_x, end_x) = if from.0 < to.0 {
                (from.0, to.0)
            } else {
                (to.0, from.0)
            };
            for x in start_x..end_x {
                if !populated_cols.contains(&x) {
                    path += 1000000
                } else {
                    path += 1
                }
            }

            let (start_y, end_y) = if from.1 < to.1 {
                (from.1, to.1)
            } else {
                (to.1, from.1)
            };
            for y in start_y..end_y {
                if !populated_rows.contains(&y) {
                    path += 1000000
                } else {
                    path += 1
                }
            }
            sum += path;
        }
    }
    sum
}

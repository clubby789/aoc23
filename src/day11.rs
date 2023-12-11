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

struct Space {
    galaxies: Vec<(usize, usize)>,
    populated_rows: FxHashSet<usize>,
    populated_cols: FxHashSet<usize>,
}

impl Space {
    pub fn path_count<const GROWTH: usize>(&self) -> usize {
        let mut sum = 0;
        for (i, &from) in self.galaxies.iter().enumerate() {
            for (j, &to) in self.galaxies.iter().enumerate().skip(i) {
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
                    if !self.populated_cols.contains(&x) {
                        path += GROWTH
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
                    if !self.populated_rows.contains(&y) {
                        path += GROWTH
                    } else {
                        path += 1
                    }
                }
                sum += path;
            }
        }
        sum
    }
}

fn parse_input(src: &str) -> Space {
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
    Space {
        galaxies,
        populated_cols,
        populated_rows,
    }
}

pub fn part1() -> usize {
    let space = parse_input(INPUT);
    space.path_count::<2>()
}

pub fn part2() -> usize {
    let space = parse_input(INPUT);
    space.path_count::<1000000>()
}

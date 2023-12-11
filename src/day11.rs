use std::num::NonZeroUsize;

const INPUT: &str = include_str!("inputs/11.txt");

struct Space {
    galaxies: Vec<(usize, usize)>,
    populated_rows: Vec<bool>,
    populated_cols: Vec<bool>,
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
                    if !self.populated_cols[x] {
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
                    if !self.populated_rows[y] {
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
        NonZeroUsize::new(src.as_bytes().iter().position(|&b| b == b'\n').unwrap() + 1).unwrap();
    let galaxies: Vec<_> = src
        .bytes()
        .enumerate()
        .filter(|&(_, b)| b == b'#')
        .map(|(i, _)| (i % width, i / width))
        .collect();
    let height = galaxies.last().unwrap().1;
    let mut populated_rows = vec![false; height + 1];
    let mut populated_cols = vec![false; width.get()];
    for &(x, y) in galaxies.iter() {
        populated_rows[y] = true;
        populated_cols[x] = true;
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

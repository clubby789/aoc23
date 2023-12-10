use rustc_hash::FxHashSet;
use std::{cell::OnceCell, num::NonZeroUsize};

const INPUT: &str = include_str!("inputs/10.txt");

/*
const INPUT: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
............"#;
*/

struct Grid<'a> {
    src: &'a [u8],
    width: NonZeroUsize,
    start_pipe: OnceCell<u8>,
}

impl<'a> Grid<'a> {
    pub fn new(src: &'a str) -> Self {
        let src = src.as_bytes();
        let width = src
            .iter()
            .position(|&b| b == b'\n')
            .and_then(|p| NonZeroUsize::new(p + 1))
            .unwrap();
        Self {
            src,
            width,
            start_pipe: OnceCell::new(),
        }
    }

    pub fn pipes(&self) -> Pipes<'_> {
        let start = self.src.iter().position(|&b| b == b'S').unwrap();
        let (x, y) = (start % self.width, start / self.width);
        Pipes {
            grid: self,
            pos: (x, y),
            last_pos: None,
            done: false,
        }
    }

    fn get_orig(&self, (x, y): (usize, usize)) -> u8 {
        debug_assert!(x < self.width.get());
        self.src[self.width.get() * y + x]
    }

    pub fn get(&self, (x, y): (usize, usize)) -> u8 {
        let v = self.get_orig((x, y));
        match (v, self.start_pipe.get()) {
            (b'S', Some(&kind)) => kind,
            _ => v,
        }
    }

    pub fn height(&self) -> usize {
        self.src.len() / self.width
    }
}

struct Pipes<'a> {
    grid: &'a Grid<'a>,
    last_pos: Option<(usize, usize)>,
    pos: (usize, usize),
    done: bool,
}

impl<'a> Iterator for Pipes<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.last_pos.is_some() && self.grid.get_orig(self.pos) == b'S' {
            self.done = true;
            return Some(self.pos);
        }
        let (last_pos, pos) = self.last_pos.map(|lp| (lp, self.pos)).unwrap_or_else(|| {
            // surrounding cells, clockwise starting from
            // directly above
            let surrounding = [(0, -1), (1, 0), (0, 1), (-1, 0)].map(|(dx, dy): (isize, isize)| {
                let p = (
                    self.pos.0.checked_add_signed(dx)?,
                    self.pos.1.checked_add_signed(dy)?,
                );
                Some((p, self.grid.get(p)))
            });

            let np1 = if let Some((np, b'|' | b'7' | b'F')) = surrounding[0] {
                Some(np)
            } else {
                None
            };
            let np2 = if let Some((np, b'-' | b'J' | b'7')) = surrounding[1] {
                Some(np)
            } else {
                None
            };
            let np3 = if let Some((np, b'|' | b'L' | b'J')) = surrounding[2] {
                Some(np)
            } else {
                None
            };
            let np4 = if let Some((np, b'-' | b'F' | b'L')) = surrounding[3] {
                Some(np)
            } else {
                None
            };
            let pipekind = match (np1, np2, np3, np4) {
                (Some(_), _, Some(_), _) => b'|',
                (_, Some(_), _, Some(_)) => b'-',
                (Some(_), Some(_), ..) => b'L',
                (Some(_), _, _, Some(_)) => b'J',
                (_, _, Some(_), Some(_)) => b'7',
                (_, Some(_), Some(_), _) => b'F',
                _ => unreachable!(),
            };
            self.grid.start_pipe.set(pipekind).unwrap();
            let new_pos = np1.or(np2).or(np3).or(np4).unwrap();
            (self.pos, new_pos)
        });
        let current = self.grid.get(pos);
        let diff = (
            pos.0 as isize - last_pos.0 as isize,
            pos.1 as isize - last_pos.1 as isize,
        );
        let new_diff: (isize, isize) = match (current, diff) {
            // N S
            (b'|', (_, -1)) => (0, -1),
            // S N
            (b'|', (_, 1)) => (0, 1),
            // E W
            (b'-', (-1, _)) => (-1, 0),
            // W E
            (b'-', (1, _)) => (1, 0),
            // N E
            (b'L', (_, 1)) => (1, 0),
            // E N
            (b'L', (-1, _)) => (0, -1),
            // N W
            (b'J', (_, 1)) => (-1, 0),
            // W N
            (b'J', (1, _)) => (0, -1),
            // S W
            (b'7', (_, -1)) => (-1, 0),
            // W S
            (b'7', (1, _)) => (0, 1),
            // S E
            (b'F', (_, -1)) => (1, 0),
            // E S
            (b'F', (-1, _)) => (0, 1),
            _ => unreachable!("'{}', {diff:?}", current as char),
        };
        self.last_pos = Some(pos);
        self.pos = (
            pos.0.checked_add_signed(new_diff.0).unwrap(),
            pos.1.checked_add_signed(new_diff.1).unwrap(),
        );

        Some(pos)
    }
}

pub fn part1() -> usize {
    let grid = Grid::new(INPUT);
    (grid.pipes().count() + 1).div_ceil(2)
}


pub fn part2() -> usize {
    let grid = Grid::new(INPUT);
    let mut points: FxHashSet<_> = grid.pipes().collect();
    let mut count = 0;

    for y in (0..grid.height()) {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum State {
            Outside,
            Inside,
            // bool means was inside before the corner
            CornerF(bool),
            CornerL(bool),
        }
        let mut state = State::Outside;
        for x in 0..grid.width.get() - 1 {
            let cell = grid.get((x, y));
            if points.contains(&(x, y)) {
                let is_inside = state == State::Inside;
                state = match (cell, state) {
                    (b'|', State::Outside) => State::Inside,
                    (b'|', State::Inside) => State::Outside,
                    (b'L', _) => State::CornerL(is_inside),
                    (b'7', State::CornerF(true)) => State::Inside,
                    (b'7', State::CornerF(false)) => State::Outside,
                    (b'7', State::CornerL(true)) => State::Outside,
                    (b'7', State::CornerL(false)) => State::Inside,
                    (b'F', _) => State::CornerF(is_inside),
                    (b'J', State::CornerF(true)) => State::Outside,
                    (b'J', State::CornerF(false)) => State::Inside,
                    (b'J', State::CornerL(true)) => State::Inside,
                    (b'J', State::CornerL(false)) => State::Outside,
                    (b'-', _) => state,
                    _ => unreachable!("'{}', {:?}, {:?}", cell as char, state, (x, y)),
                };
            } else {
                if state == State::Inside {
                    count += 1;
                }
            }
        }
    }

    count
}

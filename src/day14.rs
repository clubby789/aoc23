use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
use std::fmt::{Debug, Formatter, Write};
use std::hash::{Hash, Hasher};

const INPUT: &str = include_str!("inputs/14.txt");

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum CellKind {
    Round = b'O',
    Cube = b'#',
    Empty = b'.',
}

impl Debug for CellKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(*self as u8 as char)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Grid {
    content: Vec<CellKind>,
    width: usize,
}

impl Grid {
    pub fn new(src: &str) -> Self {
        let width = src.bytes().position(|b| b == b'\n').unwrap();
        let content = src
            .bytes()
            .filter_map(|b| {
                Some(match b {
                    b'O' => CellKind::Round,
                    b'#' => CellKind::Cube,
                    b'.' => CellKind::Empty,
                    _ => return None,
                })
            })
            .collect();
        Self { content, width }
    }

    pub fn slide_up(&mut self) {
        for mut i in 0..self.content.len() {
            if self.content[i] != CellKind::Round {
                continue;
            }
            let mut cur = i;
            while let Some(above) = cur.checked_sub(self.width) {
                if self.content[above] == CellKind::Empty {
                    cur = above;
                } else {
                    break;
                }
            }
            self.content.swap(i, cur);
        }
    }

    pub fn slide_left(&mut self) {
        for mut i in 0..self.content.len() {
            if self.content[i] != CellKind::Round {
                continue;
            }
            let mut cur = i;
            for _ in 0..(i % self.width) {
                let left = cur - 1;
                if self.content[left] == CellKind::Empty {
                    cur = left;
                } else {
                    break;
                }
            }
            self.content.swap(i, cur);
        }
    }

    pub fn slide_down(&mut self) {
        for mut i in (0..self.content.len()).rev() {
            if self.content[i] != CellKind::Round {
                continue;
            }
            let mut cur = i;
            while cur + self.width < self.content.len() {
                let below = cur + self.width;
                if self.content[below] == CellKind::Empty {
                    cur = below;
                } else {
                    break;
                }
            }
            self.content.swap(i, cur);
        }
    }

    pub fn slide_right(&mut self) {
        for mut i in (0..self.content.len()).rev() {
            if self.content[i] != CellKind::Round {
                continue;
            }
            let mut cur = i;
            let amnt = self.width - cur % self.width;
            if amnt == 0 {
                continue;
            }
            for _ in 0..amnt - 1 {
                let right = cur + 1;
                if self.content[right] == CellKind::Empty {
                    cur = right;
                } else {
                    break;
                }
            }
            self.content.swap(i, cur);
        }
    }

    pub fn weight(&self) -> usize {
        self.content
            .chunks(self.width)
            .rev()
            .enumerate()
            .map(|(i, chunk)| {
                (i + 1)
                    * chunk
                        .iter()
                        .filter(|c| matches!(c, CellKind::Round))
                        .count()
            })
            .sum()
    }
}

pub fn part1() -> usize {
    let mut grid = Grid::new(INPUT);
    grid.slide_up();
    grid.weight()
}

impl Hash for Grid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.weight())
    }
}

pub fn part2() -> usize {
    let mut grid = Grid::new(INPUT);
    let mut cache = FxHashMap::default();

    let mut i = 0;
    while i < 1000000000 {
        match cache.entry(grid.clone()) {
            Entry::Occupied(occ) => {
                let cycle_len = i - *occ.get();
                while i + cycle_len < 1000000000 {
                    i += cycle_len;
                }
            }
            Entry::Vacant(mut vac) => {
                vac.insert(i);
            }
        }
        grid.slide_up();
        grid.slide_left();
        grid.slide_down();
        grid.slide_right();
        i += 1;
    }
    grid.weight()
}

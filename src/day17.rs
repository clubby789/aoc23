use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

const INPUT: &str = include_str!("inputs/17.txt");

struct Grid<'a> {
    src: &'a [u8],
    // width including newline
    width: usize,
    height: usize,
}

// x, y, grid, straight line length, current direction
#[derive(Copy, Clone)]
struct Pos<'a, PL>(usize, usize, &'a Grid<'a>, PL, Direction);

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum PathLengthP1 {
    One,
    Two,
    Three,
}

impl PathLength for PathLengthP1 {
    fn next(self) -> Option<Self> {
        use PathLengthP1::*;
        Some(match self {
            One => Two,
            Two => Three,
            Three => return None,
        })
    }
    fn one() -> Self {
        Self::One
    }
    fn val(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum PathLengthP2 {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl PathLength for PathLengthP2 {
    fn next(self) -> Option<Self> {
        use PathLengthP2::*;
        Some(match self {
            One => Two,
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Six,
            Six => Seven,
            Seven => Eight,
            Eight => Nine,
            Nine => Ten,
            Ten => return None,
        })
    }
    fn one() -> Self {
        Self::One
    }
    fn val(&self) -> u8 {
        *self as u8
    }
    fn can_turn(&self) -> bool {
        *self >= PathLengthP2::Four
    }
}

trait PathLength: PartialEq + Eq + Sized + Copy {
    fn next(self) -> Option<Self>;
    fn one() -> Self;
    fn val(&self) -> u8;
    fn can_turn(&self) -> bool {
        true
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
#[repr(u8)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl<'a, PL: PathLength> Pos<'a, PL> {
    pub fn in_direction(&self, direction: Direction) -> Option<(usize, usize)> {
        let (x, y) = match direction {
            Direction::North => (self.0, self.1.checked_sub(1)?),
            Direction::East => ((self.0 + 1 < self.2.width).then_some(self.0 + 1)?, self.1),
            Direction::South => (self.0, (self.1 + 1 < self.2.height).then_some(self.1 + 1)?),
            Direction::West => (self.0.checked_sub(1)?, self.1),
        };
        Some((x, y))
    }

    pub fn successors(&self) -> Vec<(Self, usize)> {
        if (self.0, self.1) == (0, 0) {
            let p1 = (0, 1);
            let p2 = (1, 0);
            let c1 = self.2.get(p1.0, p1.1).unwrap();
            let c2 = self.2.get(p2.0, p2.1).unwrap();
            return vec![
                (Pos(p1.0, p1.1, self.2, PL::one(), Direction::South), c1),
                (Pos(p2.0, p2.1, self.2, PL::one(), Direction::East), c2),
            ];
        }
        let mut succ = Vec::with_capacity(3);
        if let Some(pathlen) = self.3.next() {
            if let Some((nx, ny)) = self.in_direction(self.4) {
                let cost = self.2.get(nx, ny).unwrap();
                succ.push((Pos(nx, ny, self.2, pathlen, self.4), cost))
            }
        }
        if self.3.can_turn() {
            let (left, right) = match self.4 {
                Direction::North => (Direction::West, Direction::East),
                Direction::East => (Direction::North, Direction::South),
                Direction::South => (Direction::East, Direction::West),
                Direction::West => (Direction::South, Direction::North),
            };
            let mappit = |(nx, ny)| {
                let cost = self.2.get(nx, ny).unwrap();
                (nx, ny, cost)
            };

            if let Some((nx, ny, cost)) = self.in_direction(left).map(mappit) {
                succ.push((Pos(nx, ny, self.2, PL::one(), left), cost));
            }
            if let Some((nx, ny, cost)) = self.in_direction(right).map(mappit) {
                succ.push((Pos(nx, ny, self.2, PL::one(), right), cost));
            }
        }
        succ
    }
}

impl<'a, PL: PathLength> PartialEq for Pos<'a, PL> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.3 == other.3 && self.4 == other.4
    }
}

impl<'a, PL: PathLength> Eq for Pos<'a, PL> {}

impl<'a, PL: PathLength> Hash for Pos<'a, PL> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0);
        state.write_usize(self.1);
        state.write_u8(self.3.val());
        state.write_u8(self.4 as u8);
    }
}

impl<'a, PL> Debug for Pos<'a, PL> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pos").field(&self.0).field(&self.1).finish()
    }
}

impl<'a> Grid<'a> {
    pub fn new(src: &'a str) -> Self {
        let width = src.bytes().position(|b| b == b'\n').unwrap();
        Self {
            src: src.as_bytes(),
            width,
            height: src.len().div_ceil(width + 1),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<usize> {
        if let Some(val) = self.src.get(y * (self.width + 1) + x) {
            debug_assert!((*val).is_ascii_digit());
            Some((*val & 0b1111) as usize)
        } else {
            None
        }
    }
}
pub fn part1() -> usize {
    let grid = Grid::new(INPUT);
    let start = Pos(0, 0, &grid, PathLengthP1::One, Direction::South);
    let result = pathfinding::prelude::dijkstra(
        &start,
        |p| p.successors(),
        |&Pos(x, y, ..)| x == grid.width - 1 && y == grid.height - 1,
    )
    .unwrap();
    result.1
}

pub fn part2() -> usize {
    let grid = Grid::new(INPUT);
    let start = Pos(0, 0, &grid, PathLengthP2::One, Direction::East);
    let result = pathfinding::prelude::dijkstra(
        &start,
        |p| p.successors(),
        |&Pos(x, y, _, len, _)| x == grid.width - 1 && y == grid.height - 1 && len.can_turn(),
    )
    .unwrap();
    result.1
}

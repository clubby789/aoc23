use rustc_hash::FxHashMap;
use std::cell::Cell;
use std::num::{NonZeroU32, NonZeroUsize};

const INPUT: &str = include_str!("inputs/3.txt");

const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn number_width(n: NonZeroU32) -> NonZeroU32 {
    NonZeroU32::new(n.ilog10() + 1).unwrap()
}

pub fn part1() -> usize {
    let mut sum = 0;
    let mut pos = Cell::new(0);
    let grid = INPUT.as_bytes();
    // TODO: find this while walking
    let width = INPUT.find('\n').unwrap();
    // parse this number, returning it if it was adjacent to a symbol

    'outer: loop {
        // advance until digits or end of file
        loop {
            if let Some(cur) = grid.get(pos.get()) {
                if cur.is_ascii_digit() {
                    break;
                } else {
                    pos.set(pos.get() + 1)
                }
            } else {
                break 'outer;
            }
        }
        // parse the current number and advance past it
        // return 0 if no adjacent symbols

        let on_left_edge = || pos.get() == 0 || matches!(grid.get(pos.get() - 1), Some(b'\n')|None);

        let on_right_edge = || matches!(grid.get(pos.get() + 1), Some(b'\n')|None);

        let on_top = || pos.get() < width;

        let on_bottom = || grid.len() - pos.get() <= width;

        // closures to check if there is a symbol in the given position
        // minus and plus one where necessary to bypass newlines
        let up_left = || {
            if on_left_edge() || on_top() {
                false
            } else {
                grid[pos.get() - 1 - width - 1] != b'.'
            }
        };

        let up = || {
            if on_top() {
                false
            } else {
                grid[pos.get() - width - 1] != b'.'
            }
        };

        let up_right = || {
            if on_right_edge() || on_top() {
                false
            } else {
                grid[pos.get() + 1 - width - 1] != b'.'
            }
        };

        let left = || {
            if on_left_edge() {
                false
            } else {
                grid[pos.get() - 1] != b'.'
            }
        };

        let right = || {
            if on_right_edge() {
                false
            } else {
                grid[pos.get() + 1] != b'.'
            }
        };

        let down_left = || {
            if on_left_edge() || on_bottom() {
                false
            } else {
                grid[pos.get() - 1 + width + 1] != b'.'
            }
        };

        let down = || {
            if on_bottom() {
                false
            } else {
                grid[pos.get() + width + 1] != b'.'
            }
        };

        let down_right = || {
            if on_right_edge() || on_bottom() {
                false
            } else {
                grid[pos.get() + 1 + width + 1] != b'.'
            }
        };

        let n = {
            let mut adj_symbol = false;
            let mut n = (grid[pos.get()] - b'0') as usize;
            adj_symbol |= left() || up_left() || down_left() || up() || down();
            while pos.get() < grid.len() - 1 && grid[pos.get() + 1].is_ascii_digit() {
                n = n * 10 + (grid[pos.get() + 1] - b'0') as usize;
                pos.set(pos.get() + 1);
                adj_symbol |= up()|| down();
            }
            adj_symbol |= right() || up_right() || down_right();
            pos.set(pos.get() + 1);
            if adj_symbol {
                n
            } else {
                0
            }
        };
        sum += n;
    }
    sum
}
pub fn part2() -> usize {
    let mut numbers: Vec<((usize, usize), NonZeroU32)> = Vec::with_capacity(100);
    let mut stars: Vec<(usize, usize)> = Vec::with_capacity(100);
    for item in GridIter::new(INPUT) {
        match item {
            GridItem {
                pos,
                kind: ItemKind::Number(n),
            } => numbers.push((pos, n)),
            GridItem {
                pos,
                kind: ItemKind::Symbol(b'*'),
            } => stars.push(pos),
            _ => (),
        }
    }
    let number_positions: FxHashMap<(usize, usize), NonZeroU32> = numbers
        .into_iter()
        .flat_map(|((x, y), n)| (0..number_width(n).get() as usize).map(move |dx| ((x + dx, y), n)))
        .collect();

    let mut sum = 0;

    'stars: for (x, y) in stars {
        let mut n1 = None;
        let mut n2 = None;
        for (dx, dy) in DELTAS {
            let pos = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
            if let Some(n) = number_positions.get(&pos) {
                if n1.is_none() || n1 == Some(*n) {
                    n1 = Some(*n)
                } else if n2.is_none() || n2 == Some(*n) {
                    n2 = Some(*n)
                } else {
                    continue 'stars;
                }
            }
        }
        if let (Some(n1), Some(n2)) = (n1, n2) {
            sum += n1.get() as usize * n2.get() as usize
        }
    }
    sum
}

struct GridIter<'a> {
    src: &'a [u8],
    pos: usize,
    width: Option<NonZeroUsize>,
}

impl<'a> GridIter<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
            width: None,
        }
    }

    fn parse_number(&mut self) -> NonZeroU32 {
        let mut n = (self.src[self.pos] - b'0') as u32;
        while let Some(b) = self.src.get(self.pos + 1) {
            if b.is_ascii_digit() {
                n = n * 10 + (b - b'0') as u32;
                self.pos += 1;
            } else {
                break;
            }
        }
        NonZeroU32::new(n).unwrap()
    }
}

impl<'a> Iterator for GridIter<'a> {
    type Item = GridItem;
    fn next(&mut self) -> Option<Self::Item> {
        let (kind, current_pos) = loop {
            let cur = self.pos;
            match self.src.get(cur)? {
                b'\n' => {
                    self.pos += 1;
                    self.width
                        .get_or_insert(NonZeroUsize::new(self.pos).unwrap());
                }
                b'.' => {
                    self.pos += 1;
                }
                b'0'..=b'9' => break (ItemKind::Number(self.parse_number()), cur),
                c => break (ItemKind::Symbol(*c), cur),
            }
        };
        // println!("{kind:#?}");
        self.pos += 1;
        let pos = if let Some(w) = self.width {
            (current_pos % w.get(), current_pos / w.get())
        } else {
            (current_pos, 0)
        };
        Some(GridItem { pos, kind })
    }
}

#[derive(Debug, Copy, Clone)]
struct GridItem {
    // x, y of the start
    pos: (usize, usize),
    kind: ItemKind,
}

#[derive(Debug, Copy, Clone)]
enum ItemKind {
    Number(NonZeroU32),
    Symbol(u8),
}

use rustc_hash::{FxHashMap, FxHashSet};
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
    let (numbers, symbols): (Vec<_>, Vec<_>) =
        GridIter::new(INPUT).partition(|i| matches!(i.kind, ItemKind::Number(_)));
    let symbol_adjacents = symbols
        .iter()
        .flat_map(|i| {
            DELTAS.map(|(dx, dy)| {
                (
                    i.pos.0.saturating_add_signed(dx),
                    i.pos.1.saturating_add_signed(dy),
                )
            })
        })
        .collect::<FxHashSet<_>>();
    let mut sum = 0;
    for n in numbers {
        let ItemKind::Number(num) = n.kind else {
            unreachable!()
        };
        let mut pos = n.pos;
        for _ in 0..number_width(num).get() {
            if symbol_adjacents.contains(&pos) {
                sum += num.get() as usize;
                break;
            }
            pos.0 += 1;
        }
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
        self.pos += 1;
        while let Some(b) = self.src.get(self.pos) {
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
        loop {
            match self.src.get(self.pos)? {
                b'\n' => {
                    self.pos += 1;
                    self.width
                        .get_or_insert(NonZeroUsize::new(self.pos).unwrap());
                }
                b'.' => {
                    self.pos += 1;
                }
                _ => break,
            }
        }
        while matches!(self.src.get(self.pos)?, b'\n' | b'.') {
            self.pos += 1;
        }
        let mut c = *self.src.get(self.pos)?;
        if c == b'\n' {
            self.pos += 1;
            // This will only set the width the first time
            self.width
                .get_or_insert(NonZeroUsize::new(self.pos).unwrap());
            c = *self.src.get(self.pos)?;
        }
        let current_pos = self.pos;
        let kind = match c {
            b'0'..=b'9' => ItemKind::Number(self.parse_number()),
            s => {
                self.pos += 1;
                ItemKind::Symbol(s)
            }
        };
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

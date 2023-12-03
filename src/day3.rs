use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/3.txt");

pub fn part1() -> usize {
    let mut sum = 0;
    let mut walker = GridWalker::new(INPUT);
    while let Some(n) = walker.walk_until_number() {
        let mut n = n as usize;
        let mut adj_symbol = false;
        adj_symbol |= walker.left().0 != b'.'
            || walker.up_left().0 != b'.'
            || walker.down_left().0 != b'.'
            || walker.up().0 != b'.'
            || walker.down().0 != b'.';
        while let Some(nx @ b'0'..=b'9') = walker.next() {
            n = n * 10 + (nx - b'0') as usize;
            walker.step();
            adj_symbol |= walker.up().0 != b'.' || walker.down().0 != b'.';
        }
        adj_symbol |= walker.right().0 != b'.'
            || walker.up_right().0 != b'.'
            || walker.down_right().0 != b'.';
        walker.step();
        if adj_symbol {
            sum += n;
        }
    }
    sum
}

struct SmallList<const N: usize, T> {
    storage: [Option<T>; N],
    overfull: bool,
}

impl<const N: usize, T> Default for SmallList<N, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, T> SmallList<N, T> {
    pub fn new() -> Self {
        assert_ne!(N, 0);
        Self {
            // can't do [None; N] as None::<T> is non-Copy
            storage: [0; N].map(|_| None),
            overfull: false,
        }
    }

    pub fn insert(&mut self, val: T) {
        if self.overfull {
            return;
        }
        for idx in 0..N {
            if self.storage[idx].is_none() {
                self.storage[idx] = Some(val);
                return;
            }
        }
        self.overfull = true;
    }
}

pub fn part2() -> usize {
    let mut gears: FxHashMap<usize, SmallList<2, usize>> =
        FxHashMap::with_capacity_and_hasher(50, Default::default());

    fn pos_if_star(inp: (u8, Option<usize>)) -> Option<usize> {
        if inp.0 == b'*' {
            inp.1
        } else {
            None
        }
    }

    let mut walker = GridWalker::new(INPUT);
    while let Some(n) = walker.walk_until_number() {
        let mut adj_star = None;
        let mut n = n as usize;
        adj_star = pos_if_star(walker.left())
            .or_else(|| pos_if_star(walker.up_left()))
            .or_else(|| pos_if_star(walker.down_left()))
            .or_else(|| pos_if_star(walker.up()))
            .or_else(|| pos_if_star(walker.down()));
        while let Some(nx @ b'0'..=b'9') = walker.next() {
            n = n * 10 + (nx - b'0') as usize;
            walker.step();
            adj_star = adj_star
                .or_else(|| pos_if_star(walker.up()).or_else(|| pos_if_star(walker.down())));
        }
        adj_star = adj_star.or_else(|| {
            pos_if_star(walker.right())
                .or_else(|| pos_if_star(walker.up_right()))
                .or_else(|| pos_if_star(walker.down_right()))
        });
        walker.step();
        if let Some(pos) = adj_star {
            gears.entry(pos).or_default().insert(n)
        }
    }
    gears
        .values()
        .filter_map(|v| match v.storage {
            [Some(n1), Some(n2)] if !v.overfull => Some(n1 * n2),
            _ => None,
        })
        .sum()
}

struct GridWalker<'a> {
    grid: &'a [u8],
    pos: usize,
    width: usize,
}

impl<'a> GridWalker<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            grid: src.as_bytes(),
            pos: 0,
            width: src.as_bytes().iter().position(|b| *b == b'\n').unwrap(),
        }
    }

    /// Walk forward until `pos` is at a digit and return it
    /// Return None if we reached the end of the input instead
    pub fn walk_until_number(&mut self) -> Option<u8> {
        loop {
            match self.grid.get(self.pos) {
                Some(c @ b'1'..=b'9') => return Some(*c - b'0'),
                None => return None,
                _ => self.pos += 1,
            }
        }
    }

    #[inline(always)]
    pub fn cur(&self) -> u8 {
        self.grid[self.pos]
    }

    #[inline(always)]
    pub fn next(&self) -> Option<u8> {
        self.grid.get(self.pos + 1).copied()
    }

    #[inline(always)]
    pub fn step(&mut self) {
        self.pos += 1;
    }

    fn on_left_edge(&self) -> bool {
        self.pos == 0 || matches!(self.grid.get(self.pos - 1), Some(b'\n') | None)
    }

    fn on_right_edge(&self) -> bool {
        matches!(self.grid.get(self.pos + 1), Some(b'\n') | None)
    }

    fn on_top(&self) -> bool {
        self.pos < self.width
    }

    fn on_bottom(&self) -> bool {
        self.grid.len() - self.pos <= self.width
    }

    // get char in direction, or a '.'
    pub fn up_left(&self) -> (u8, Option<usize>) {
        if self.on_left_edge() || self.on_top() {
            (b'.', None)
        } else {
            let p = self.pos - 1 - self.width - 1;
            (self.grid[p], Some(p))
        }
    }

    pub fn up(&self) -> (u8, Option<usize>) {
        if self.on_top() {
            (b'.', None)
        } else {
            let p = self.pos - 1 - self.width;
            (self.grid[p], Some(p))
        }
    }

    pub fn up_right(&self) -> (u8, Option<usize>) {
        if self.on_right_edge() || self.on_top() {
            (b'.', None)
        } else {
            let p = self.pos + 1 - self.width - 1;
            (self.grid[p], Some(p))
        }
    }

    pub fn left(&self) -> (u8, Option<usize>) {
        if self.on_left_edge() {
            (b'.', None)
        } else {
            let p = self.pos - 1;
            (self.grid[p], Some(p))
        }
    }

    pub fn right(&self) -> (u8, Option<usize>) {
        if self.on_right_edge() {
            (b'.', None)
        } else {
            let p = self.pos + 1;
            (self.grid[p], Some(p))
        }
    }

    pub fn down_left(&self) -> (u8, Option<usize>) {
        if self.on_left_edge() || self.on_bottom() {
            (b'.', None)
        } else {
            let p = self.pos - 1 + self.width + 1;
            (self.grid[p], Some(p))
        }
    }

    pub fn down(&self) -> (u8, Option<usize>) {
        if self.on_bottom() {
            (b'.', None)
        } else {
            let p = self.pos + self.width + 1;
            (self.grid[p], Some(p))
        }
    }

    pub fn down_right(&self) -> (u8, Option<usize>) {
        if self.on_right_edge() || self.on_bottom() {
            (b'.', None)
        } else {
            let p = self.pos + 1 + self.width + 1;
            (self.grid[p], Some(p))
        }
    }
}

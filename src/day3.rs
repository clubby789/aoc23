use rustc_hash::FxHashMap;
use std::cell::Cell;

const INPUT: &str = include_str!("inputs/3.txt");

pub fn part1() -> usize {
    let mut sum = 0;
    let mut pos = Cell::new(0);
    let grid = INPUT.as_bytes();
    let width = INPUT.find('\n').unwrap();

    'outer: loop {
        // advance until digits or end of file
        loop {
            if let Some(&cur) = grid.get(pos.get()) {
                if cur > b'0' && cur <= b'9' {
                    break;
                } else {
                    pos.set(pos.get() + 1)
                }
            } else {
                break 'outer;
            }
        }
        let on_left_edge =
            || pos.get() == 0 || matches!(grid.get(pos.get() - 1), Some(b'\n') | None);

        let on_right_edge = || matches!(grid.get(pos.get() + 1), Some(b'\n') | None);

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
            while pos.get() < grid.len() - 1 && matches!(grid[pos.get() + 1], b'0'..=b'9') {
                n = n * 10 + (grid[pos.get() + 1] - b'0') as usize;
                pos.set(pos.get() + 1);
                adj_symbol |= up() || down();
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
    let mut pos = Cell::new(0);
    let grid = INPUT.as_bytes();
    let width = INPUT.find('\n').unwrap();
    let mut gears: FxHashMap<usize, SmallList<2, usize>> =
        FxHashMap::with_capacity_and_hasher(50, Default::default());

    'outer: loop {
        // advance until digits or end of file
        loop {
            if let Some(&cur) = grid.get(pos.get()) {
                if cur > b'0' && cur <= b'9' {
                    break;
                } else {
                    pos.set(pos.get() + 1)
                }
            } else {
                break 'outer;
            }
        }
        let on_left_edge =
            || pos.get() == 0 || matches!(grid.get(pos.get() - 1), Some(b'\n') | None);

        let on_right_edge = || matches!(grid.get(pos.get() + 1), Some(b'\n') | None);

        let on_top = || pos.get() < width;

        let on_bottom = || grid.len() - pos.get() <= width;

        // closures to check if there is a star in the given position
        // minus and plus one where necessary to bypass newlines
        let up_left = || {
            if on_left_edge() || on_top() {
                None
            } else {
                let p = pos.get() - 1 - width - 1;
                if grid[p] == b'*' {
                    Some(p)
                } else {
                    None
                }
            }
        };

        let up = || {
            if on_top() {
                None
            } else {
                let p = pos.get() - width - 1;
                if grid[p] == b'*' {
                    Some(p)
                } else {
                    None
                }
            }
        };

        let up_right = || {
            if on_right_edge() || on_top() {
                None
            } else {
                let p = pos.get() + 1 - width - 1;
                if grid[p] == b'*' {
                    Some(p)
                } else {
                    None
                }
            }
        };

        let left = || {
            if on_left_edge() {
                None
            } else {
                let p = pos.get() - 1;
                if grid[p] == b'*' {
                    Some(p)
                } else {
                    None
                }
            }
        };

        let right = || {
            if on_right_edge() {
                None
            } else {
                let p = pos.get() + 1;
                if grid[p] == b'*' {
                    Some(p)
                } else {
                    None
                }
            }
        };

        let down_left = || {
            if on_left_edge() || on_bottom() {
                None
            } else {
                let p = pos.get() - 1 + width + 1;
                if grid[p] == b'*' {
                    Some(p)
                } else {
                    None
                }
            }
        };

        let down = || {
            if on_bottom() {
                None
            } else {
                let p = pos.get() + width + 1;
                if grid[p] == b'*' {
                    Some(p)
                } else {
                    None
                }
            }
        };

        let down_right = || {
            if on_right_edge() || on_bottom() {
                None
            } else {
                let p = pos.get() + 1 + width + 1;
                if grid[p] == b'*' {
                    Some(p)
                } else {
                    None
                }
            }
        };

        let mut adj_star = None;
        let mut n = (grid[pos.get()] - b'0') as usize;
        adj_star = left()
            .or_else(up_left)
            .or_else(down_left)
            .or_else(up)
            .or_else(down);
        while pos.get() < grid.len() - 1 && matches!(grid[pos.get() + 1], b'0'..=b'9') {
            n = n * 10 + (grid[pos.get() + 1] - b'0') as usize;
            pos.set(pos.get() + 1);
            adj_star = adj_star.or_else(|| up().or_else(down));
        }
        adj_star = adj_star.or_else(|| right().or_else(up_right).or_else(down_right));
        pos.set(pos.get() + 1);
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

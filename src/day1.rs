const INPUT: &str = include_str!("inputs/1.txt");

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            (
                l.as_bytes()
                    .iter()
                    .find(|b| b.is_ascii_digit())
                    .map(|i| i - b'0')
                    .unwrap(),
                l.as_bytes()
                    .iter()
                    .rev()
                    .find(|b| b.is_ascii_digit())
                    .map(|i| i - b'0')
                    .unwrap(),
            )
        })
        .map(|(l, r)| (l * 10 + r) as usize)
        .sum()
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let matcher = |window: &[u8]| match *window {
                [n, ..] if n.is_ascii_digit() => Some((n - b'0') as usize),
                [b'o', b'n', b'e', ..] => Some(1),
                [b't', b'w', b'o', ..] => Some(2),
                [b't', b'h', b'r', b'e', b'e', ..] => Some(3),
                [b'f', b'o', b'u', b'r', ..] => Some(4),
                [b'f', b'i', b'v', b'e', ..] => Some(5),
                [b's', b'i', b'x', ..] => Some(6),
                [b's', b'e', b'v', b'e', b'n', ..] => Some(7),
                [b'e', b'i', b'g', b'h', b't', ..] => Some(8),
                [b'n', b'i', b'n', b'e', ..] => Some(9),
                [_, _, _, _, _, _, ..] => unreachable!(),
                _ => None,
            };
            let hi = WindowsUpto::new(l.as_bytes(), 5).find_map(matcher).unwrap();
            let lo = WindowsUpto::new(l.as_bytes(), 5)
                .rev()
                .find_map(matcher)
                .unwrap();
            hi * 10 + lo
        })
        .sum()
}

struct WindowsUpto<'a, T: 'a> {
    v: &'a [T],
    size: usize,
    steps_back: usize,
}

impl<'a, T: 'a> WindowsUpto<'a, T> {
    pub fn new(v: &'a [T], size: usize) -> Self {
        Self {
            v,
            size,
            steps_back: 0,
        }
    }
}

impl<'a, T> Iterator for WindowsUpto<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let length = self.size.min(self.v.len());
            let ret = Some(&self.v[..length]);
            self.v = &self.v[1..];
            ret
        }
    }
}

impl<'a, T> DoubleEndedIterator for WindowsUpto<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let ret = if self.steps_back < self.size {
                // first few iterations
                &self.v[self.v.len() - (self.steps_back + 1)..]
            } else {
                &self.v[self.v.len() - (self.steps_back + 1)..][..self.size]
            };
            // This should really remove the back of v, but since we only step
            // exclusively forward or back, who cares :^)
            self.steps_back += 1;
            Some(ret)
        }
    }
}

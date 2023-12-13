const INPUT: &str = include_str!("inputs/13.txt");

struct Pattern<'a> {
    source: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Pattern<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        let width = source.iter().position(|&b| b == b'\n').unwrap() + 1;
        let height = source.len().div_ceil(width);
        Self {
            source,
            width,
            height,
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &'a [u8]> + DoubleEndedIterator + ExactSizeIterator {
        self.source.chunks(self.width).map(|slice| match slice {
            [start @ .., b'\n'] | start => start,
        })
    }
}

fn find_reflection_value(pat: impl AsRef<[u8]>, ignore: Option<usize>) -> Option<usize> {
    fn is_mirrored<T: PartialEq>(row: &[T], mirror: usize) -> bool {
        let (mut before, mut after) = row.split_at(mirror);
        while let Some(((before_last, before_rest), (after_first, after_rest))) =
            before.split_last().zip(after.split_first())
        {
            if before_last != after_first {
                return false;
            }
            before = before_rest;
            after = after_rest;
        }
        true
    }
    let pat = Pattern::new(pat.as_ref());
    // Try each column
    let rows: Vec<_> = pat.rows().collect();
    for reflect_col in 1..pat.width - 1 {
        if rows.iter().all(|row| is_mirrored(row, reflect_col)) && ignore != Some(reflect_col) {
            return Some(reflect_col);
        }
    }

    for reflect_row in 1..pat.height {
        if is_mirrored(&rows, reflect_row) && ignore != Some(100 * reflect_row) {
            return Some(100 * reflect_row);
        }
    }
    None
}

pub fn part1() -> usize {
    INPUT
        .split("\n\n")
        .map(|p| find_reflection_value(p, None).unwrap())
        .sum()
}

pub fn part2() -> usize {
    let mut sum = 0;
    'patterns: for pat in INPUT.split("\n\n") {
        let mut pattern = pat.to_owned().into_bytes();
        let orig = find_reflection_value(&pattern, None).unwrap();
        fn flip(b: &mut u8) {
            match *b {
                b'#' => *b = b'.',
                b'.' => *b = b'#',
                _ => (),
            }
        }
        for i in 0..pattern.len() {
            if i != 0 {
                flip(&mut pattern[i - 1]);
            }
            if pattern[i] == b'\n' {
                continue;
            }
            flip(&mut pattern[i]);
            if let Some(val) = find_reflection_value(&pattern, Some(orig)) {
                debug_assert!(val != orig);
                sum += val;
                continue 'patterns;
            }
        }
        unreachable!()
    }
    sum
}

const INPUT: &str = include_str!("inputs/18.txt");

fn solve_points(path: &[(i64, i64)], points_on_path: u64) -> usize {
    let mut area = 0;
    for i in 0..path.len() {
        let j = (i + 1) % path.len();
        let v1 = path[i];
        let v2 = path[j];

        area += (v1.0 * v2.1) - (v1.1 * v2.0);
    }
    // let area = area.abs() / 2;
    // A = i + b/2 - 1
    // A + 1 = i + b/2
    let b_2 = points_on_path / 2;
    let area = area.abs() / 2;
    let interior = area + 1 - b_2 as i64;
    (interior + points_on_path as i64) as usize
}

struct Trench<'a, F> {
    input: &'a [u8],
    position: usize,
    location: (i64, i64),
    f: F,
    done: bool,
}

impl<'a, F> Trench<'a, F>
where
    F: FnMut(&[u8], (i64, i64)) -> ((i64, i64), usize),
{
    pub fn new(source: &'a str, f: F) -> Self {
        Self {
            input: source.as_bytes(),
            position: 0,
            location: (0, 0),
            done: false,
            f,
        }
    }
}

impl<'a, F> Iterator for Trench<'a, F>
where
    F: FnMut(&[u8], (i64, i64)) -> ((i64, i64), usize),
{
    type Item = (i64, i64);
    fn next(&mut self) -> Option<Self::Item> {
        let Some(line) = self.input.get(self.position..) else {
            return if self.done {
                None
            } else {
                self.done = true;
                Some((0, 0))
            };
        };
        let ret = self.location;
        let (new_loc, len) = (self.f)(line, ret);
        self.position += len;
        self.location = new_loc;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(self.input.len() / 14))
    }
}

pub fn part1() -> usize {
    let path = Trench::new(INPUT, |line, last| {
        let (direction, amnt, length) = match line {
            &[dir, _, amnt, b' ', ..] => (dir, (amnt & 0b1111) as i64, 14),
            &[dir, _, hi, lo, ..] => (dir, ((hi & 0b1111) * 10 + (lo & 0b1111)) as i64, 15),
            _ => unreachable!(),
        };
        let diff = match direction {
            b'U' => (0, -amnt),
            b'R' => (amnt, 0),
            b'D' => (0, amnt),
            b'L' => (-amnt, 0),
            _ => unreachable!(),
        };
        ((last.0 + diff.0, last.1 + diff.1), length)
    })
    .collect::<Vec<_>>();
    let points_on_path = path
        .windows(2)
        .map(|window| {
            let dx = window[1].0.abs_diff(window[0].0);
            let dy = window[1].1.abs_diff(window[0].1);
            dx + dy
        })
        .sum();
    solve_points(&path, points_on_path)
}

pub fn part2() -> usize {
    let path = Trench::new(INPUT, |line, last| {
        fn parse_hex(dir: u8, amnt: [u8; 5]) -> (u8, i64) {
            const fn hex_table() -> [u8; 255] {
                let mut table = [0; 255];
                table[b'0' as usize] = 0;
                table[b'1' as usize] = 1;
                table[b'2' as usize] = 2;
                table[b'3' as usize] = 3;
                table[b'4' as usize] = 4;
                table[b'5' as usize] = 5;
                table[b'6' as usize] = 6;
                table[b'7' as usize] = 7;
                table[b'8' as usize] = 8;
                table[b'9' as usize] = 9;
                table[b'a' as usize] = 10;
                table[b'b' as usize] = 11;
                table[b'c' as usize] = 12;
                table[b'd' as usize] = 13;
                table[b'e' as usize] = 14;
                table[b'f' as usize] = 15;
                table
            }
            const HEX_TABLE: [u8; 255] = hex_table();
            let amnt = amnt
                .into_iter()
                .fold(0i64, |acc, x| (acc << 4) + HEX_TABLE[x as usize] as i64);
            (dir, amnt)
        }
        let ((direction, amnt), len) = match line {
            &[_, b' ', _, b' ', b'(', b'#', a, b, c, d, e, dir, ..] => {
                (parse_hex(dir, [a, b, c, d, e]), 14)
            }
            &[_, b' ', _, _, b' ', b'(', b'#', a, b, c, d, e, dir, ..] => {
                (parse_hex(dir, [a, b, c, d, e]), 15)
            }
            _ => unreachable!(),
        };
        let diff = match direction {
            b'0' => (0, -amnt),
            b'1' => (amnt, 0),
            b'2' => (0, amnt),
            b'3' => (-amnt, 0),
            _ => unreachable!(),
        };
        ((last.0 + diff.0, last.1 + diff.1), len)
    })
    .collect::<Vec<_>>();
    let points_on_path = path
        .windows(2)
        .map(|window| {
            let dx = window[1].0.abs_diff(window[0].0);
            let dy = window[1].1.abs_diff(window[0].1);
            dx + dy
        })
        .sum();
    solve_points(&path, points_on_path)
}

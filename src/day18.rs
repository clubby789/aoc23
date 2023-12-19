const INPUT: &str = include_str!("inputs/18.txt");

fn solve_points(path: impl Iterator<Item = (i64, i64)>) -> usize {
    let mut path = path.chain(std::iter::once((0, 0)));
    let mut area = 0;
    let mut points_on_path = 0;
    let mut prev = path.next().unwrap();
    while let Some(cur) = path.next() {
        points_on_path += {
            #[inline(always)]
            fn branchless_abs(a: i64) -> i64 {
                (a + (a >> 63)) ^ (a >> 63)
            }
            branchless_abs((prev.0 - cur.0) ^ (prev.1 - cur.1))
        };
        area += (prev.0 * cur.1) - (prev.1 * cur.0);
        prev = cur;
    }
    // A = i + b/2 - 1
    // A + 1 = i + b/2
    // A + 1 - b/2 = i
    // Total points = A + 1 - b/2 + b
    let b_2 = points_on_path / 2;
    let area = area.abs() / 2;
    let interior = area + 1 - b_2;
    (interior + points_on_path) as usize
}

struct Trench<'a, F> {
    input: &'a [u8],
    position: usize,
    location: (i64, i64),
    f: F,
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
            f,
        }
    }
}

impl<'a, F> Iterator for Trench<'a, F>
where
    F: FnMut(&[u8], (i64, i64)) -> ((i64, i64), usize),
{
    type Item = (i64, i64);
    // perf says this saves about 15% for part 2
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.input.get(self.position..)?;
        let ret = self.location;
        let (new_loc, len) = (self.f)(line, ret);
        self.position += len;
        self.location = new_loc;
        Some(ret)
    }
}

pub fn part1() -> usize {
    let trench = Trench::new(INPUT, |line, last| {
        let (direction, amnt, length) = match line {
            &[dir, _, amnt, b' ', ..] => (dir, (amnt & 0b1111) as i64, 14),
            &[dir, _, hi, lo, ..] => (dir, ((hi & 0b1111) * 10 + (lo & 0b1111)) as i64, 15),
            _ => unreachable!(),
        };
        let diff = match direction {
            b'U' => (0, -amnt),
            b'R' => (amnt, 0),
            b'D' => (0, amnt),
            _ => {
                debug_assert_eq!(direction, b'L');
                (-amnt, 0)
            }
        };
        ((last.0 + diff.0, last.1 + diff.1), length)
    });
    solve_points(trench)
}

pub fn part2() -> usize {
    let trench = Trench::new(INPUT, |line, last| {
        fn parse_hex(dir: u8, amnt: [u8; 5]) -> (u8, i64) {
            const HEX_TABLE_LO: [u8; 256] = {
                let mut table = [0; 256];
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
            };
            const HEX_TABLE_HI: [u8; 256] = {
                let mut table = [0; 256];
                table[b'0' as usize] = 0 << 4;
                table[b'1' as usize] = 1 << 4;
                table[b'2' as usize] = 2 << 4;
                table[b'3' as usize] = 3 << 4;
                table[b'4' as usize] = 4 << 4;
                table[b'5' as usize] = 5 << 4;
                table[b'6' as usize] = 6 << 4;
                table[b'7' as usize] = 7 << 4;
                table[b'8' as usize] = 8 << 4;
                table[b'9' as usize] = 9 << 4;
                table[b'a' as usize] = 10 << 4;
                table[b'b' as usize] = 11 << 4;
                table[b'c' as usize] = 12 << 4;
                table[b'd' as usize] = 13 << 4;
                table[b'e' as usize] = 14 << 4;
                table[b'f' as usize] = 15 << 4;
                table
            };
            let amnt = (HEX_TABLE_LO[amnt[0] as usize] as i64) << 4 * 4
                | (HEX_TABLE_HI[amnt[1] as usize] as i64) << 4 * 2
                | (HEX_TABLE_LO[amnt[2] as usize] as i64) << 4 * 2
                | (HEX_TABLE_HI[amnt[3] as usize] as i64)
                | HEX_TABLE_LO[amnt[4] as usize] as i64;
            (dir, amnt)
        }
        let ((direction, amnt), len) = match line {
            &[_, _, _, b' ', _, _, a, b, c, d, e, dir, ..] => (parse_hex(dir, [a, b, c, d, e]), 14),
            &[_, _, _, _, b' ', _, _, a, b, c, d, e, dir, ..] => {
                (parse_hex(dir, [a, b, c, d, e]), 15)
            }
            _ => unreachable!(),
        };
        let diff = match direction {
            b'0' => (0, -amnt),
            b'1' => (amnt, 0),
            b'2' => (0, amnt),
            _ => {
                debug_assert_eq!(direction, b'3');
                (-amnt, 0)
            }
        };
        ((last.0 + diff.0, last.1 + diff.1), len)
    });
    solve_points(trench)
}

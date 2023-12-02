const INPUT: &str = include_str!("inputs/2.txt");

// Removes the 'Game x: ' prefix from a line
fn skip_game(game: &str, num: usize) -> &str {
    match num {
        0..=9 => &game[8..],
        10..=99 => &game[9..],
        100.. => &game[10..],
    }
}

pub fn part1() -> usize {
    INPUT
        .lines()
        .enumerate()
        .filter(|&(i, s)| {
            Pulls::new(skip_game(s, i + 1)).all(|(r, g, b)| r <= 12 && g <= 13 && b <= 14)
        })
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .enumerate()
        .map(|(i, game)| {
            let (red, green, blue) = Pulls::new(skip_game(game, i + 1))
                .fold((0, 0, 0), |(r, g, b), (red, green, blue)| {
                    (r.max(red), g.max(green), b.max(blue))
                });
            red as usize * green as usize * blue as usize
        })
        .sum()
}

struct Pulls<'a> {
    src: &'a [u8],
    pos: usize,
}

// Takes a list of pulls and returns the number of (red, green, blue) cubes in each
impl<'a> Pulls<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
        }
    }
}

impl<'a> Iterator for Pulls<'a> {
    type Item = (u8, u8, u8);
    fn next(&mut self) -> Option<Self::Item> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        loop {
            let n = match (self.src.get(self.pos)?, self.src.get(self.pos + 1)?) {
                // single digit
                (lo, b' ') => {
                    debug_assert!(lo.is_ascii_digit());
                    self.pos += 2;
                    lo - b'0'
                }
                // double digit
                (hi, lo) => {
                    debug_assert!(hi.is_ascii_digit());
                    debug_assert!(lo.is_ascii_digit());
                    self.pos += 3;
                    (hi - b'0') * 10 + (lo - b'0')
                }
            };
            match self.src[self.pos] {
                b'r' => {
                    red = n;
                    self.pos += 3;
                }
                b'g' => {
                    green = n;
                    self.pos += 5;
                }
                b'b' => {
                    blue = n;
                    self.pos += 4;
                }
                _ => unreachable!(),
            }
            match self.src.get(self.pos) {
                Some(b',') => {
                    self.pos += 2;
                    continue;
                }
                // end of pull or line
                _ => {
                    self.pos += 2;
                    return Some((red, green, blue));
                }
            }
        }
    }
}

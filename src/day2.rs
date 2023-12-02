const INPUT: &str = include_str!("inputs/2.txt");

// Given a single game, find the minimum number of cubes that must have been in the bag
fn min_num_cubes(game: &str) -> (u8, u8, u8) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let pulls = game.split_once(": ").unwrap().1;
    for (r, g, b) in Pulls::new(pulls) {
        red = red.max(r);
        green = green.max(g);
        blue = blue.max(b);
    }
    (red, green, blue)
}

pub fn part1() -> usize {
    let mut sum = 0;
    'games: for (i, game) in INPUT.lines().enumerate() {
        let pulls = game.split_once(": ").unwrap().1;
        for (r, g, b) in Pulls::new(pulls) {
            if r > 12 || g > 13 || b > 14 {
                continue 'games;
            }
        }
        sum += i + 1;
    }
    sum
}
pub fn part2() -> usize {
    let mut sum = 0;
    for game in INPUT.lines() {
        let (red, green, blue) = min_num_cubes(game);
        sum += red as usize * green as usize * blue as usize
    }
    sum
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
            pos: 0
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
                _ => unreachable!()
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
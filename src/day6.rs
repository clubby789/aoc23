const INPUT: &str = include_str!("inputs/6.txt");

fn calculate_number_of_wins(limit: u64, distance: u64) -> u32 {
    /*
    d = h * (l - h)
    h * (l - h) > d
    hl - h^2 > d
    h^2 - hl + d = 0
    solve for h with quadratic formula
    h = (l +/- sqrt(l^2 - 4d)) / 2
     */
    let limit = limit as f64;
    let distance = distance as f64;
    let lo = (limit - (limit.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    let hi = (limit + (limit.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    (hi.ceil() - lo.floor()) as u32 - 1
}

struct Part1Iterator<'a> {
    src_time: &'a [u8],
    src_dist: &'a [u8],
    pos: usize,
}

impl<'a> Part1Iterator<'a> {
    pub fn new(s: &'a str) -> Self {
        let src = s.as_bytes();
        let newline = src.iter().position(|&b| b == b'\n').unwrap();
        let (src_time, src_dist) = src.split_at(newline);
        let src_time = &src_time[9..]; // skip 'Time:' and extra spaces used for alignment
        let src_dist = &src_dist[10..]; // skip newline and 'Distance:'
        Self {
            src_time,
            src_dist,
            pos: 0,
        }
    }
}

impl<'a> Iterator for Part1Iterator<'a> {
    type Item = (u64, u64);
    fn next(&mut self) -> Option<Self::Item> {
        let mut time = 0;
        let mut distance = 0;
        let mut pos = self.pos;
        while let Some((&t, &d)) = Option::zip(self.src_time.get(pos), self.src_dist.get(pos)) {
            // reached space at the end of the number
            if t == b' ' && time > 0 {
                break;
            }
            if t != b' ' {
                time = time * 10 + (t & 0xf) as u64;
            }
            if d != b' ' {
                distance = distance * 10 + (d & 0xf) as u64;
            }
            pos += 1;
        }
        // didn't get any numbers
        if time == 0 {
            return None;
        }
        self.pos = pos;
        Some((time, distance))
    }
}

pub fn part1() -> usize {
    Part1Iterator::new(INPUT)
        .map(|(time, distance)| calculate_number_of_wins(time, distance) as usize)
        .product()
}

fn parse_part2(input: &str) -> (u64, u64) {
    let mut pos = 9;
    let input = input.as_bytes();
    while input[pos] == b' ' {
        pos += 1;
    }
    let mut time = 0;
    loop {
        match input[pos] {
            b' ' => (),
            b'\n' => {
                break;
            }
            n => {
                debug_assert!(matches!(n, b'0'..=b'9'));
                time = time * 10 + (n & 0xf) as u64;
            }
        }
        pos += 1;
    }
    pos += 10;
    let mut distance = 0;
    while let Some(&n) = input.get(pos) {
        pos += 1;
        if n == b' ' {
            continue;
        }
        debug_assert!(matches!(n, b'0'..=b'9'), "'{}'", n as char);
        distance = distance * 10 + (n & 0xf) as u64;
    }

    (time, distance)
}

pub fn part2() -> usize {
    let (time, distance) = parse_part2(INPUT);
    calculate_number_of_wins(time, distance) as usize
}

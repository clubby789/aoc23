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
    let limit = limit as f32;
    let distance = distance as f32;
    let lo = (limit - (limit.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    let hi = (limit + (limit.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    (hi.ceil() - lo.floor()) as u32 - 1
}

fn parse_ascii(s: &str) -> u64 {
    s.as_bytes()
        .iter()
        .fold(0, |acc, b| acc * 10 + (b & 0xf) as u64)
}

pub fn part1() -> usize {
    let (times, distances) = INPUT.split_once('\n').unwrap();
    let times = times.split_once(':').unwrap().1.trim_start();
    let distances = distances.split_once(':').unwrap().1.trim_start();
    let times = times.split_ascii_whitespace().map(parse_ascii);
    let distances = distances.split_ascii_whitespace().map(parse_ascii);
    times
        .zip(distances)
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

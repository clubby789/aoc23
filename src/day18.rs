const INPUT: &str = include_str!("inputs/18.txt");

fn solve_points(path: &[(i64, i64)], points_on_path: i64) -> usize {
    let mut area = 0;
    for i in 0..path.len() {
        let j = (i + 1) % path.len();
        let v1 = path[i];
        let v2 = path[j];

        area += (v1.0 * v2.1) - (v1.1 * v2.0);
    }
    //let area = area.abs() / 2;
    // A = i + b/2 - 1
    // A + 1 = i + b/2
    let b_2 = points_on_path / 2;
    let area = area.abs() / 2;
    let interior = area + 1 - b_2;
    (interior + points_on_path) as usize
}

pub fn part1() -> usize {
    let mut path = Vec::with_capacity(INPUT.len() / 14 + 1);
    let mut points_on_path = 0;
    path.push((0, 0));
    for (direction, amnt) in INPUT.as_bytes().split(|&b| b == b'\n').map(|line| {
        let (dir, amnt) = match line {
            &[dir, _, amnt, b' ', ..] => (dir, amnt & 0b1111),
            &[dir, _, hi, lo, ..] => (dir, (hi & 0b1111) * 10 + (lo & 0b1111)),
            _ => unreachable!(),
        };
        (dir, amnt as i64)
    }) {
        let last = *path.last().unwrap();
        let diff = match direction {
            b'U' => (0, -amnt),
            b'R' => (amnt, 0),
            b'D' => (0, amnt),
            b'L' => (-amnt, 0),
            _ => unreachable!(),
        };
        points_on_path += amnt;
        path.push((last.0 + diff.0, last.1 + diff.1))
    }
    solve_points(&path, points_on_path)
}

pub fn part2() -> usize {
    let mut path = Vec::with_capacity(INPUT.len() / 14 + 1);
    let mut points_on_path = 0;
    path.push((0, 0));
    for (direction, amnt) in INPUT.as_bytes().split(|&b| b == b'\n').map(|line| {
        fn parse_hex(dir: u8, amnt: [u8; 5]) -> (u8, i64) {
            let dir = match dir {
                b'0' => b'R',
                b'1' => b'D',
                b'2' => b'L',
                b'3' => b'U',
                _ => unreachable!(),
            };
            let amnt = amnt.into_iter().fold(0i64, |acc, x| {
                (acc << 4)
                    + match x {
                        b'0'..=b'9' => (x & 0b1111) as i64,
                        b'a'..=b'f' => (x - b'a' + 10) as i64,
                        _ => unreachable!(),
                    }
            });
            (dir, amnt)
        }
        let (dir, amnt) = match line {
            &[.., a, b, c, d, e, dir, _] => {
                parse_hex(dir, [a, b, c, d, e])
            }
            &[.., a, b, c, d, e, dir, _] => {
                parse_hex(dir, [a, b, c, d, e])
            }
            _ => unreachable!(),
        };
        (dir, amnt)
    }) {
        let last = *path.last().unwrap();
        let diff = match direction {
            b'U' => (0, -amnt),
            b'R' => (amnt, 0),
            b'D' => (0, amnt),
            b'L' => (-amnt, 0),
            _ => unreachable!(),
        };
        points_on_path += amnt;
        path.push((last.0 + diff.0, last.1 + diff.1))
    }
    solve_points(&path, points_on_path)
}

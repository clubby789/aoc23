const INPUT: &str = include_str!("inputs/8.txt");

// As long as the input is alphabetic, we can use 5 bits per letter and store it as
// a u16, meaning we can index into a reasonably sized array with it
// sadly this means sample input doesn't work
fn alpha_to_u16(b: [u8; 3]) -> u16 {
    debug_assert!(matches!(b, [b'A'..=b'Z', b'A'..=b'Z', b'A'..=b'Z']));
    let b1 = ((b[0] - b'A') as u16) << 10;
    let b2 = ((b[1] - b'A') as u16) << 5;
    let b3 = ((b[2] - b'A') as u16);
    debug_assert!(b1 & b2 & b3 == 0, "overlapping bits");
    b1 | b2 | b3
}

const AAA: u16 = 0;
const ZZZ: u16 = 26425;

fn parse_node(n: &[u8]) -> (u16, (u16, u16)) {
    let [k1, k2, k3, _, _, _, _, l1, l2, l3, _, _, r1, r2, r3, _, ..] = *n else {
        unreachable!();
    };
    (
        alpha_to_u16([k1, k2, k3]),
        (alpha_to_u16([l1, l2, l3]), alpha_to_u16([r1, r2, r3])),
    )
}

fn make_map(nodes: &str) -> Box<[(u16, u16); u16::MAX as usize]> {
    let mut data: Box<[(u16, u16); u16::MAX as usize]> =
        vec![(0, 0); u16::MAX as usize].try_into().unwrap();
    for (key, value) in nodes.as_bytes().chunks(17).map(parse_node) {
        data[key as usize] = value;
    }
    data
}

pub fn part1() -> usize {
    let (directions, nodes) = INPUT.split_once("\n\n").unwrap();
    let nodes = make_map(nodes);
    let mut cur = AAA;
    let mut iter = directions.bytes().cycle().enumerate();
    for (_, dir) in iter.by_ref() {
        let node = nodes[cur as usize];
        if dir == b'L' {
            cur = node.0
        } else {
            cur = node.1
        };
        if cur == ZZZ {
            break;
        }
    }
    iter.next().unwrap().0
}

fn lcm(a: usize, b: usize) -> usize {
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

pub fn part2() -> usize {
    let (directions, nodes) = INPUT.split_once("\n\n").unwrap();
    let nodes = make_map(nodes);
    let mut cur = Vec::with_capacity(u16::MAX as usize / 32);
    cur.extend(
        (0..u16::MAX)
            .step_by(32)
            .filter(|n| nodes[*n as usize].0 != 0),
    );
    let mut running_lcm = 1;

    let mut iter = directions.bytes().cycle().enumerate();
    for (i, dir) in iter {
        if cur.is_empty() {
            break;
        }
        for j in (0..cur.len()).rev() {
            let nodes = nodes[cur[j] as usize];
            let new = if dir == b'L' { nodes.0 } else { nodes.1 };
            if new & 0b11111 == (b'Z' - b'A') as u16 {
                cur.remove(j);
                running_lcm = lcm(running_lcm, i + 1);
            } else {
                cur[j] = new;
            }
        }
    }
    // paths cycle between their second entry all the way up to Z
    // the LCM of all cycle lengths gives us the point at which they all
    // reach Z at once
    running_lcm
}

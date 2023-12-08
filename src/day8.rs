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

fn parse_node(n: &str) -> (u16, (u16, u16)) {
    let [k1, k2, k3, _, _, _, _, l1, l2, l3, _, _, r1, r2, r3, _] = *n.as_bytes() else {
        unreachable!();
    };
    (
        alpha_to_u16([k1, k2, k3]),
        (alpha_to_u16([l1, l2, l3]), alpha_to_u16([r1, r2, r3])),
    )
}

fn make_map(nodes: &str) -> Box<[Option<(u16, u16)>; u16::MAX as usize]> {
    let mut data: Box<[Option<(u16, u16)>; u16::MAX as usize]> =
        Box::new([None; u16::MAX as usize]);
    for (key, value) in nodes.lines().map(parse_node) {
        data[key as usize] = Some(value);
    }
    data
}

pub fn part1() -> usize {
    let (directions, nodes) = INPUT.split_once("\n\n").unwrap();
    let nodes = make_map(nodes);
    let mut cur = AAA;
    let mut iter = directions.bytes().cycle().enumerate();
    while let Some((_, dir)) = iter.next() {
        let node = nodes[cur as usize].unwrap();
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
    let mut cur = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if i & 0b11111 == (b'Z' - b'A') as usize && v.is_some() {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut running_lcm = 1;

    let mut iter = directions.bytes().cycle().enumerate();
    while let Some((i, dir)) = iter.next() {
        if cur.is_empty() {
            break;
        }
        for j in (0..cur.len()).rev() {
            let nodes = nodes[cur[j]].unwrap();
            let new = if dir == b'L' { nodes.0 } else { nodes.1 };
            if new & 0b11111 == (b'Z' - b'A') as u16 {
                cur.remove(j);
                running_lcm = lcm(running_lcm, i + 1);
            } else {
                cur[j] = new as usize;
            }
        }
    }
    // paths cycle between their second entry all the way up to Z
    // the LCM of all cycle lengths gives us the point at which they all
    // reach Z at once
    running_lcm
}

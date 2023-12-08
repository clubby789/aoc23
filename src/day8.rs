use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/8.txt");

fn parse_node(n: &str) -> (&str, (&str, &str)) {
    let (key, value) = n.split_once(" = ").unwrap();
    let (l, r) = value.trim_matches(['(', ')']).split_once(", ").unwrap();
    (key, (l, r))
}

pub fn part1() -> usize {
    let (directions, nodes) = INPUT.split_once("\n\n").unwrap();
    let nodes: FxHashMap<_, _> = nodes.lines().map(parse_node).collect();
    let mut cur = "AAA";
    let mut iter = directions.bytes().cycle().enumerate();
    while let Some((_, dir)) = iter.next() {
        let node = nodes.get(cur).unwrap();
        if dir == b'L' {
            cur = node.0
        } else {
            cur = node.1
        };
        if cur == "ZZZ" {
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
    let nodes: FxHashMap<_, _> = nodes.lines().map(parse_node).collect();
    let mut cur = nodes
        .keys()
        .filter(|k| k.ends_with('Z'))
        .copied()
        .collect::<Vec<_>>();
    let mut running_lcm = 1;

    let mut iter = directions.bytes().cycle().enumerate();
    while let Some((i, dir)) = iter.next() {
        if cur.is_empty() {
            break;
        }
        for j in (0..cur.len()).rev() {
            let nodes = *nodes.get(cur[j]).unwrap();
            let new = if dir == b'L' { nodes.0 } else { nodes.1 };
            if new.ends_with('Z') {
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

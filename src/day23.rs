/*
const INPUT: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
 */

const INPUT: &str = include_str!("inputs/23.txt");

#[derive(Clone)]
struct BitSet {
    inner: Vec<bool>,
    last: Option<usize>,
    len: usize,
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            inner: Vec::with_capacity(INPUT.len()),
            last: None,
            len: 0,
        }
    }

    pub fn set(&mut self, idx: usize) {
        if idx >= self.inner.len() {
            self.inner.resize(idx + 1, false);
        }
        if !self.inner[idx] {
            self.len += 1;
            self.inner[idx] = true;
        }
        self.last = Some(idx);
    }

    pub fn get(&self, idx: usize) -> bool {
        self.inner.get(idx).copied().unwrap_or(false)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

fn explore<const FOLLOW_SLOPES: bool>(
    graph: &[u8],
    width: usize,
    start: usize,
    end: usize,
) -> usize {
    let mut longest = 0;
    let mut path = BitSet::new();
    path.set(start);
    // queue of ongoing paths
    let mut queue = vec![path];
    let width = width as isize;

    'queue: while let Some(mut path) = queue.pop() {
        if path.len() > 7000 {
            continue;
        }
        let node = path.last.unwrap();
        for (d_n, &direction) in [-width, 1, width, -1].iter().enumerate() {
            if FOLLOW_SLOPES {
                match (graph[node], d_n) {
                    (b'.', _) | (b'^', 0) | (b'>', 1) | (b'v', 2) | (b'<', 3) => (),
                    _ => continue,
                }
            }

            let Some(np) = node.checked_add_signed(direction) else {
                continue;
            };

            // no backtracking
            if path.get(np) {
                continue;
            }
            // reached end, don't explore this path further
            if np == end && path.len() > longest {
                longest = path.len();
                println!("new longest: {longest}");
                continue 'queue;
            }

            if matches!(graph.get(np), Some(b'.' | b'>' | b'<' | b'v' | b'^')) {
                let mut new_path = path.clone();
                new_path.set(np);
                queue.push(new_path)
            }
        }
    }
    longest
}

pub fn part1() -> usize {
    let input = INPUT;
    let width = input.bytes().position(|b| b == b'\n').unwrap() + 1;
    let start = input.bytes().position(|b| b == b'.').unwrap();
    let end = input.bytes().rposition(|b| b == b'.').unwrap();
    explore::<true>(input.as_bytes(), width, start, end)
}

pub fn part2() -> usize {
    let input = INPUT;
    let width = input.bytes().position(|b| b == b'\n').unwrap() + 1;
    let start = input.bytes().position(|b| b == b'.').unwrap();
    let end = input.bytes().rposition(|b| b == b'.').unwrap();
    explore::<false>(input.as_bytes(), width, start, end)
}

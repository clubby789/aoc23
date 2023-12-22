use std::collections::VecDeque;

use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/21.txt");

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, u32);

const N_STEPS: u32 = 64;

impl Pos {
    pub fn successors<'a>(self, grid: &'a [u8], width: usize) -> impl Iterator<Item = Self> + 'a {
        let w = width as isize;
        [-1, 1, -w, w].into_iter().filter_map(move |diff| {
            let pos = self.0.checked_add_signed(diff)?;
            match grid.get(pos)? {
                b'.' | b'S' if self.1 < 1024 => Some(Pos(pos, self.1 + 1)),
                _ => None,
            }
        })
    }
}

fn get_reachable(input: &str) -> impl Iterator<Item = Pos> + '_ {
    let input = input.as_bytes();
    let width = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let start = input.iter().position(|&b| b == b'S').unwrap();
    let mut queue = VecDeque::with_capacity(2048);
    let mut visited = FxHashMap::with_capacity_and_hasher(input.len(), Default::default());
    queue.push_back(Pos(start, 0));
    std::iter::from_fn(move || {
        let pos = loop {
            let p = queue.pop_front()?;
            if !visited.contains_key(&p.0) {
                break p;
            }
        };
        // println!("visiting {pos:?}");
        visited.insert(pos.0, pos.1);
        let w = width as isize;
        for diff in [-1, 1, -w, w] {
            let Some(np) = pos.0.checked_add_signed(diff) else {
                continue;
            };
            // println!("{np} - {:?}", input.get(np));
            if let Some(b'.' | b'S') = input.get(np) {
                // println!("adding Pos({np}, {})", pos.1 + 1);
                queue.push_back(Pos(np, pos.1 + 1));
            }
        }
        Some(pos)
    })
}

pub fn part1() -> usize {
    get_reachable(INPUT)
        .filter(|p| p.1 <= N_STEPS && p.1 % 2 == 0)
        .count()
}

pub fn part2() -> usize {
    let reachable = get_reachable(INPUT)
        .map(|p| (p.0, p.1))
        .collect::<FxHashMap<_, _>>();
    // stolen from https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let even_corners = reachable
        .values()
        .filter(|&&v| v % 2 == 0 && v > N_STEPS + 1)
        .count();
    let odd_corners = reachable
        .values()
        .filter(|&&v| v % 2 == 1 && v > N_STEPS + 1)
        .count();

    let even_full = reachable.values().filter(|&&v| v % 2 == 0).count();
    let odd_full = reachable.values().filter(|&&v| v % 2 == 1).count();
    let n = 202300;
    // 308618359186200 too low
    // 637535765988470 too low
    let even = n * n;
    let odd = (n + 1) * (n + 1);

    odd * odd_full + even * even_full - ((n + 1) * odd_corners) + (n * even_corners)
}

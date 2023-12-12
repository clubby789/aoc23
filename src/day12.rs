use rustc_hash::FxHashMap;
use std::fmt::{Formatter, Write};

const INPUT: &str = include_str!("inputs/12.txt");

#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum SpringKind {
    Operational = b'.',
    Damaged = b'#',
    Unknown = b'?',
}

impl std::fmt::Debug for SpringKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(*self as u8 as char)
    }
}

impl SpringKind {
    pub fn maybe_damaged(&self) -> bool {
        matches!(self, SpringKind::Unknown | SpringKind::Damaged)
    }
}

enum CacheResult {
    Hit(usize),
    Miss(usize),
}

impl CacheResult {
    pub fn value(self) -> usize {
        match self {
            CacheResult::Hit(v) | CacheResult::Miss(v) => v,
        }
    }
}

fn find_places<'data>(
    cache: &mut FxHashMap<(&'data [u8], &'data [SpringKind]), usize>,
    groups: &'data [u8],
    springs: &'data [SpringKind],
) -> CacheResult {
    if let Some(cached) = cache.get(&(groups, springs)) {
        return CacheResult::Hit(*cached);
    }
    let Some((&first, rest)) = groups.split_first() else {
        return CacheResult::Miss(0);
    };

    let mut sum = 0;
    for (i, location) in springs.windows(first as usize).enumerate() {
        let prev = i.checked_sub(1).and_then(|idx| springs.get(idx));
        let next = springs.get(i + first as usize);
        if let Some(SpringKind::Damaged) = prev {
            break;
        }
        if matches!(next, Some(SpringKind::Damaged)) {
            continue;
        }

        if location.iter().all(SpringKind::maybe_damaged) {
            if rest.is_empty() {
                if springs
                    .get(i + first as usize + 1..)
                    .unwrap_or_default()
                    .iter()
                    .any(|s| matches!(s, SpringKind::Damaged))
                {
                    // ...
                } else {
                    sum += 1;
                }
            } else {
                if let Some(slice) = springs.get(i + first as usize + 1..) {
                    let amnt = find_places(cache, rest, slice);
                    if let CacheResult::Miss(val) = amnt {
                        cache.insert((rest, slice), val);
                    }
                    sum += amnt.value();
                }
            }
        }
        if matches!(location[0], SpringKind::Damaged) {
            break;
        }
    }
    CacheResult::Miss(sum)
}

fn parse_input_line(line: &str) -> (Vec<SpringKind>, Vec<u8>) {
    let (springs, groups) = line.split_once(' ').unwrap();
    let springs = springs
        .bytes()
        .map(|b| match b {
            b'.' => SpringKind::Operational,
            b'#' => SpringKind::Damaged,
            b'?' => SpringKind::Unknown,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let groups: Vec<u8> = groups.split(',').map(|n| n.parse().unwrap()).collect();
    (springs, groups)
}

pub fn part1() -> usize {
    let mut sum = 0;
    for line in INPUT.lines() {
        let (springs, groups) = parse_input_line(line);
        let mut cache = FxHashMap::default();
        let amnt = find_places(&mut cache, &groups, &springs);
        sum += amnt.value();
    }
    sum
}

pub fn part2() -> usize {
    let mut sum = 0;
    for line in INPUT.lines() {
        let (springs, groups) = parse_input_line(line);
        let springs = {
            let mut new = Vec::with_capacity(springs.len() * 5 + 5);
            for _ in 0..5 {
                new.extend(springs.as_slice());
                new.push(SpringKind::Unknown);
            }
            // remove trailing
            new.pop();
            new
        };
        let groups = groups.repeat(5);
        let mut cache = FxHashMap::default();
        let amnt = find_places(&mut cache, &groups, &springs);
        sum += amnt.value();
    }
    sum
}

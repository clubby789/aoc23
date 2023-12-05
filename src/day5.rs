use std::fmt::{Debug, Formatter};

/*
const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
*/
const INPUT: &str = include_str!("inputs/5.txt");

pub fn part1() -> usize {
    let mut parts = INPUT.split("\n\n");
    let seeds = parts.next().unwrap().split_once(": ").unwrap().1;
    let mut numbers: Vec<usize> = seeds.split(' ').map(|s| s.parse().unwrap()).collect();
    while let Some(part) = parts.next() {
        let mut new_numbers = vec![None; numbers.len()];
        let mut mappings = part.lines().skip(1);
        for m in mappings {
            let mut parts = m.split(' ');
            let dest_start = parts.next().unwrap().parse::<usize>().unwrap();
            let src_start = parts.next().unwrap().parse::<usize>().unwrap();
            let len = parts.next().unwrap().parse::<usize>().unwrap();
            let src = src_start..src_start + len;
            for (i, n) in numbers.iter().enumerate().filter(|&(_, n)| src.contains(n)) {
                new_numbers[i] = Some(n - src_start + dest_start)
            }
        }
        // replace any mapped numbers
        for (n, nn) in numbers.iter_mut().zip(&new_numbers).filter_map(|(n, nn)| {
            if let Some(nn) = nn {
                Some((n, nn))
            } else {
                None
            }
        }) {
            *n = *nn;
        }
    }
    *numbers.iter().min().unwrap()
}

pub fn part2() -> usize {
    // copyable range type
    #[derive(Copy, Clone)]
    struct Range {
        start: usize,
        end: usize,
    }

    impl Range {
        pub fn new(start: usize, end: usize) -> Self {
            debug_assert!(end >= start, "{end} < {start}");
            Self { start, end }
        }
    }
    #[derive(Copy, Clone)]
    enum Mapped {
        Yes(Range),
        No(Range),
    }

    impl Mapped {
        pub fn make_no(&mut self) {
            if let Mapped::Yes(n) = *self {
                *self = Mapped::No(n)
            }
        }
    }

    let mut parts = INPUT.split("\n\n");
    let seeds = parts.next().unwrap().split_once(": ").unwrap().1;
    let mut ranges = Vec::new();
    let mut split = seeds.split(' ');
    while let Some(lo) = split.next() {
        let len = split.next().unwrap();
        let lo: usize = lo.parse().unwrap();
        let len: usize = len.parse().unwrap();
        ranges.push(Mapped::No(Range::new(lo, lo + len)));
    }

    while let Some(part) = parts.next() {
        let (_, mappings) = part.split_once('\n').unwrap();
        for m in mappings.lines() {
            let mut parts = m.split(' ').map(|n| n.parse::<usize>().unwrap());
            let dest_start = parts.next().unwrap();
            let src_start = parts.next().unwrap();
            let len = parts.next().unwrap();
            for i in 0..ranges.len() {
                let Mapped::No(r) = ranges[i] else {
                    continue;
                };
                let starts_before = r.start < src_start;
                let starts_after = r.start > src_start + len;
                let ends_after = r.end > src_start + len;
                let ends_before = r.end < src_start;
                if starts_before && ends_before || starts_after {
                    continue;
                }
                match (starts_before, ends_after) {
                    // the mapping is fully contained by this range
                    // | RRRRRRRR | -> // RRMMMRR
                    // |   MMM    |
                    (true, true) => {
                        let lo_part = Range::new(r.start, src_start);
                        let mid = Range::new(dest_start, dest_start + len);
                        let hi_part = Range::new(src_start + len, r.end);
                        ranges[i] = Mapped::No(lo_part);
                        ranges.extend([Mapped::Yes(mid), Mapped::No(hi_part)]);
                    }
                    // the mapping extends beyond the end of the range
                    // | RRRRRRRR   | -> | RRRMMMMM
                    // |    MMMMMMM |
                    (true, false) => {
                        let lo_part = Range::new(r.start, src_start);
                        let len = r.end - src_start;
                        let hi_mapped = Range::new(dest_start, dest_start + len);
                        ranges[i] = Mapped::No(lo_part);
                        ranges.push(Mapped::Yes(hi_mapped))
                    }
                    // the mapping extends beyond the start of the range
                    // |     RRRRRRRR | -> |     MMMMRRR |
                    // | MMMMMMMMM    |
                    (false, true) => {
                        let hi_part = Range::new(src_start + len, r.end);
                        let offset = r.start - src_start;
                        let len = (src_start + len) - r.start;
                        let mapped_start = dest_start + offset;
                        let lo_mapped = Range::new(mapped_start, mapped_start + len);
                        ranges[i] = Mapped::No(hi_part);
                        ranges.push(Mapped::Yes(lo_mapped));
                    }
                    // the range is fully inside the mapping
                    // |    RRR    | -> // |    MMM    |
                    // | MMMMMMMMM |
                    (false, false) => {
                        let offset = r.start - src_start;
                        let mapped_start = dest_start + offset;
                        let mapped_end = mapped_start + (r.end - r.start);
                        ranges[i] = Mapped::Yes(Range::new(mapped_start, mapped_end));
                    }
                }
            }
        }
        for r in ranges.iter_mut() {
            r.make_no();
        }
    }
    ranges
        .into_iter()
        .map(|r| match r {
            Mapped::Yes(n) | Mapped::No(n) => n.start,
        })
        .min()
        .unwrap()
}

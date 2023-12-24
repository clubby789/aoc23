use std::{
    cell::{Cell, RefCell},
    num::NonZeroU32,
    ops::RangeInclusive,
    rc::Rc,
};

use rustc_hash::{FxHashMap, FxHashSet};

/*const INPUT: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
*/

const INPUT: &str = include_str!("inputs/22.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: u32,
    y: u32,
    // height
    z: NonZeroU32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Brick(Pos, Pos);

enum Orientation {
    // X differs
    Horizontal1,
    // Y differs
    Horizontal2,
    // Z differs
    Vertical,
}

impl Brick {
    pub fn points(&self) -> impl Iterator<Item = Pos> {
        let mut offset = 0;
        let mut points = [self.0, self.1];
        points.sort_unstable();
        let [cur, end] = points;
        let mut cur = Some(cur);
        // println!("getting points of brick {self:?}");
        std::iter::from_fn(move || {
            let ret = cur?;
            let next = if ret.x < end.x {
                Pos {
                    x: ret.x + 1,
                    ..ret
                }
            } else if ret.y < end.y {
                Pos {
                    y: ret.y + 1,
                    ..ret
                }
            } else if ret.z < end.z {
                Pos {
                    z: NonZeroU32::new(ret.z.get() + 1).unwrap(),
                    ..ret
                }
            } else {
                cur = None;
                debug_assert_eq!(ret, end);
                return Some(ret);
            };
            cur = Some(next);
            Some(ret)
        })
    }

    fn orientation(&self) -> Orientation {
        if self.0.x != self.1.x {
            Orientation::Horizontal1
        } else if self.0.y != self.1.y {
            Orientation::Horizontal2
        } else {
            debug_assert!(self.0.z != self.1.z || self.0 == self.1);
            Orientation::Vertical
        }
    }

    // inline gives 5x speedup, always 10x
    #[inline(always)]
    // If this brick is directly below and supporting `other`
    pub fn supports(&self, other: &Brick) -> bool {
        use Orientation::*;
        // Top must be exactly 1 below bottom of other
        // println!("does {self:?} support {other:?}");
        if self.1.z.get() != other.0.z.get() - 1 {
            // println!("no - z");
            return false;
        }
        let self_x = (self.0.x..=self.1.x);
        let self_y = (self.0.y..=self.1.y);
        let other_x = (other.0.x..=other.1.x);
        let other_y = (other.0.y..=other.1.y);

        fn range_overlap(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
            r1.start() <= r2.end() && r2.start() <= r1.end()
        }

        let r = match (self.orientation(), other.orientation()) {
            // x and y must match
            (Vertical, Vertical) => self.0.x == other.0.x && self.0.y == other.0.y,
            // y must match and x range must overlap
            (Horizontal1, Horizontal1) => self.0.y == other.0.y && range_overlap(&self_x, &other_x),
            // both x and y ranges must overlap
            (Horizontal1, Horizontal2) | (Horizontal2, Horizontal1) => {
                (range_overlap(&self_y, &other_y)) && (range_overlap(&self_x, &other_x))
            }
            // y must match and x range must overlap
            (Horizontal1, Vertical) | (Vertical, Horizontal1) => {
                self.0.y == other.0.y && (range_overlap(&self_x, &other_x))
            }
            // x must match and y range must overlap
            (Horizontal2, Vertical) | (Vertical, Horizontal2) => {
                self.0.x == other.0.x && (range_overlap(&self_y, &other_y))
            }
            // x must match and y range must overlap
            (Horizontal2, Horizontal2) => {
                self.0.x == other.0.x && (range_overlap(&self_y, &other_y))
            }
        };
        // println!("{r}");
        r
    }

    pub fn drop(self) -> Option<Self> {
        let (a, b) = (self.0, self.1);
        Some(Self(
            Pos {
                z: NonZeroU32::new(a.z.get() - 1)?,
                ..a
            },
            Pos {
                z: NonZeroU32::new(b.z.get() - 1)?,
                ..b
            },
        ))
    }
}

fn parse(input: &str) -> Vec<Brick> {
    INPUT
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('~').unwrap();
            let mut a = a.split(',').map(|n| n.parse().unwrap());
            let mut b = b.split(',').map(|n| n.parse().unwrap());
            Brick(
                Pos {
                    x: a.next().unwrap(),
                    y: a.next().unwrap(),
                    z: NonZeroU32::new(a.next().unwrap()).unwrap(),
                },
                Pos {
                    x: b.next().unwrap(),
                    y: b.next().unwrap(),
                    z: NonZeroU32::new(b.next().unwrap()).unwrap(),
                },
            )
        })
        .collect()
}

fn simulate_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_unstable_by_key(|b| {
        debug_assert!(b.0 <= b.1, "{b:?}");
        b.0.z
    });

    let mut fallen = Vec::with_capacity(bricks.len());
    fn do_fall(bricks: &mut Vec<Brick>, fallen: &mut Vec<Brick>) {
        let mut any_fell = false;
        for i in 0..bricks.len() {
            let b = bricks[i];
            let Some(below) = b.drop() else {
                fallen.push(bricks.remove(i));
                return;
            };
            // find other landed bricks
            let mut this_fell = true;
            for b2 in fallen.iter() {
                if b2.supports(&b) {
                    this_fell = false;
                    fallen.push(bricks.remove(i));
                    return;
                }
            }
            if this_fell {
                bricks[i] = below;
            }
        }
    }

    while !bricks.is_empty() {
        do_fall(&mut bricks, &mut fallen)
    }
    fallen
}

fn build_support_maps(bricks: &[Brick]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    // TODO: use iterator::unzip
    let support_map = bricks
        .iter()
        .map(|b| {
            let mut supporting = vec![];
            for (i, b2) in bricks.iter().enumerate() {
                if b.supports(&b2) {
                    supporting.push(i);
                }
            }
            supporting
        })
        .collect::<Vec<_>>();

    // map of bricks to everything they're supported by
    let supported_by_map = bricks
        .iter()
        .map(|b| {
            let mut supported_by = vec![];
            for (i, b2) in bricks.iter().enumerate() {
                if b2.supports(&b) {
                    supported_by.push(i);
                }
            }
            supported_by
        })
        .collect::<Vec<_>>();
    (support_map, supported_by_map)
}

pub fn part1() -> usize {
    let bricks = parse(INPUT);
    let bricks = simulate_bricks(bricks);

    let (support_map, supported_by_map) = build_support_maps(&bricks);

    let mut count = 0;

    for i in 0..bricks.len() {
        let supporting = &support_map[i];
        if supporting
            .iter()
            .all(|&sup| supported_by_map[sup].len() > 1)
        {
            count += 1;
        }
    }

    count
}

pub fn part2() -> usize {
    let bricks = parse(INPUT);
    let bricks = simulate_bricks(bricks);

    let (support_map, supported_by_map) = build_support_maps(&bricks);

    let mut total_fall = 0;
    for i in 0..bricks.len() {
        let mut fallen = FxHashSet::default();
        // we pretend the removed brick has fallen, so we need to subtract 1 from the length at the end
        fallen.insert(i);
        let mut visit = support_map[i].clone();
        while let Some(b) = visit.pop() {
            // there are no bricks suporting this one that aren't destroyed or fallen
            if supported_by_map[b]
                .iter()
                .filter(|&supporting_b| !fallen.contains(supporting_b))
                .next()
                .is_none()
            {
                fallen.insert(b);
                visit.extend(&support_map[b]);
            }
        }
        // println!("remove {i}, {fallen:?} fall");

        total_fall += (fallen.len() - 1);
    }
    total_fall
}

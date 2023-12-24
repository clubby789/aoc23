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
            r2.contains(r1.start())
                || r2.contains(r1.end())
                || r1.contains(&r2.start())
                || r1.contains(&r2.end())
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

pub fn part1() -> usize {
    let mut bricks = parse(INPUT);
    bricks.sort_unstable_by_key(|b| {
        debug_assert!(b.0 <= b.1, "{b:?}");
        b.0.z
    });

    #[derive(Clone, PartialEq, Eq)]
    struct FallingBrick {
        brick: Cell<Brick>,
        falling: Cell<bool>,
    }
    impl FallingBrick {
        pub fn is_falling(&self) -> bool {
            self.falling.get()
        }
        pub fn brick(&self) -> Brick {
            self.brick.get()
        }
    }

    let bricks: Vec<_> = bricks
        .into_iter()
        .map(|b| FallingBrick {
            brick: Cell::new(b),
            falling: Cell::new(true),
        })
        .collect();

    fn do_fall(bricks: &[FallingBrick]) -> bool {
        let mut any_fell = false;
        for b in bricks.iter().filter(|b| b.is_falling()) {
            let Some(below) = b.brick().drop() else {
                b.falling.set(false);
                continue;
            };
            // find other landed bricks
            let mut this_fell = true;
            // let below_points = below.points().collect::<FxHashSet<_>>();
            for b2 in bricks.iter().filter(|&b2| !b2.is_falling()) {
                if b2.brick().supports(&b.brick()) {
                    this_fell = false;
                    b.falling.set(false);

                    break;
                }
            }
            if this_fell {
                b.brick.set(below);
                any_fell = true;
            }
        }
        any_fell
    }

    while do_fall(&bricks) {}
    let mut count = 0;

    // for each brick b
    'bricks: for (i, b) in bricks.iter().enumerate() {
        let b = b.brick();
        // find all the bricks b supports
        let supported_by_b: Vec<_> = bricks
            .iter()
            .filter_map(|fb| {
                let fb = fb.brick();
                if b.supports(&fb) {
                    Some(fb)
                } else {
                    None
                }
            })
            .collect();
        if supported_by_b.is_empty() {
            // definitely safe
            count += 1;
            continue 'bricks;
        }
        for supported in supported_by_b {
            // find all the bricks supporting `supported`
            let supporting_this: Vec<_> = bricks
                .iter()
                .filter_map(|fb| {
                    let fb = fb.brick();
                    if fb.supports(&supported) {
                        Some(fb)
                    } else {
                        None
                    }
                })
                .collect();
            debug_assert_ne!(supporting_this.len(), 0);
            if supporting_this.len() == 1 {
                // this brick is ONLY being supported by b, so we can't remove this
                continue 'bricks;
            }
        }
        // all b's supported bricks have other supports, safe to remove
        count += 1;
    }

    count
}

pub fn part2() -> usize {
    0
}

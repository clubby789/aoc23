use std::{cell::RefCell, num::NonZeroU32, rc::Rc};

use rustc_hash::FxHashSet;

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

impl Brick {
    pub fn points(&self) -> impl Iterator<Item = Pos> {
        let mut offset = 0;
        let mut points = [self.0, self.1];
        points.sort_unstable();
        let [mut cur, end] = points;
        std::iter::from_fn(move || {
            if cur == end {
                return None;
            }
            let res = if cur.x < end.x {
                Pos {
                    x: cur.x + 1,
                    ..cur
                }
            } else if cur.y < end.y {
                Pos {
                    y: cur.y + 1,
                    ..cur
                }
            } else if cur.z < end.z {
                Pos {
                    z: NonZeroU32::new(cur.z.get() + 1).unwrap(),
                    ..cur
                }
            } else {
                unreachable!()
            };
            cur = res;
            Some(res)
        })
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
            let (a, b) = l.split_once("~").unwrap();
            let mut a = a.split(",").map(|n| n.parse().unwrap());
            let mut b = b.split(",").map(|n| n.parse().unwrap());
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

struct LandedBrick {
    brick: Brick,
    supporting: RefCell<Vec<Rc<Self>>>,
}

impl LandedBrick {
    pub fn bricks(self: Rc<Self>) -> impl Iterator<Item = Rc<Self>> {
        let mut stack = vec![self];
        std::iter::from_fn(move || {
            let cur = stack.pop()?;
            stack.extend(cur.supporting.borrow().iter().cloned());
            Some(cur)
        })
    }
}

pub fn part1() -> usize {
    let mut bricks = parse(INPUT);
    // Lowest bricks are at the bottom
    bricks.sort_unstable_by_key(|b| b.0.z.min(b.1.z));
    let mut landed: Vec<_> = Vec::with_capacity(bricks.len());

    fn do_fall(bricks: &mut Vec<Brick>, landed: &mut Vec<Rc<LandedBrick>>) {
        println!("falling/landed: {}/{}", bricks.len(), landed.len());
        for i in (0..bricks.len()).rev() {
            if let Some(below) = bricks[i].drop() {
                let below_points = below.points().collect::<FxHashSet<_>>();
                if let Some(supporting) = landed.iter_mut().find(|landed| {
                    let points = landed.brick.points().collect::<FxHashSet<_>>();
                    points.intersection(&below_points).next().is_some()
                }) {
                    supporting
                        .supporting
                        .borrow_mut()
                        .push(Rc::new(LandedBrick {
                            brick: bricks.remove(i),
                            supporting: RefCell::new(Vec::new()),
                        }));
                } else {
                    bricks[i] = below;
                }
            } else {
                landed.push(Rc::new(LandedBrick {
                    brick: bricks.remove(i),
                    supporting: RefCell::new(Vec::new()),
                }));
            }
        }
    }

    while !bricks.is_empty() {
        do_fall(&mut bricks, &mut landed);
    }
    let mut count = 0;

    let count = landed
        .iter()
        .flat_map(|l| l.clone().bricks())
        .filter(|lb| lb.supporting.borrow().is_empty())
        .count();
    // 545 too high
    count
}

pub fn part2() -> usize {
    0
}

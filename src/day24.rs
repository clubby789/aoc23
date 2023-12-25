use std::ops::{Mul, RangeInclusive};

use z3::{
    ast::{self, Ast, Float, Int, Real},
    SatResult, Solver,
};

const INPUT: &str = include_str!("inputs/24.txt");
const TEST_RANGE: RangeInclusive<i64> = 200000000000000..=400000000000000;

// pos + t * D = pos2 + s * D2
// t = ((pos2 - pos) x D2) / (D x D2)

#[derive(Debug, PartialEq, Eq)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl std::ops::Mul<i64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

pub fn part1() -> usize {
    let hailstones = INPUT
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let mut p = p.split(',').map(|p| p.trim().parse().unwrap());
            let mut v = v.split(',').map(|p| p.trim().parse().unwrap());
            (
                Vector3 {
                    x: p.next().unwrap(),
                    y: p.next().unwrap(),
                    z: p.next().unwrap(),
                },
                Vector3 {
                    x: v.next().unwrap(),
                    y: v.next().unwrap(),
                    z: v.next().unwrap(),
                },
            )
        })
        .collect::<Vec<(_, _)>>();
    let mut collide = 0;
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    for (i, (pos_a, vel_a)) in hailstones.iter().enumerate() {
        println!("checking {i}");
        for (pos_b, vel_b) in hailstones.iter().skip(i + 1) {
            let solver = z3::Solver::new(&ctx);
            let pos_a_x = Int::from_i64(&ctx, pos_a.x);
            let pos_a_y = Int::from_i64(&ctx, pos_a.y);
            let vel_a_x = Int::from_i64(&ctx, vel_a.x);
            let vel_a_y = Int::from_i64(&ctx, vel_a.y);

            let pos_b_x = Int::from_i64(&ctx, pos_b.x);
            let pos_b_y = Int::from_i64(&ctx, pos_b.y);
            let vel_b_x = Int::from_i64(&ctx, vel_b.x);
            let vel_b_y = Int::from_i64(&ctx, vel_b.y);

            let time_a = Real::fresh_const(&ctx, "time_a");
            let time_b = Real::fresh_const(&ctx, "time_b");
            let line_a_x = pos_a_x.to_real() + vel_a_x.to_real() * &time_a;
            let line_a_y = pos_a_y.to_real() + vel_a_y.to_real() * &time_a;

            let line_b_x = pos_b_x.to_real() + vel_b_x.to_real() * &time_b;
            let line_b_y = pos_b_y.to_real() + vel_b_y.to_real() * &time_b;

            solver.assert(&time_a.ge(&Real::from_real(&ctx, 0, 1)));
            solver.assert(&time_b.ge(&Real::from_real(&ctx, 0, 1)));
            solver.assert(&line_a_x._eq(&line_b_x));
            solver.assert(&line_a_y._eq(&line_b_y));
            solver.assert(&line_a_x.ge(&Real::from_int(&Int::from_i64(&ctx, *TEST_RANGE.start()))));
            solver.assert(&line_a_x.le(&Real::from_int(&Int::from_i64(&ctx, *TEST_RANGE.end()))));
            solver.assert(&line_a_y.ge(&Real::from_int(&Int::from_i64(&ctx, *TEST_RANGE.start()))));
            solver.assert(&line_a_y.le(&Real::from_int(&Int::from_i64(&ctx, *TEST_RANGE.end()))));
            if let z3::SatResult::Sat = solver.check() {
                collide += 1;
            }
        }
    }
    collide
}

pub fn part2() -> usize {
    let hailstones = INPUT
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let mut p = p.split(',').map(|p| p.trim().parse().unwrap());
            let mut v = v.split(',').map(|p| p.trim().parse().unwrap());
            (
                Vector3 {
                    x: p.next().unwrap(),
                    y: p.next().unwrap(),
                    z: p.next().unwrap(),
                },
                Vector3 {
                    x: v.next().unwrap(),
                    y: v.next().unwrap(),
                    z: v.next().unwrap(),
                },
            )
        })
        .collect::<Vec<(_, _)>>();
    let mut collide = 0;
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let s = Solver::new(&ctx);

    let p_x = Real::new_const(&ctx, "p_x");
    let p_y = Real::new_const(&ctx, "p_y");
    let p_z = Real::new_const(&ctx, "p_z");
    let v_x = Real::new_const(&ctx, "v_x");
    let v_y = Real::new_const(&ctx, "v_y");
    let v_z = Real::new_const(&ctx, "v_z");
    // apparently solving for 3 is fine
    for (i, (pos, vel)) in hailstones.iter().enumerate().take(3) {
        let time = Real::fresh_const(&ctx, "time");
        let thrown_x = &p_x + &v_x * &time;
        let thrown_y = &p_y + &v_y * &time;
        let thrown_z = &p_z + &v_z * &time;

        let this_x =
            Int::from_i64(&ctx, pos.x).to_real() + Int::from_i64(&ctx, vel.x).to_real() * &time;
        let this_y =
            Int::from_i64(&ctx, pos.y).to_real() + Int::from_i64(&ctx, vel.y).to_real() * &time;
        let this_z =
            Int::from_i64(&ctx, pos.z).to_real() + Int::from_i64(&ctx, vel.z).to_real() * &time;

        s.assert(&thrown_x._eq(&this_x));
        s.assert(&thrown_y._eq(&this_y));
        s.assert(&thrown_z._eq(&this_z));
    }
    if let SatResult::Sat = s.check() {
        let m = s.get_model().unwrap();
        [p_x, p_y, p_z]
            .iter()
            .map(|p| {
                let Some((n, 1)) = m.get_const_interp(p).and_then(|v| v.as_real()) else {
                    unreachable!();
                };
                n
            })
            .sum::<i64>() as usize
    } else {
        panic!("unsat")
    }
}

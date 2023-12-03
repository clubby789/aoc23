use seq_macro::seq;
use std::time::{Duration, Instant};
seq!(N in 1..=3 {
    #[allow(unused)]
    mod day~N;
});

fn timeit<F, U>(f: F) -> (Duration, U)
where
    F: Fn() -> U,
{
    // warm up for 2s
    let now = Instant::now();
    while now.elapsed() < Duration::from_secs(2) {
        std::hint::black_box(f());
    }
    let now = Instant::now();
    let mut iters = 0;
    while now.elapsed() < Duration::from_secs(2) {
        std::hint::black_box(f());
        iters += 1;
    }
    let avg = now.elapsed() / iters;
    let ret = std::hint::black_box(f());
    (avg, ret)
}

seq! {
    N in 1..=3 {
        static FUNCS: &[(fn() -> usize, fn() -> usize)] = &[
            #(
                (day~N::part1 as _, day~N::part2 as _),
            )*
        ];
    }
}

fn main() {
    let (f1, f2) = FUNCS.last().unwrap();
    if std::env::var("TIMEIT").is_ok() {
        let (t1, res) = timeit(f1);
        println!("Solved part 1 in {t1:?} - {res}");
        let (t2, res) = timeit(f2);
        println!("Solved part 2 in {t2:?} - {res}");
    } else {
        println!("Part 1 - {}", f1());
        println!("Part 2 - {}", f2());
    }
}

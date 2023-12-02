use seq_macro::seq;
use std::time::{Duration, Instant};
seq!(N in 1..=2 {
    #[allow(unused)]
    mod day~N;
});

fn timeit<F, U>(f: F) -> (Duration, U)
where
    F: Fn() -> U,
{
    // warm up
    let now = Instant::now();
    while now.elapsed() < Duration::from_secs(2) {
        std::hint::black_box(f());
    }
    let now = Instant::now();
    let ret = std::hint::black_box(f());
    (now.elapsed(), ret)
}

fn main() {
    let (t1, res) = timeit(day2::part1);
    println!("Solved part 1 in {t1:?} - {res}");
    let (t2, res) = timeit(day2::part2);
    println!("Solved part 2 in {t2:?} - {res}");
}

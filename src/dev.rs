// temporary nodule

use std::time::Instant;

pub fn measure<F>(name: &str, mut f: F)
where
    F: FnMut() -> (),
{
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();

    println!(
        "{} took {}ms",
        name,
        (elapsed.as_secs() * 1000000 + elapsed.subsec_micros() as u64) as f64 / 1000.0
    );
}
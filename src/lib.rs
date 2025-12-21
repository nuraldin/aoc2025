pub mod day08;
pub mod day09;

#[macro_export]
macro_rules! time_it {
    ($label:expr, $expr:expr) => {{
        let start = std::time::Instant::now();
        let result = $expr;
        let elapsed = start.elapsed();

        println!("{label}: {elapsed:?}", label = $label, elapsed = elapsed);
        result
    }};
}

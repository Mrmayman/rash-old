use crate::interpreter::Interpreter;

mod interpreter;
mod sprite;
mod code_thread;
mod machine_state;

mod instructions {
    pub mod base;
    pub mod operator;
}

fn main() {
    let mut interpreter: Interpreter = Interpreter::new();
    let starting_time = std::time::Instant::now();
    interpreter.load();
    interpreter.run();
    println!("{} seconds elapsed, 1 million iterations", starting_time.elapsed().as_secs_f64());

    let starting_time = std::time::Instant::now();
    let pi = calculate_pi();
    println!("pi = {}.\n{} seconds elapsed, 1 million iterations", pi, starting_time.elapsed().as_secs_f64());
}

fn calculate_pi() -> f64 {
    let mut pi = 0.0;
    let n = 4.0;
    let mut d = 1.0;
    for i in 0..10_000_000 {
        let a = (2 * (i % 2) - 1) as f64;
        pi += a * n / d;
        d += 2.0;
    }
    pi
}

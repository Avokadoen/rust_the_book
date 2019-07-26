use std::io;
use fibonacci_number::fibo;

extern crate time;
use time::PreciseTime;


fn main() {
    let fibo_number = loop{
        println!("please write your fibonacci number");

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        let fibo_number: u64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break fibo_number;
    };

    println!("fibo {}: {}", fibo_number, fibo::fibonacci_number(fibo_number));
    println!("DIY benchmark: {}ns", time(fibo_number));

    println!("fibo float {}: {}", fibo_number, fibo::fibonacci_number_float(fibo_number));
    println!("DIY benchmark: {}ns", time(fibo_number));

       println!("fibo formula {}: {}", fibo_number, fibo::fibonacci_number_formula(fibo_number as f64));
    println!("DIY benchmark: {}ns", time(fibo_number));
}


// Run function and return result with nano seconds duration
fn time(number: u64) -> (i64) {
    let start = PreciseTime::now();
    fibo::fibonacci_number(number);
    let end = PreciseTime::now();

    start.to(end).num_nanoseconds().expect("Benchmark iter took greater than 2^63 nanoseconds")
}
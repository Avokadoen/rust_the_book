use std::io;

fn main() {
    let fibo_number = loop{
        println!("please write your fibonacci number");

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        let fibo_number: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break fibo_number;
    };

    println!("fibo {}: {}", fibo_number, fibonacci_number(fibo_number));
}

fn fibonacci_number(n: u32) -> u32{
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut iter_count: u32 = 2;
    let mut fibo_number: u32 = 1;
    let mut prev_fibo_number: u32 = 1;
    while iter_count < n {
        fibo_number = fibo_number + prev_fibo_number;
        prev_fibo_number = fibo_number - prev_fibo_number;
        iter_count += 1;
    }

    return fibo_number;
}

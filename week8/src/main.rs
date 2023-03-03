use std::io;

fn main() {
    println!("Please enter a positive integer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let factorial = factorial(n);

    println!("The factorial of {} is {}", n, factorial);
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
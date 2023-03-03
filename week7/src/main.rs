extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    println!("Your new password is: {}", password);
}

use std::collections::HashMap;

pub fn most_frequent(numbers: &str) {
    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut frequency = HashMap::new();

    for &number in &numbers {
        *frequency.entry(number).or_insert(0) += 1;
    }

    let mut max_number = numbers[0];
    let mut max_frequency = 0;

    for (&number, &count) in frequency.iter() {
        if count > max_frequency {
            max_frequency = count;
            max_number = number;
        }
    }

    println!("most frequent numbers: {:?}", max_number);
    
}







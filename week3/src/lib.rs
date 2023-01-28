pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "Marco".to_string()
    }
}

pub fn my_sort(input: &str) {
    let mut numbers: Vec<i32> = input
    .split_whitespace()
    .map(|s| s.parse().expect("Failed to parse input"))
    .collect();

    numbers.sort();

    println!("Sorted numbers: {:?}", numbers);
}


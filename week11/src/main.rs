use std::io;

fn caesar_cipher(text: &str, shift: u8, encode: bool) -> String {
    text.chars()
        .map(|c| {
            let base = if c.is_ascii_lowercase() {
                b'a'
            } else if c.is_ascii_uppercase() {
                b'A'
            } else {
                return c;
            };

            let offset = c as u8 - base;
            let new_offset = if encode {
                (offset + shift) % 26
            } else {
                (offset as i8 - shift as i8).rem_euclid(26) as u8
            };

            (base + new_offset) as char
        })
        .collect()
}

fn main() {
    println!("Enter the text to encode/decode:");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read input");

    println!("Enter the shift value:");
    let mut shift = String::new();
    io::stdin().read_line(&mut shift).expect("Failed to read input");
    let shift: u8 = shift.trim().parse().expect("Please enter a valid positive integer");

    let encoded = caesar_cipher(&text, shift, true);
    let decoded = caesar_cipher(&encoded, shift, false);

    println!("Original text: {}", text);
    println!("Encoded text: {}", encoded);
    println!("Decoded text: {}", decoded);
}

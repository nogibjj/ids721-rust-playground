use qrcode::{QrCode, Version, EcLevel};
use std::io::{self, Write};

fn main() {
    println!("Please enter the text to encode:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let code = QrCode::with_version(input.as_bytes(), Version::Normal(4), EcLevel::L).unwrap();
    let image = code.render::<char>().quiet_zone(false).build();

    let mut file = std::fs::File::create("qrcode.txt").unwrap();
    write!(&mut file, "{}", image).unwrap();

    println!("QR code saved to qrcode.txt");
}

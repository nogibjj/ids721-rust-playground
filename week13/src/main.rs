const SIZE: usize = 32;

fn is_star(x: usize, y: usize) -> bool {
    (x & y) == 0
}

fn main() {
    for y in 0..SIZE {
        for x in 0..SIZE {
            if is_star(x, y) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

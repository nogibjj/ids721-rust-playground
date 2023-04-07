use rand::Rng;
use std::iter;

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

fn landscape_line(prob_tree: f64) -> String {
    let mut rng = rand::thread_rng();
    iter::repeat_with(|| {
        if rng.gen_bool(prob_tree) {
            "🌲"
        } else {
            "  "
        }
    })
    .take(WIDTH)
    .collect()
}

fn main() {
    let prob_tree = 0.1;

    for _ in 0..HEIGHT {
        let line = landscape_line(prob_tree);
        println!("{}", line);
    }
}

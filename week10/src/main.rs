use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

const WIDTH: usize = 21;
const HEIGHT: usize = 11;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Path,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if *self == Cell::Wall { "██" } else { "  " })
    }
}

fn depth_first_search(maze: &mut [[Cell; WIDTH]; HEIGHT], x: usize, y: usize) {
    let mut rng = thread_rng();
    let dirs: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut neighbors = dirs.to_vec();
    neighbors.shuffle(&mut rng);

    for &(dx, dy) in &neighbors {
        let nx = match x.checked_add((2 * dx) as usize) {
            Some(value) => value,
            None => continue,
        };
        let ny = match y.checked_add((2 * dy) as usize) {
            Some(value) => value,
            None => continue,
        };

        if nx >= 1 && nx < WIDTH - 1 && ny >= 1 && ny < HEIGHT - 1 {
            if maze[ny][nx] == Cell::Wall {
                maze[y + dy as usize][x + dx as usize] = Cell::Path;
                maze[ny][nx] = Cell::Path;
                depth_first_search(maze, nx, ny);
            }
        }
    }
}

fn main() {
    let mut maze = [[Cell::Wall; WIDTH]; HEIGHT];
    maze[1][1] = Cell::Path;
    depth_first_search(&mut maze, 1, 1);

    for row in &maze {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

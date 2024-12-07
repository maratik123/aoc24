use common::input;
use day06_lib::{load_input, visited_points, DAY};

fn main() {
    let (map, guard_pos) = load_input(input(DAY, ""));
    println!("{}", visited_points((&map, guard_pos)).len());
}

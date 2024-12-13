use std::fs;

mod part1;
mod part2;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).expect(&format!("{path} file not found"));

    let sol1 = part1::solve(&input);
    println!("1: {sol1}");

    let sol2 = part2::solve(&input);
    println!("2: {sol2}");
}

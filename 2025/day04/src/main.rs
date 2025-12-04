use utils::get_input;

mod common;
mod part_one;
mod part_two;

fn main() {
    // let test = include_str!("test.txt");
    let data = get_input(2025, 4);

    let p1 = part_one::solve_part_one(&data);

    println!("{p1}");

    let p2 = part_two::solve_part_two(&data);

    println!("{p2}");
}

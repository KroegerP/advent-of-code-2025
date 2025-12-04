use utils::get_input;

mod common;
mod part_one;
mod part_two;

fn main() {
    let test = String::from(include_str!("test.txt"));
    // let input = get_input(2025, 2);

    // let ans = part_one::part_one(&input);

    // println!("{ans}");

    let p2 = part_two::part_two(&test);

    println!("{p2}");
}

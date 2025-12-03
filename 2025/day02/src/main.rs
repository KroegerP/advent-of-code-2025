use utils::get_input;

mod part_one;

fn main() {
    // let input = String::from(include_str!("test.txt"));
    let input = get_input(2025, 2);

    let ans = part_one::part_one(input);

    println!("{ans}");
}

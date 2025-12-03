use utils::get_input;

mod part_one;
mod part_two;

fn main() {
    let input = get_input(2025, 3);

    let ans: u32 = part_one::solve_part_one(&input);

    println!("{}", ans);

    let data = include_str!("test.txt");

    let ans: u64 = part_two::solve_part_two(&input);

    // 16882670420197 too low
    // 168680094117407 too low

    println!("{}", ans);
}

use utils::get_input;

mod part_one;
mod part_two;

fn main() {
    let input = get_input(2025, 3);

    let _ans: u32 = part_one::solve_part_one(&input);

    let ans: u64 = part_two::solve_part_two(&input);

    println!("{}", ans);
}

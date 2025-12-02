use utils::get_input;

mod part_one;
mod part_two;

fn main() {
    let data = get_input(2025, 1);

    part_one::solve_part_one();

    part_two::solve_part_two(data);
}

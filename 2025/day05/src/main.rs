use utils::get_input;

mod part_one;
mod part_two;

fn main() {
    let _test = include_str!("test.txt");
    let _data = get_input(2025, 5);

    part_one::solve_part_one(&_data);

    part_two::solve_part_two(&_data)
}

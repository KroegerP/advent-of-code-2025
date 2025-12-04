use crate::common;

pub fn solve_part_one(input: &str) -> u32 {
    let global_map = common::build_matrix(input);

    input.split_whitespace().enumerate().fold(0, |acc, (x, f)| {
        let valid_count = f.chars().enumerate().fold(0, |acc, (y, c)| {
            if c != '@' {
                return acc;
            }

            let neighbor_chars = common::get_neighbors(&global_map, x as isize, y as isize);

            if neighbor_chars.len() < 4 {
                acc + 1
            } else {
                acc
            }
        });

        acc + valid_count
    })
}

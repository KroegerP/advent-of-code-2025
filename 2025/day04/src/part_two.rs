use crate::common;

fn pass_though_map(input_map: Vec<Vec<char>>) -> (u32, Vec<Vec<char>>) {
    let mut cur_map = input_map.clone();

    let valid_count = input_map.iter().enumerate().fold(0, |acc, (x, f)| {
        let valid_count = f.iter().enumerate().fold(0, |acc, (y, &c)| {
            if c != '@' {
                return acc;
            }

            let neighbor_chars = common::get_neighbors(&input_map, x as isize, y as isize);

            if neighbor_chars.len() < 4 {
                cur_map[x][y] = '.';
                acc + 1
            } else {
                acc
            }
        });

        acc + valid_count
    });

    (valid_count, cur_map)
}

pub fn solve_part_two(input: &str) -> u32 {
    let global_map = common::build_matrix(input);

    let mut total_count = 0;

    let mut new_map = global_map;

    loop {
        let (res_count, updated_map) = pass_though_map(new_map);

        if res_count == 0 {
            break;
        }

        new_map = updated_map;
        total_count += res_count;
    }

    total_count
}

use crate::common;

fn find_invalids(f_id: u64, l_id: u64) -> Vec<u64> {
    let num_range = f_id..l_id + 1;

    let invalid_ids: Vec<u64> = num_range
        .filter(|&f| {
            let f_string = f.to_string();
            let f_size = f_string.len();
            let mut is_valid = true;

            let mut index = 0;

            let middle_idx = if f_size % 2 == 0 {
                f_size / 2
            } else {
                f_size / 2 + 1
            };

            while index < middle_idx {
                if f_string.chars().nth(index) != f_string.chars().nth(index + middle_idx) {
                    is_valid = false;
                    break;
                }

                index += 1;
            }

            is_valid
        })
        .collect();

    return invalid_ids;
}

pub fn part_two(input_data: &str) -> u64 {
    let mut sum_of_invalids: u64 = 0;

    input_data
        .split(",")
        .map(common::str_to_num_range)
        .for_each(|(f_id, l_id)| {
            let invalid_nums: u64 = find_invalids(f_id, l_id).iter().sum();

            sum_of_invalids += invalid_nums;
        });

    sum_of_invalids
}

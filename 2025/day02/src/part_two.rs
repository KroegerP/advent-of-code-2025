use crate::common;

fn get_pattern_size(input_data: &str) -> isize {
    let (first_digit, rest) = input_data.split_at(1);

    let next_matching_idx = rest.chars().position(|c| c.to_string() == first_digit);

    let position: isize = match next_matching_idx {
        Some(u) => u as isize,
        None => -2,
    };

    position + 1
}

fn find_invalids(f_id: u64, l_id: u64) -> Vec<u64> {
    let num_range = f_id..l_id + 1;

    let invalid_ids: Vec<u64> = num_range
        .filter(|&f| {
            let f_string = f.to_string();
            let pattern_size = get_pattern_size(&f_string);

            if pattern_size < 0 {
                return false;
            }

            let str_vect: Vec<char> = f_string.chars().collect();

            let mut chunks = str_vect.chunks(pattern_size as usize);

            if let Some(first_chunk) = chunks.next() {
                // Check if all remaining chunks are equal to the first one
                let res = chunks.all(|chunk| chunk == first_chunk);

                res
            } else {
                false
            }
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
            let invalid_nums_vec = find_invalids(f_id, l_id);

            println!("{}-{} {:?}", f_id, l_id, invalid_nums_vec);

            let res: u64 = invalid_nums_vec.iter().sum();

            sum_of_invalids += res;
        });

    sum_of_invalids
}

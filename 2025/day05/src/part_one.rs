fn str_to_num_range(id_range: &str) -> (u64, u64) {
    let mut split_str = id_range.split("-");

    let first_id = split_str.next().unwrap().trim().parse().unwrap();
    let last_id = split_str.next().unwrap().trim().parse().unwrap();

    (first_id, last_id)
}

fn get_ranges(range_list: &str) -> Vec<(u64, u64)> {
    range_list
        .split_whitespace()
        .map(str_to_num_range)
        .collect()
}

pub fn solve_part_one(input: &str) {
    let mut split_lists = input.split("\n\n");

    let valid_ids = get_ranges(split_lists.next().unwrap());

    let id_count: u64 = split_lists
        .next()
        .unwrap()
        .split_whitespace()
        .fold(0, |acc, f| {
            let cur: u64 = f.parse().unwrap();

            if valid_ids
                .iter()
                .any(|&range| cur >= range.0 && range.1 >= cur)
            {
                acc + 1
            } else {
                acc
            }
        });

    println!("{id_count:?}");
}

use std::{collections::HashSet, ops::Range, thread};

fn str_to_num_range(id_range: &str) -> (u64, u64) {
    let mut split_str = id_range.split("-");

    let first_id = split_str.next().unwrap().trim().parse().unwrap();
    let last_id = split_str.next().unwrap().trim().parse().unwrap();

    (first_id, last_id)
}

fn expand_num_range(id_tuple: (u64, u64)) -> Range<u64> {
    id_tuple.0..id_tuple.1 + 1
}

fn build_invalid_set_threaded(range_list: &str) -> HashSet<u64> {
    let mut handles = Vec::new();

    for range_str in range_list.split_whitespace() {
        let local_str = range_str.to_string();
        let handle = thread::spawn(move || {
            expand_num_range(str_to_num_range(&local_str))
                .into_iter()
                .collect::<HashSet<u64>>()
        });

        handles.push(handle);
    }

    let mut final_set = HashSet::new();

    for handle in handles {
        let temp_set = handle.join().unwrap();

        final_set.extend(temp_set);
    }

    final_set
}

fn build_invalid_set(range_list: &str) -> usize {
    let mut global_values: HashSet<u64> = HashSet::new();

    range_list
        .split_whitespace()
        .map(str_to_num_range)
        .map(expand_num_range)
        .for_each(|f| {
            let range = f.into_iter().collect::<HashSet<u64>>();

            global_values.extend(range);
        });

    global_values.len()
}

pub fn solve_part_two(input: &str) {
    let mut split_lists = input.split("\n\n");

    let valid_id_count = build_invalid_set(split_lists.next().unwrap());

    println!("{valid_id_count:?}");
}

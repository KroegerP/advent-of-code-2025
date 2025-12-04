pub fn str_to_num_range(id_range: &str) -> (u64, u64) {
    let mut split_str = id_range.split("-");

    let first_id = split_str.next().unwrap().trim().parse().unwrap();
    let last_id = split_str.next().unwrap().trim().parse().unwrap();

    (first_id, last_id)
}

fn concat_numbers(nums: &[u32]) -> u64 {
    nums.iter().fold(0u64, |acc, &n| {
        let digits = ((n as f64).log10().floor() as u32 + 1).max(1);
        acc * 10u64.pow(digits) + n as u64
    })
}

fn initialize_acc(input: &str, n: usize) -> (u32, &str) {
    let range_1 = input.split_at(input.len() - n);

    let max_1 = range_1.0.chars().max().unwrap();

    let max_1_idx = input.find(max_1).unwrap();

    let range_2 = input.split_at(max_1_idx + 1);

    (max_1.to_digit(10).unwrap(), range_2.1)
}

fn max_n_numbers(input: &str, n: usize) -> u64 {
    let mut acc: Vec<u32> = vec![];

    let mut new_search = input;

    while acc.len() < n {
        let (max_val, new_search_str) = initialize_acc(new_search, n - acc.len() - 1);

        acc.push(max_val);
        new_search = new_search_str;
    }

    concat_numbers(&acc)
}

pub fn solve_part_two(input: &str) -> u64 {
    input.split_whitespace().map(|f| max_n_numbers(f, 12)).sum()
}

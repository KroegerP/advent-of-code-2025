fn concat_numbers(nums: &[u32]) -> u64 {
    nums.iter().fold(0u64, |acc, &n| {
        let digits = ((n as f64).log10().floor() as u32 + 1).max(1);
        acc * 10u64.pow(digits) + n as u64
    })
}

fn initialize_acc(input: &str, n: usize) -> Vec<u32> {
    let range_1 = input.split_at(input.len() - n);

    let max_1 = range_1.0.chars().max().unwrap();

    let max_1_idx = input.find(max_1).unwrap();

    println!("Max Value {max_1}");

    let range_2 = input.split_at(max_1_idx + 1);

    let mut vec_range: Vec<u32> = range_2.1.chars().map(|f| f.to_digit(10).unwrap()).collect();

    vec_range.insert(0, max_1.to_digit(10).unwrap());

    vec_range
}

fn max_n_numbers(input: &str, n: usize) -> u64 {
    let mut acc = initialize_acc(input, n);

    while acc.len() > n {
        let min = acc.iter().min().unwrap();

        let position = acc.iter().position(|v| v == min).unwrap();

        acc.remove(position);
    }

    let res = concat_numbers(&acc);

    println!("{res}");

    res
}

pub fn solve_part_two(input: &str) -> u64 {
    input.split_whitespace().map(|f| max_n_numbers(f, 12)).sum()
}

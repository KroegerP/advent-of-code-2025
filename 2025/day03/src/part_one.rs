pub fn solve_part_one(input: &str) -> u32 {
    input
        .split_whitespace()
        .map(|f| {
            let range_1 = f.split_at(f.len() - 1);

            let max_1 = range_1
                .0
                .chars()
                // .map(|f| f.to_digit(10).unwrap())
                .max()
                .unwrap();

            let max_1_idx = f.find(max_1).unwrap();

            let range_2 = f.split_at(max_1_idx + 1);

            let max_2 = range_2
                .1
                .chars()
                .map(|f| f.to_digit(10).unwrap())
                .max()
                .unwrap();

            let comb = format!("{}{}", max_1, max_2);

            let res: u32 = comb.parse().unwrap();

            res
        })
        .sum()
}

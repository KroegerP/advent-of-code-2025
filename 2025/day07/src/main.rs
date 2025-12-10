use std::collections::HashSet;

use utils::get_input;

fn get_start(row: &str) -> usize {
    row.char_indices().find(|fid| fid.1 == 'S').unwrap().0
}

fn get_carrot_indices(row: &str) -> HashSet<usize> {
    row.char_indices()
        .filter(|fid| fid.1 == '^')
        .map(|f| f.0)
        .collect()
}

fn get_new_splits(
    beam_indices: &HashSet<usize>,
    carrot_indices: HashSet<usize>,
    max_size: usize,
) -> (HashSet<usize>, usize) {
    let beam_splits: HashSet<usize> = beam_indices
        .intersection(&carrot_indices)
        .copied()
        .collect();

    println!("Hit Carrots: {:?}", beam_splits);

    let mut new_indices: HashSet<usize> = HashSet::new();
    let new_count = beam_splits.len();

    for i in beam_splits {
        if i < max_size + 1 {
            new_indices.insert(i + 1);
        }
        if i > 0 {
            new_indices.insert(i - 1);
        }
    }

    (new_indices, new_count)
}

fn main() {
    // let data = include_str!("test.txt");
    let data = get_input(2025, 7);

    let mut beam_indices: HashSet<usize> = HashSet::new();

    let split_count = data
        .split_whitespace()
        .enumerate()
        .fold(0, |acc, (index, line)| {
            if index == 0 {
                beam_indices.insert(get_start(line));
                return acc;
            }

            let carrots = get_carrot_indices(line);

            if carrots.len() == 0 {
                return acc;
            }

            println!("CUR SPLITS {:?}", beam_indices);
            println!("CARROTS {:?}", carrots);

            let (new_splits, new_count) = get_new_splits(&beam_indices, carrots, line.len());

            println!("NEW SPLITS {:?}", new_splits);

            beam_indices = new_splits;

            println!("");

            acc + new_count
        });

    println!("{split_count}");
}

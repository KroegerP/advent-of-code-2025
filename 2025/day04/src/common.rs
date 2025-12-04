pub fn build_matrix(input: &str) -> Vec<Vec<char>> {
    let split_input: Vec<&str> = input.split_whitespace().collect();

    split_input.iter().map(|&f| f.chars().collect()).collect()
}

pub fn get_neighbors(global_map: &Vec<Vec<char>>, x: isize, y: isize) -> Vec<char> {
    let mut neighbors: Vec<char> = Vec::new();

    for (x_coord, y_coord) in [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ] {
        if x_coord < 0 || x_coord == global_map.len() as isize {
            continue;
        }

        if y_coord < 0 || y_coord == global_map[0].len() as isize {
            continue;
        }

        let char = global_map[x_coord as usize][y_coord as usize];

        if char == '@' {
            neighbors.push(char);
        }
    }

    neighbors
}

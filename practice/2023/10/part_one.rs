const DATA: &str = include_str!("input.txt");

fn get_move_dur(direction_char: char) -> (i8, i8) {
    match direction_char {
        '|' => (-1, 1),
        '-' => (0, -1),
        'L' => (1, 0),
        'J' => (-1, 0),
        '7' => (-1, 0),
        'F' => (-1, 0),
        '.' => (-1, 0),
        other => panic!("Unknown character {other}"),
    }
}

fn get_starting_pos() -> (usize, usize) {
    let mut starting_position = (0, 0);

    let (row, row_contents) = DATA
        .split_whitespace()
        .enumerate()
        .find(|(_i, f)| f.contains("S"))
        .unwrap_or_default();

    row_contents.chars().enumerate().for_each(|(col, f)| {
        if f == 'S' {
            starting_position = (row, col)
        }
    });

    return starting_position;
}

fn main() {
    let starting_pos = get_starting_pos();

    println!("Starting at: row {} col {}", starting_pos.0, starting_pos.1);
}

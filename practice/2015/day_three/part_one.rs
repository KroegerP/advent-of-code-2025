use std::collections::HashSet;

const DATA: &str = include_str!("input.txt");


fn make_move(direction: char) -> (i8, i8) {
    match direction {
        '^' => (0, 1),
        'v' => (0, -1),
        '>' => (1, 0),
        '<' => (-1, 0),
        other => panic!("Unknown character {other}")
    }
}

fn main() {
    let mut position: (i8, i8) = (0, 0);
    let mut a: HashSet<(i8, i8)> = vec![position].into_iter().collect();

    DATA.chars().for_each(|f| {
        let new_pos = make_move(f);

        position = (position.0 + new_pos.0, position.1 + new_pos.1);

        a.insert(position.clone());
    });

    println!("{}", a.len())
}

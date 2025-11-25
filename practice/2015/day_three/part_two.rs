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
    let mut santa_position: (i8, i8) = (0, 0);
    let mut robot_position: (i8, i8) = (0, 0);

    let mut santa_visits: HashSet<(i8, i8)> = vec![].into_iter().collect();
    let mut robot_visits: HashSet<(i8, i8)> = vec![].into_iter().collect();

    DATA.chars().collect::<Vec<char>>().chunks(2).for_each(
        |f| {
            println!("{} {}", f[0], f[1]);
            let santa_move = make_move(f[0]);
            let robot_move = make_move(f[1]);

            santa_position = (santa_position.0 + santa_move.0, santa_position.1 + santa_move.1);
            robot_position = (robot_position.0 + robot_move.0, robot_position.1 + robot_move.1);

            santa_visits.insert(santa_position.clone());
            robot_visits.insert(robot_position.clone());
        }
    );

    let diff = robot_visits.intersection(&santa_visits).count();

    println!("{}", santa_visits.len() + robot_visits.len() - diff);
}

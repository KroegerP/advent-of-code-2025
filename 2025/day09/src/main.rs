use itertools::Itertools;

use utils::get_input;

fn main() {
    // let data = include_str!("test.txt");
    let data = get_input(2025, 9);

    let coordinates: Vec<(i64, i64)> = data
        .split_whitespace()
        .map(|f| {
            let mut xy = f.split(",");

            (
                xy.next().unwrap().parse().unwrap(),
                xy.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut max = 0;

    for pair in coordinates.iter().combinations(2) {
        let [&p1, &p2] = pair[0..2] else {
            unreachable!()
        };
        let x1 = p1.0.min(p2.0);
        let x2 = p1.0.max(p2.0);
        let y1 = p1.1.min(p2.1);
        let y2 = p1.1.max(p2.1);
        max = max.max((x2 - x1 + 1) * (y2 - y1 + 1));
    }

    println!("{max}");
}

use utils::get_input;

fn calc_area(c1: &(i64, i64), c2: &(i64, i64)) -> u64 {
    // println!("Calculating {:?} {:?}", c1, c2);
    (((c2.1 - c1.1).abs() + 1) * ((c2.0 - c1.0).abs() + 1)) as u64
}

fn main() {
    // let data = include_str!("test.txt");
    let data = get_input(2025, 9);

    let mut coordinates: Vec<(i64, i64)> = data
        .split_whitespace()
        .map(|f| {
            let mut xy = f.split(",");

            (
                xy.next().unwrap().parse().unwrap(),
                xy.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    coordinates.sort_by_key(|a| (a.0, a.1));

    // println!("{:?}", coordinates);

    let mut l = 0;
    let mut r = coordinates.len() - 1;

    let mut max: u64 = 0;

    while r > l {
        // println!("Trying {:?} {:?}", coordinates[l], coordinates[r]);

        let new_calc: u64 = calc_area(&coordinates[l], &coordinates[r]);

        // println!("{new_calc}");

        if new_calc > max {
            println!("New Max: {new_calc}");
            max = new_calc;
        }

        let next_l_area: u64 = calc_area(&coordinates[l + 1], &coordinates[r]);
        let next_r_area: u64 = calc_area(&coordinates[l], &coordinates[r - 1]);

        if next_l_area > next_r_area {
            l += 1;
        } else {
            r -= 1;
        }
    }

    println!("{max}");
}

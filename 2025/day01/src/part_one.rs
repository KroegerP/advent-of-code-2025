const DATA: &str = include_str!("input.txt");

fn get_clicks(rotation: &str) -> i32 {
    let (dir, count) = rotation.split_at(1);

    let clicks: i32 = count.parse().unwrap();

    print!("{} {} |     ", dir, clicks);

    if dir == "R" { clicks } else { clicks * -1 }
}

pub fn solve_part_one() {
    let mut current_position = 50;

    let mut zero_counter = 0;

    DATA.split_whitespace().enumerate().for_each(|(i, f)| {
        let clicks = get_clicks(f);

        print!("{} + {} = ", current_position, clicks);

        current_position = ((current_position + clicks) % 100 + 100) % 100;

        println!("{}, ", current_position);

        if current_position == 0 {
            zero_counter += 1;
            println!("Went to zero at index {} {}", i, f);
        }
    });

    println!("Went to zero {} times", zero_counter);
}

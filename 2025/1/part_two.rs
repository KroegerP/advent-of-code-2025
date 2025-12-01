const DATA: &str = include_str!("input.txt");

fn get_clicks(rotation: &str) -> i32 {
    let (dir, count) = rotation.split_at(1);

    let clicks: i32 = count.parse().unwrap();

    if dir == "R" { clicks } else { clicks * -1 }
}

fn main() {
    let mut current_position = 50;

    let mut zero_counter = 0;

    DATA.split_whitespace().for_each(|f| {
        let clicks = get_clicks(f);

        let mut local_0_count = 0;

        let raw_pos = current_position + clicks;

        let new_position = ((current_position + clicks) % 100 + 100) % 100;

        if new_position == 0 {
            local_0_count += 1;
        }

        if (raw_pos.abs() / 100) > 0 {
            let num_passes = raw_pos.abs() / 100;

            local_0_count += num_passes;

            if (raw_pos.abs() / 100) > 1 {
                print!("Added {} full trips | ", num_passes);
            }
        }

        println!(
            "{} + {} = {}  | Total added: {}",
            current_position, clicks, new_position, local_0_count
        );

        current_position = new_position;
        zero_counter += local_0_count;
    });

    println!("Went to zero {} times", zero_counter);
}

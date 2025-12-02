use utils::get_input;

fn get_clicks(cur_pos: i32, rotation: &str) -> (i32, i32) {
    let (dir, count) = rotation.split_at(1);

    let clicks: i32 = count.parse().unwrap();

    let mut new_position = cur_pos;
    let mut zero_count = 0;

    match dir {
        "R" => {
            new_position = new_position + clicks;
            zero_count += new_position / 100;
            new_position = (new_position % 100).abs();
        }
        "L" => {
            new_position = new_position - clicks;
            if new_position <= 0 {
                zero_count += (new_position - 100) / -100;
            }
            if cur_pos == 0 {
                zero_count -= 1;
            }
            new_position = (new_position % 100 + 100) % 100;
        }
        _other => unreachable!(),
    }

    (new_position, zero_count)
}

fn solve_part_two(input: String) {
    let mut current_position = 50;

    let mut zero_counter = 0;

    input.split_whitespace().for_each(|f| {
        let (new_position, zeros_count) = get_clicks(current_position, f);

        current_position = new_position;
        zero_counter += zeros_count;
    });

    println!("Went to zero {} times", zero_counter);
}

fn main() {
    let data = get_input(2025, 1);

    solve_part_two(data);
}

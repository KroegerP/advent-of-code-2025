use std::{str::FromStr};

const DATA: &str = include_str!("input.txt");

fn main() {
    let contents = String::from_str(DATA).unwrap();

    let mut count = 0;
    let mut flag = false;

    for (u, c) in contents.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;

            if count < 0 && !flag {
                println!("{}", u + 1);
                flag = true;
            }
        }
    }
}

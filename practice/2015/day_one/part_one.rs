use std::{str::FromStr};

const DATA: &str = include_str!("input.txt");

fn main() {
    let contents = String::from_str(DATA).unwrap();

    let plus_count = contents.chars().filter(|c| c.to_ascii_lowercase() == '(').count();
    let minus_count = contents.chars().filter(|c| c.to_ascii_lowercase() == ')').count();

    let res = plus_count - minus_count;

    println!("{res}")
}

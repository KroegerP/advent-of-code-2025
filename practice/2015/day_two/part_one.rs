use std::{cmp::min, thread::{self, JoinHandle}};

const DATA: &str = include_str!("input.txt");


fn smallest_dim(x: u16, y: u16, z: u16) -> u16 {
    min(x, min(y, z))
}

fn get_areas(l: u16, w:u16, h: u16) -> Vec<u16> {
    vec![l * w, w * h, h * l]
}

fn get_surface_area(l: u16, w:u16, h: u16) -> u16 {
    (2 * l * w) +  (2 * w * h) + (2 * h * l)
}

fn dims_to_surface_area(dim: &str) -> u16 {
    let split_str: Vec<&str> = dim.split("x").collect();

    if split_str.len() < 3 || split_str.len() > 3 {
        return 0;
    }

    let l = split_str[0].parse::<u16>().unwrap();
    let w = split_str[1].parse::<u16>().unwrap();
    let h = split_str[2].parse::<u16>().unwrap();

    let areas = get_areas(l, w, h);

    get_surface_area(l, w, h) + smallest_dim(areas[0], areas[1], areas[2])
}

fn main() {
    let mut handles: Vec<JoinHandle<u16>> = Vec::new();

    for dim_list in DATA.split_whitespace() {
        let handle = thread::spawn(move || {
            dims_to_surface_area(dim_list)
        });

        handles.push(handle);
    }

    let mut sum: u32 = 0;

    for handle in handles {
        sum += handle.join().unwrap() as u32;
    }

    println!("{}", sum)
}

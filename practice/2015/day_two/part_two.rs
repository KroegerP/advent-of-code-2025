const DATA: &str = include_str!("input.txt");


fn smallest_2_dims(x: u16, y: u16, z: u16) -> (u16, u16) {
    let mut sorted_vec = vec![x, y, z];

    sorted_vec.sort();

    (sorted_vec[0], sorted_vec[1])
}

fn get_smallest_perimeter(l: u16, w:u16, h: u16) -> u16 {
    let (x, y) = smallest_2_dims(l, w, h);

    (2 * x) + (2 * y)
}

fn get_volume(l: u16, w:u16, h: u16) -> u16 {
    l * w * h
}

fn dims_to_surface_area(dim: &str) -> u16 {
    let split_str: Vec<&str> = dim.split("x").collect();

    if split_str.len() < 3 || split_str.len() > 3 {
        return 0;
    }

    let l = split_str[0].parse::<u16>().unwrap();
    let w = split_str[1].parse::<u16>().unwrap();
    let h = split_str[2].parse::<u16>().unwrap();

    get_volume(l, w, h) + get_smallest_perimeter(l, w, h)
}

fn main() {
    let mut sum: u32 = 0;

    for dim_list in DATA.split_whitespace() {
        sum += dims_to_surface_area(dim_list) as u32;
    }

    println!("{}", sum)
}

pub mod input_utils {
    pub fn get_input(year: u8, day: u8) -> String {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");

        reqwest::blocking::get(url)
            .expect("request failed")
            .text()
            .expect("body failed")
    }
}
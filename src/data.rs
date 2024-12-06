use std::fs;

pub fn load(day: u32, suffix: Option<&str>) -> String {
    load_raw(day, suffix).trim().replace('\r', "")
}

pub fn load_raw(day: u32, suffix: Option<&str>) -> String {
    let file = format!("./data/day{:02}{}.txt", day, suffix.unwrap_or(""));
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}

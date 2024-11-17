use std::fs;
use std::io;

pub fn read_input(year: u32, day: u8) -> Result<String, io::Error> {
    let path = format!("input/{}/day{:02}.txt", year, day);
    fs::read_to_string(path)
}

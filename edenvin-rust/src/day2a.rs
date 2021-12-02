use std::fs::File;
use std::io::{BufRead, BufReader};
use std::println;

pub fn run() -> std::io::Result<()> {
    let file = File::open("data/2.txt")?;
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line2 = line?;
        let line = line2.split(" ").collect::<Vec<&str>>();
        let direction = line[0].to_string();
        let length = line[1].parse::<i32>().unwrap();

        match direction.as_ref() {
            "forward" => horizontal += length,
            "down" => depth += length,
            _ => depth -= length,
        }
    }

    let result = horizontal * depth;

    println!("{}", result.to_string());
    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::println;

pub fn run() -> std::io::Result<()> {
    let file = File::open("data/1.txt")?;
    let reader = BufReader::new(file);

    let mut last_value: i32 = 99999999;
    let mut counter = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().parse::<i32>().unwrap();

        if line > last_value {
            counter = counter + 1;
        }

        last_value = line;
    }

    println!("{}", counter.to_string());
    Ok(())
}

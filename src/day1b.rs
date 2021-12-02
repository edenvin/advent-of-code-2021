use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> std::io::Result<()> {
    let file = File::open("../data/1.txt")?;
    let reader = BufReader::new(file);

    let measurements: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut last_value: i32 = 99999999;
    let mut counter = 0;

    let mut two_back = measurements[0].parse::<i32>().unwrap();
    let mut one_back = measurements[1].parse::<i32>().unwrap();

    for n in 2..measurements.len() {
        let line = measurements[n].parse::<i32>().unwrap();

        let sum = two_back + one_back + line;
        if sum > last_value {
            counter = counter + 1;
        }

        two_back = one_back;
        one_back = line;
        last_value = sum;
    }

    println!("{}", counter.to_string());
    Ok(())
}

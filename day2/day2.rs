use std::io::{BufRead, BufReader};
use std::fs::File;

static MAX_DISTANCE: i32 = 3;

fn main() -> std::io::Result<()> {
    let file = File::open("input/input.txt")?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<i32>> = Vec::new();


    let mut line_index = 0;
    for line in reader.lines() {
        data.push(Vec::new());
        // Split line into words
        for word in line.unwrap().split_whitespace() {
            let number = word.parse::<i32>().unwrap();
            data[line_index].push(number);
        }

        line_index += 1;
    }

    println!("Doing part 1...{}\n", data.len());

    part_one(data);

    Ok(())
}

fn part_one(mut data: Vec<Vec<i32>>) {
    println!("Data Length: {}", data.len());

    let mut safe_count = 0;

    for row in data {
        let mut increased = false;
        let mut decreased = false;
        let mut equal = false;

        let mut safe = true;

        for (index, value) in row.iter().enumerate() {
            if index == 0 {
                continue;
            }

            let last_value = row[index - 1];

            if last_value < *value {
                increased = true;
            }
            if last_value > *value {
                decreased = true
            }
            if last_value == *value {
                equal = true
            }

            safe = (value - last_value).abs() <= MAX_DISTANCE;

            safe = safe && increased != decreased && !equal;

            if !safe {
                break;
            }
        }

        safe_count = if safe { safe_count + 1 } else { safe_count };
    }

    println!("Part 1 Answer: {}\n", safe_count);
}
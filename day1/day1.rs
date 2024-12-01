use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file = File::open("input/input.txt")?;
    let reader = BufReader::new(file);

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    let mut line_index = 0;
    for line in reader.lines() {
        // Split line into words
        for word in line.unwrap().split_whitespace() {
            let number = word.parse::<i32>().unwrap();
            if line_index & 1 == 0 {
                list_one.push(number);
            }
            else {
                list_two.push(number);
            }
            line_index += 1;
        }
    }

    println!("Doing part 1...\n");

    //part_one(list_one, list_two);

    println!("Doing part 2...\n");

    part_two(list_one, list_two);

    Ok(())
}

fn part_one(mut list_one: Vec<i32>, mut list_two: Vec<i32>) {
    println!("List 1 Length: {}", list_one.len());
    println!("List 2 Length: {}", list_two.len());
    
    list_one.sort();
    list_two.sort();

    let mut total_distance = 0;
    for (index, _val) in list_one.iter().enumerate() {

        let mut distance = list_one[index] - list_two[index];
        distance = if distance < 0 { distance * -1 } else { distance };

        total_distance += distance;
    }

    println!("Part 1 Answer: {}\n", total_distance);
}

fn part_two(list_one: Vec<i32>, list_two: Vec<i32>) {
    let mut summary_score = 0;

    let mut summary_scores = HashMap::new();
    // create entries from list 1
    for (_index, value) in list_one.iter().enumerate() {
        if !summary_scores.contains_key(value) {
            summary_scores.insert(value, 0);
        }
    }

    // add number of times entry from list 1 shows up in list 2
    for (_index, value) in list_two.iter().enumerate() {
        if summary_scores.contains_key(value) {
            let val_so_far = summary_scores.get(value).unwrap();
            summary_scores.insert(value, val_so_far + value);
        }
    }

    // get the sum of all summary values
    for (_index, value) in list_one.iter().enumerate() {
        if summary_scores.contains_key(value) {
            summary_score += summary_scores.get(value).unwrap();
        }
    }

    println!("Part 2 Answer: {}", summary_score);
}
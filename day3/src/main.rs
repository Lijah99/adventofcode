use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("input/input.txt")?;
    let reader = BufReader::new(file); 

    let mut input_string = String::new();

    for line in reader.lines() {
        input_string.push_str(&line.unwrap());
    }

    part_one(&input_string);

    part_two(&input_string);

    Ok(())
}

fn part_one(input_string: &str) {
    let mut sum = 0;

    sum += get_sum_from_string(&input_string);

    println!("Part 1 Results: {}\n", sum);
}

fn part_two(input_string: &str) {
    let mut sum = 0;
    
    let splits: Vec<&str> = input_string.split("don't()").collect();
    
    // Calculate sum since it starts at Do()
    sum += get_sum_from_string(splits[0]);
    
    for i in 1..splits.len() {
        let huh = splits[i];
        let vec: Vec<&str> = huh.split("do()").collect();
    
        // skip first index since it's a dont block
        for j in 1..vec.len() {
            sum += get_sum_from_string(vec[j]);
        }
    }
    
    println!("Part 2 Results: {}\n", sum);
}

fn get_sum_from_string(string: &str) -> i32 {
    let mut sum = 0;

    let splits: Vec<&str> = string.split("mul(").collect();
        for split in splits {
            if split.len() < 4 {
                continue;
            }

            let numbers = get_numbers(split);

            if numbers.is_none() {
                continue;
            }

            let (number1, number2) = numbers.unwrap();

            sum += number1 * number2;
        }

    return sum;
}

fn get_numbers(split: &str) -> Option<(i32, i32)> {
    let split_split = split.split_once(")");
    if split_split.is_none() {
        return None;
    }

    let (string, _ignore) = split_split.unwrap();

    if string.len() < 3 || string.len() > 7 {
        return None;   
    }

    let num_split = string.split_once(",");

    if num_split.is_none() {
        return None;
    }

    let (number1_string, number2_string) = num_split.unwrap();

    if (number1_string.len() < 1 || number1_string.len() > 3) && (number2_string.len() < 1 || number2_string.len() > 3) {
        return None;
    }

    let number1 = number1_string.parse::<i32>();
    let number2 = number2_string.parse::<i32>();

    if number1.is_err() || number2.is_err() {
        return None;
    }

    return Some((number1.unwrap(), number2.unwrap()));
}

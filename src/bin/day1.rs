
use std::fs;
use std::collections::HashMap;

fn extract_digits_2(text: &str) -> Vec<isize> {
    let regex_set = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut integer_list: Vec<Option<isize>> = vec![None; text.len()];

    for i in 0..text.len() {
        for (re, val) in regex_set.iter() {
            if text[i..].starts_with(re) {
                integer_list[i] = Some(*val);
                break;
            }
        }
    }
    integer_list.iter().flatten().cloned().collect::<Vec<_>>()
}

fn first_last_add(integer_list : &Vec<isize>) -> isize {
    integer_list.first().unwrap() * 10 + integer_list.last().unwrap()
}
fn day1_2(file_data:&String)-> isize {
    file_data.lines()
        .map(|a| first_last_add(&extract_digits_2(a)) )
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn day1(file_data:&String)-> u32 {
    let mut final_value :u32 = 0;
    
    for row in file_data.lines() {
        // char is a singlue unicode character. 
        let integers: Vec<u32>  = row.chars().filter_map(|v| v.to_digit(10)).collect();

        let row_value = if integers.len() >= 2 {
            (integers[0].to_string() + &integers[integers.len() -1].to_string()).parse::<u32>().unwrap()
        } else { 
            (integers[0].to_string() + &integers[0].to_string()).parse::<u32>().unwrap()
        };

        final_value += row_value;
    };
    final_value 
}
fn main() {
    let file_data = fs::read_to_string("./puzzle_inputs/day_01.txt").unwrap();

    let answer_1 = day1(&file_data);
    assert_eq!(answer_1, 55816);
    println!("Problem 1 Answer: {:?}", answer_1);

    let answer_2 = day1_2(&file_data);
    assert_eq!(answer_2, 54980);
    println!("Problem 2 Answer: {:?}", answer_2);
}

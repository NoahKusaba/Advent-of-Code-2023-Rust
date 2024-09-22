use std::collections::HashMap;
use std::fs;

fn power_sets(line: &str) -> u32 {
    // Split the input line and handle cases where the format might be unexpected
    let line_split: Vec<_> = line.split(": ").collect();
    if line_split.len() < 2 {
        panic!("Invalid input format");
    }

    // Split the sets
    let game_sets = line_split[1].split("; ");

    // Initialize color counts
    let mut line_totals: HashMap<&str, u32> = HashMap::from([
        ("red", 0),
        ("blue", 0),
        ("green", 0),
    ]);

    // Process each game set
    for game_set in game_sets {
        for draw in game_set.split(", ") {
            let mut parts = draw.split(" ");
            let color_count: u32 = parts.next().expect("Missing count").parse().expect("Invalid number");
            let color = parts.next().expect("Missing color");

            // Update max count per color
            if let Some(total) = line_totals.get_mut(color) {
                if *total < color_count {
                    *total = color_count;
                }
            }
        }
    }

    // Return the product of the highest counts for each color
    line_totals.values().product()
}


fn valid_game_ids(line: &str) -> u32 {
    let total_red = 12;
    let total_green = 13;
    let total_blue = 14;

    // Split the line into game number and game sets
    let mut parts = line.split(": ");
    let game_number_str = parts.next().unwrap().split_whitespace().nth(1).unwrap();
    let game_number: u32 = game_number_str.parse().expect("Failed to parse game number");

    let game_sets_str = parts.next().unwrap();
    
    // Iterate over each game set
    for game_set in game_sets_str.split("; ") {
        let mut line_totals = HashMap::from([
            ("red", 0),
            ("blue", 0),
            ("green", 0),
        ]);

        // Process each draw
        for draw in game_set.split(", ") {
            let mut draw_parts = draw.split_whitespace();
            let color_count: u32 = draw_parts.next().unwrap().parse().expect("Failed to parse color count");
            let color = draw_parts.next().unwrap();

            if let Some(total) = line_totals.get_mut(color) {
                *total += color_count;
            }

            if line_totals["red"] > total_red || line_totals["green"] > total_green || line_totals["blue"] > total_blue {
                return 0;
            }
        }
    }

    game_number
}

fn day_2_1(file_data:&String) -> u32 {
    file_data
        .lines() 
        .map(|line: &str| valid_game_ids(line) )
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn day_2_2(file_data:&String) -> u32 {
    file_data
        .lines()
        .map(|line: &str| power_sets(line) )
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn main(){
    let file_data = fs::read_to_string("./puzzle_inputs/day_02.txt").unwrap();

    let answer_1 = day_2_1(&file_data);
    assert_eq!(answer_1, 2076);
    println!("Problem 1: Sum of Valid Game of ids is {:?}", answer_1);

    // Problem 2
    let answer_2 = day_2_2(&file_data);
    assert_eq!(answer_2, 70950);
    println!("Problem 2: Sum of Valid Game of ids is {:?}", answer_2);
}
use std::fs;

fn retrieve_integers(map_string: &str) -> Vec<usize>{
    map_string.split(" ")
        .map(|integer| integer.parse().unwrap())
        .collect::<Vec<usize>>()
}

fn extract_seeds(seed_maps: Vec<String> )-> Vec<usize> {
    seed_maps[0]
        .split("seeds: ")
        .collect::<Vec<_>>()[1]
        .split(" ")
        .map(|seed_num| seed_num.parse().unwrap())
        .collect::<Vec<_>>()
}

fn get_location_numbers(seed:usize, seed_maps: &Vec<String>)-> usize {

    let mut current_state:usize = seed.clone();
    for seed_map in &seed_maps[1..]{
        let current_maps = &seed_map.lines()
            .map(String::from)
            .collect::<Vec<_>>()[2..];

        let maps_parsed = current_maps.iter()
            .map(|map_| retrieve_integers(map_))
            .collect::<Vec<Vec<usize>>>();

        for map in &maps_parsed{
            if current_state >= map[1] && current_state <= map[1] + map[2] -1{
                current_state = current_state - map[1] + map[0];
                break;
            }
        }        
    }
    return current_state

}   
fn get_seed_ranges(seeds: &Vec<usize>) -> Vec<(usize,usize)> {

    let mut seed_ranges: Vec<(usize,usize)> = Vec::new();

    for i in (0..seeds.len()).step_by(2){
        if i < seeds.len(){
            seed_ranges.push((seeds[i], seeds[i+1]))
        }
    }
    seed_ranges
}

// Binary search 
fn get_position_by_seed_range(seed_range:(usize,usize), seed_maps: &Vec<String>)-> usize{
    let (mut start, mut end) = seed_range;
    end = start + end - 1;
    let mut min_location = usize::MAX; // Set initial minimum location to max, so it will be set in first pass. 
    while start <= end {
        let mid = start +(end -start) /2;
        let mid_position = get_location_numbers(mid, &seed_maps);
        if mid_position < min_location{
            end = mid-1;
            min_location = mid_position;
        } else{
            start = mid + 1;
        }
    }
    min_location
}
fn main(){
    let file_data: String = fs::read_to_string("./puzzle_inputs/day_05.txt").expect("attempting to read file");
    let seed_maps: Vec<String> = file_data.split("\r\n\r")
        .map(String::from)
        .collect::<Vec<String>>();

    let seeds: Vec<usize> = extract_seeds(seed_maps.clone());

    // PROBLEM 1: Determine lowest location number
    let location_numbers_01: Vec<usize> = seeds.iter()
        .map(|seed| get_location_numbers(*seed, &seed_maps))
        .collect::<Vec<usize>>();
    let answer_1 = location_numbers_01.iter().min().unwrap().clone();
    assert_eq!(answer_1, 309796150);
    println!("Problem 1 Lowest location number is {:?}", answer_1); 

    // PROBLEM 2: Determine lowest locaiton number, from range of seeds
    let seed_ranges: Vec<(usize,usize)> = get_seed_ranges(&seeds);
    let location_numbers_02: Vec<usize> = seed_ranges.iter()
        .map(|seed_range| get_position_by_seed_range(*seed_range, &seed_maps) )
        .collect::<Vec<usize>>();
    let answer_2 = location_numbers_02.iter().min().unwrap().clone(); 
    assert_eq!(answer_2, 50716416);
    println!("Problem 2 Lowest location number is {:?}", answer_2); 
}
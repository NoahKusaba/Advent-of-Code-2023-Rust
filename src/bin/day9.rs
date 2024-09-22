use std::fs;

fn generate_history(pattern: &Vec<isize>) -> Vec<Vec<isize>>{
    let mut history_vectors: Vec<Vec<isize>> = Vec::new();
    history_vectors.push(pattern.clone());

    while !history_vectors
        .last()
        .unwrap()
        .iter()
        .all(|val| *val == 0){
            history_vectors.push(
                history_vectors
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<isize>>());

    }
    return history_vectors
}

fn calculate_next_value(pattern: &Vec<isize>) -> isize{

    let history_vectors: Vec<Vec<isize>> = generate_history(pattern);
    
    history_vectors
        .iter()
        .rev()
        .filter_map(|v| v.last())
        .sum()

}
fn calculate_prev_value(pattern: &Vec<isize>) -> isize{

    let history_vectors: Vec<Vec<isize>> = generate_history(pattern);
    
    history_vectors
        .iter()
        .rev()
        .filter_map(|v| v.first())
        .map(|&x| x)
        .reduce(|a,b| b - a )
        .unwrap_or(0)

}

fn main(){
    let file_data:String = fs::read_to_string("./puzzle_inputs/day_09.txt").expect("Failed to read file");
    let patterns: Vec<Vec<isize>> = file_data
        .lines()
        .map(|line| 
                line.split(" ")
                .map(|val| val.parse().unwrap())
                .collect::<Vec<isize>>())
        .collect();
    let next_values_1: Vec<isize> = patterns
        .iter()
        .map(|pattern| calculate_next_value(pattern))
        .collect();

    let answer_1= next_values_1.iter().sum::<isize>();
    println!("Problem 1 answer {:?}", answer_1);
    assert_eq!(answer_1, 2043677056);

    // Problem 2 
    let prev_values_2: Vec<isize> = patterns
        .iter()
        .map(|pattern| calculate_prev_value(pattern))
        .collect();
    let answer_2 = prev_values_2.iter().sum::<isize>();
    println!("Problem 2 answer {:?}", answer_2);
    assert_eq!(answer_2, 1062);

}
use std::fs;
fn extract_characters_cords(file_data: String) -> (Vec<usize>, Vec<Vec<(usize, usize)>>, Vec<(usize, usize)>) {
    let file_lines: Vec<Vec<char>> = file_data.lines().map(|line| line.chars().collect()).collect();

    let mut integer_list = Vec::new();
    let mut integer_adjacent_list = Vec::new();
    let mut special_list = Vec::new();

    let max_y_len = file_lines.len();
    let max_x_len = file_lines[0].len();

    for (y_idx, line) in file_lines.iter().enumerate() {
        let mut current_integer = String::new();
        let mut current_integer_adjacent = Vec::new();

        for (x_idx, &ch) in line.iter().enumerate() {
            if ch.is_numeric() {
                current_integer.push(ch);

                let adjacent_positions = [
                    (x_idx.wrapping_sub(1), y_idx),             // left
                    (x_idx + 1, y_idx),                         // right
                    (x_idx, y_idx.wrapping_sub(1)),             // top
                    (x_idx, y_idx + 1),                         // down
                    (x_idx.wrapping_sub(1), y_idx.wrapping_sub(1)), // top-left
                    (x_idx + 1, y_idx.wrapping_sub(1)),         // top-right
                    (x_idx.wrapping_sub(1), y_idx + 1),         // bottom-left
                    (x_idx + 1, y_idx + 1),                     // bottom-right
                ];

                current_integer_adjacent.extend(
                    adjacent_positions
                        .iter()
                        .cloned()
                        .filter(|&(x, y)| x < max_x_len && y < max_y_len),
                );
            } else {
                if !current_integer.is_empty() {
                    integer_list.push(current_integer.parse::<usize>().unwrap());
                    integer_adjacent_list.push(current_integer_adjacent.clone());
                    current_integer.clear();
                    current_integer_adjacent.clear();
                }
                if ch != '.' {
                    special_list.push((x_idx, y_idx));
                }
            }
        }

        if !current_integer.is_empty() {
            integer_list.push(current_integer.parse::<usize>().unwrap());
            integer_adjacent_list.push(current_integer_adjacent);
        }
    }
    
    (integer_list, integer_adjacent_list, special_list)
}


fn day3_01(integer_list:Vec<usize>, integer_adjacent_list: Vec<Vec<(usize, usize)>>, special_list: Vec<(usize, usize)>)-> usize{
    let mut valid_sum: usize = 0;

    for integer_index in 0..integer_list.len(){
        for &special_char_position in &special_list{
            if  integer_adjacent_list[integer_index].contains(&special_char_position){
                valid_sum += integer_list[integer_index];
                break;
            }
        }
    }
    valid_sum     
}

fn day3_02(integer_list:Vec<usize>, integer_adjacent_list: Vec<Vec<(usize, usize)>>, special_list: Vec<(usize, usize)>)-> usize{
    let mut valid_sum: usize = 0;
    
    for &special_char_position in &special_list{
        let mut adjacent_characters: Vec<_> = Vec::new();

        for integer_index in 0..integer_list.len(){
            if  integer_adjacent_list[integer_index].contains(&special_char_position){
                adjacent_characters.push(integer_list[integer_index]);
            }
        }
        if adjacent_characters.len() == 2{
            valid_sum += adjacent_characters.iter().product::<usize>();
        }
    }
    valid_sum
}
fn main(){
    let file_data: String = fs::read_to_string("./puzzle_inputs/day_03.txt").unwrap();
    let (integer_list, integer_adjacent_list, special_list) = extract_characters_cords(file_data);

    let answer_1 = day3_01(integer_list.clone(), integer_adjacent_list.clone(), special_list.clone());
    assert_eq!(answer_1, 540212);
    println!("Problem 1 Final Sum {}", answer_1);

    // Problem 2
    let answer_2 = day3_02(integer_list.clone(), integer_adjacent_list.clone(), special_list.clone());
    assert_eq!(answer_2, 87605697);
    println!("Problem 2 Final Sum {}", answer_2);

}
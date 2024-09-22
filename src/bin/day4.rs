use std::fs;
use std::collections::HashMap;
fn extract_ticket_numbers(file_data: String) -> (Vec<Vec<String>>, HashMap<u128,u128>, Vec<u128>) {  
    
    let file_lines = file_data.lines()
                                .map(|line:&str| line)
                                .collect::<Vec<_>>();

    let mut ticket_numbers: Vec<Vec<String>> = Vec::new();
    let mut game_numbers_hash: HashMap<u128, u128> = HashMap::new();
    let mut game_numbers: Vec<u128> = Vec::new();

    for file_line in file_lines{

        let winning_number = file_line
            .split(" | ")
            .collect::<Vec<_>>()[0]
            .split(": ")
            .collect::<Vec<_>>()[1]
            .split(" ")
            .map(String::from)
            .collect::<Vec<_>>();

        let ticket_number = file_line
            .split(" | ")
            .collect::<Vec<_>>()[1]
            .split(" ")
            .into_iter()
            .map(String::from)
            .filter(|s| winning_number.contains(s) && s.len() > 0)
            .collect::<Vec<_>>();

        let game_number = file_line
            .split(": ")
            .collect::<Vec<_>>()[0]
            .split("Card ")
            .collect::<Vec<_>>()[1]
            .trim()
            .parse()
            .expect("Expected game number");

        game_numbers_hash.insert(game_number, 1);

        if ticket_number.len() > 0 {
            game_numbers.push(game_number);
            ticket_numbers.push(ticket_number);

        }
    }
    (ticket_numbers, game_numbers_hash, game_numbers)
}


fn day_4_01(file_data : String) -> usize{
    let (ticket_numbers, _game_numbers_hash, _game_numbers) = extract_ticket_numbers(file_data);

    ticket_numbers
        .iter()
        .filter(|s| s.len() > 0)
        .map(|winning_number_counts| 2_usize.pow((winning_number_counts.len() -1).try_into().expect("calculating points")))
        .collect::<Vec<_>>()
        .iter()
        .sum()

}

fn day_4_02(file_data:String)-> u128{
    let (ticket_numbers, mut game_numbers_hash,game_numbers) = extract_ticket_numbers(file_data);

    for winning_ticket_index in 0..ticket_numbers.len(){
        let num_winning_tickets: u128 = ticket_numbers[winning_ticket_index].len().try_into().unwrap();
        let current_game_number = game_numbers[winning_ticket_index] ;
        let current_ticket_copy = game_numbers_hash.get(&current_game_number).copied().unwrap_or(0);
        let max_range = if (current_game_number + num_winning_tickets + 1) > game_numbers_hash.len() as u128 {1 + game_numbers_hash.len() as u128} else {current_game_number + num_winning_tickets + 1};
        for game_number in (current_game_number +1)..(max_range ) {

            match game_numbers_hash.get(&game_number) {
                Some(count) => { game_numbers_hash.insert(game_number, count + 1 * current_ticket_copy); }
                None =>               { println!("This shouldn't be possible ");}
            }
        }
    }
    game_numbers_hash.values().fold(0, |a,b| a+b) 
}

fn main(){
    let file_data: String = fs::read_to_string("./puzzle_inputs/day_04.txt").expect("attempting to read file");

    let answer_1 = day_4_01(file_data.clone());
    assert_eq!(answer_1, 15268);
    println!("Game 1 Total Score {:?} ", answer_1 );

    // Problem 2
    let answer_2= day_4_02(file_data);
    assert_eq!(answer_2, 6283755);
    println!("Game 2 Total Score {:?} ", answer_2 );

}
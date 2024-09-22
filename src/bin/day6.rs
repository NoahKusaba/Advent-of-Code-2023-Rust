use std::fs;
#[derive(Debug)]

struct Race{ time: usize, distance:usize}

impl Race {
    fn get_button_wins(&self) -> Vec<usize>{
        let mut valid_times: Vec<usize> = Vec::new();
        for time_pressed in 1..(self.time -1) {
            if self.distance < time_pressed * (self.time - time_pressed){
                valid_times.push(time_pressed * (self.time - time_pressed));
            }
            else if valid_times.len() > 0 {
                return valid_times
            }

        }
        valid_times
    }
}

fn extract_time_distance(file_data: &String) -> Vec<Race> {
    let lines = file_data.lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let times = lines[0]
        .split("Time:")
        .collect::<Vec<_>>()[1]
        .split(" ")
        .filter_map(|character| character.parse().ok())
        .collect::<Vec<usize>>();
    let distances = lines[1]
        .split("Distance: ")
        .collect::<Vec<_>>()[1]
        .split(" ")
        .filter_map(|character| character.parse().ok())
        .collect::<Vec<usize>>();

    let mut races: Vec<Race> = Vec::new(); 
    for index in 0..times.len(){
        let race = Race{
            time: times[index],
            distance:  distances[index]
        };

        races.push(race);

    }
    races
}

fn extract_time_distance_2(file_data: &String) -> Race {
    let lines = file_data.lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let time = lines[0]
        .split("Time:")
        .collect::<Vec<_>>()[1]
        .replace(char::is_whitespace, "")
        .parse().unwrap();

    let distance = lines[1]
        .split("Distance: ")
        .collect::<Vec<_>>()[1]
        .replace(char::is_whitespace, "")
        .parse().unwrap();

    Race{ time: time, distance:  distance}
}

fn main(){
    let file_data: String = fs::read_to_string("./puzzle_inputs/day_06.txt").expect("attempting to read file");
    let races = extract_time_distance(&file_data);
    
    // Puzzle 1 
    let answer_1 = races.iter()
        .map(|race| race.get_button_wins().len())
        .reduce(|a,b| a * b)
        .unwrap();
    assert_eq!(answer_1, 1413720);
    println!("Puzzle 1 Answer {:?}", answer_1);

    // Puzzle 2
    let race_2 = extract_time_distance_2(&file_data);
    let answer_2= race_2.get_button_wins().len();
    assert_eq!(answer_2, 30565288);
    println!("Puzzle 2 Answer {:?}", answer_2);

}
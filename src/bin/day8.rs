use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn gcd( a: usize, b: usize) -> usize {
    if a == 0{
        return b
    }
    return gcd( b%a , a )
}

fn lcm_of_vec(numbers: Vec<usize>, idx: usize) -> usize {
    if idx == numbers.len() - 1 { 
        return numbers[idx]
    }
    let a = numbers[idx];
    let b = lcm_of_vec(numbers.clone(), idx + 1 ) ;
    return a * b / gcd(a,b)
}

fn index_in_direction(new_direction: char ) -> usize {
    match new_direction {
        'R' => 1,
        'L' => 0,
         _ => panic!("Unable to parse direction")
    }

}
struct MapNodes{node:HashMap<String, Vec<String>>, pattern:Vec<usize>}

impl MapNodes {
    fn count_steps_1(&self) -> usize {
        let mut current_node = String::from("AAA");
        let mut count: usize = 0; 
        loop{ 
            for direction in &self.pattern {
                count +=1 ; 
                current_node = self.node.get(&current_node).unwrap()[*direction].clone();
                if current_node == "ZZZ"{ return count }
            }
        }
    }
}

impl MapNodes {
    // Brute Force Solution
    // fn count_steps_2(&self) -> usize {
    //     let mut start_nodes: Vec<String> = self.node.keys().into_iter().filter(|f| f.chars().last() == Some('A') ).map(String::from).collect::<Vec<String>>();

    //     for index in 0..start_nodes.len(){
    //         let z_steps = Vec::new();
    //         let steps_full_loop = 0

    //         for direction in &self.pattern {
    //             count +=1 ; 
                
    //             // println!("Old Current_nodes {:?}",current_nodes[index] );
    //             current_nodes[index] = self.node.get(&current_nodes[index]).unwrap()[*direction].clone();

    //         }
    //         if current_nodes.iter().all(|val| val.chars().last() == Some('Z') ) { return count }
    //     }

    //     }
    // }
    fn count_steps_2(&self) -> usize {
        let mut start_nodes: Vec<String> = self.node.keys().into_iter().filter(|f| f.chars().last() == Some('A') ).map(String::from).collect::<Vec<String>>();
        let mut total_steps = HashMap::new();
        let mut count = 0; 
        while total_steps.len() != start_nodes.len(){

            for direction in &self.pattern {
                count +=1 ; 
                for index in 0..start_nodes.len(){
                    start_nodes[index] = self.node.get(&start_nodes[index]).unwrap()[*direction].clone();
                    if !total_steps.contains_key(&index)  && start_nodes[index].chars().last() == Some('Z'){
                        total_steps.insert(index, count);
                    }
                }
            }
        }
        lcm_of_vec( total_steps.values().cloned().collect(), 0 )
    }
}

fn parse_maps(map: &str) -> Vec<String> {
    let re = Regex::new(r"[A-Z0-9]{3}").unwrap();

    let matches =  re.find_iter(map)
                    .map(|m| m.as_str().to_string())
                    .collect::<Vec<String>>();
    
    if matches.len() != 2 {
        panic!("Unable to parse maps {:?} with input map {:?}", matches, map)  
    }
    else{
        matches
    }
}

fn extract_nodes(file_data: &String) -> MapNodes {

    let pattern = file_data.lines().next().expect("ERROR: Unable to parse pattern!").trim().chars().map(|c| index_in_direction(c)).collect::<Vec<usize>>();
    let node_map: HashMap<String, Vec<String>> = file_data.lines()
                                                          .skip(2)
                                                          .map(|x| {
                                                            let mut line = x.split(" = ");
                                                            let key = line.next().unwrap().to_string();
                                                            let values = parse_maps(line.next().unwrap());
                                                         (  key ,values ) })
                                                         .collect();                                                          
    MapNodes { node: node_map , pattern: pattern}
}

fn main(){
    let file_data:String = fs::read_to_string("./puzzle_inputs/day_08.txt").expect("Attempting to read file");
    let nodes: MapNodes = extract_nodes(&file_data);
    let answer_1 = nodes.count_steps_1();
    assert_eq!(answer_1, 13207);
    println!("Problem 1, Total Steps {:?}", answer_1 );

    // Problem 2
    let answer_2 = nodes.count_steps_2();
    assert_eq!(answer_2, 12324145107121);
    println!("Problem 2, Total Steps {:?}", answer_2);
}
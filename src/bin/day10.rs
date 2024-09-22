use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct GridPoint {
    val: char,
    position: (isize, isize), // (y, x) coordinates
    count: usize,             // Count value
}

// Implementing PartialEq and Eq for GridPoint
impl PartialEq for GridPoint {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for GridPoint {}

// Implementing Hash for GridPoint, bassed only on position
impl Hash for GridPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state); // Hash only the position, not the count
    }
}
// Problem_2 cares about order, but problem 1 is more efficient with a Hashset 
struct Node{ 
    grid_point: GridPoint, 
    graph: Vec<Vec<char>>, 
    visited: HashSet<GridPoint>,
    visited_2:Vec<GridPoint>
    }

impl Node{
    // Set the initial loc variable to the index position of the starting value.
    fn set_initial_start(&mut self) {
        for vertical_index in 0..self.graph.len(){
            for horizontal_index in 0..self.graph[0].len(){
                if self.graph[vertical_index][horizontal_index] == 'S'{
                   self.grid_point.position = (vertical_index as isize, horizontal_index as isize);
                   self.grid_point.val = self.graph[vertical_index][horizontal_index];
                   self.visited.insert(self.grid_point.clone());
                }
            }
        }
    }
}

impl Node{
    fn get_valid_directions(&mut self) -> HashSet<GridPoint> {

        // S + direction = loc
        let pipe_directions: HashMap<char, Vec<(isize, isize)>> = HashMap::from([
            ('|', vec![(1, 0), (-1, 0)]),
            ('-', vec![(0, 1), (0, -1)]),
            ('L', vec![(-1, 0), (0, 1)]),
            ('J', vec![(-1, 0), (0, -1)]),
            ('7', vec![(1, 0), (0, -1)]),
            ('F', vec![(1, 0), (0, 1)]),
            ('S', vec![(1, 0), (-1, 0), (0, 1), (0, -1)]),
        ]);

        let valid_pipe_directions = pipe_directions.get(&self.grid_point.val).unwrap();

        let mut valid_points = HashSet::new();
        for direction in valid_pipe_directions {
            let potential_point = (self.grid_point.position.0 + direction.0, self.grid_point.position.1 + direction.1);
            
            // Filter out invalid points & already visited points 
            if potential_point.0 >= 0 && potential_point.1 >= 0 
                && potential_point.0 < self.graph.len() as isize 
                && potential_point.1 < self.graph[0].len() as isize 
                && self.graph[potential_point.0 as usize][potential_point.1 as usize] != '.' {
                    let potential_val = self.graph[potential_point.0 as usize][potential_point.1 as usize] ;
                    let new_grid_point = GridPoint{ val: potential_val , position:potential_point, count: self.grid_point.count + 1 };
                    // determine if the pipe shapes can link pipes -J links but not -|
                    let inverted_directions: Vec<(isize, isize)> = pipe_directions.get(&potential_val).unwrap()
                        .iter()
                        .map(|(x, y)| (-x, -y))  // Apply the negation to both parts of the tuple
                        .collect();
                    
                    if inverted_directions.contains(direction) {    
                        if !self.visited.contains(&new_grid_point)   { 
                            valid_points.insert(new_grid_point.clone());
                        }
                        else if self.visited.contains(&new_grid_point) 
                                && self.visited.get(&new_grid_point).unwrap().count > self.grid_point.count + 1   {
                            self.visited.remove(&new_grid_point);
                            self.visited.insert(new_grid_point.clone()); // re add back to visited
                            valid_points.insert(new_grid_point.clone());
                        }
                    }
            }
        }
        valid_points
    }
}

impl Node{
    fn problem_1(&mut self) -> usize {
        self.set_initial_start();

        let mut current_traversals = self.get_valid_directions(); // tracks remaining paths to visit 

        while !current_traversals.is_empty(){   
            // Pop a random value from the Hashset
            let current_grid = current_traversals.iter().next().unwrap().clone();
            current_traversals.remove(&current_grid);

            // Set as the current grid
            self.grid_point = current_grid.clone();
            self.visited.insert(current_grid.clone());

            // Calculate new traversals from new gridPoint and add new values to current traversals 
            current_traversals.extend(self.get_valid_directions() ) ; 

            

    }
    return self.visited.iter().max_by_key(|point| point.count).unwrap().count
}
}

impl Node{
    fn get_valid_directions_2(&mut self) -> Option<GridPoint>{

        // S + direction = loc
        let pipe_directions: HashMap<char, Vec<(isize, isize)>> = HashMap::from([
            ('|', vec![(1, 0), (-1, 0)]),
            ('-', vec![(0, 1), (0, -1)]),
            ('L', vec![(-1, 0), (0, 1)]),
            ('J', vec![(-1, 0), (0, -1)]),
            ('7', vec![(1, 0), (0, -1)]),
            ('F', vec![(1, 0), (0, 1)]),
            ('S', vec![(1, 0), (-1, 0), (0, 1), (0, -1)]),
        ]);

        let valid_pipe_directions = pipe_directions.get(&self.grid_point.val).unwrap();

        let mut valid_points = Vec::new();
        for direction in valid_pipe_directions {
            let potential_point = (self.grid_point.position.0 + direction.0, self.grid_point.position.1 + direction.1);
            
            // Filter out invalid points & already visited points 
            if potential_point.0 >= 0 && potential_point.1 >= 0 
                && potential_point.0 < self.graph.len() as isize 
                && potential_point.1 < self.graph[0].len() as isize 
                && self.graph[potential_point.0 as usize][potential_point.1 as usize] != '.' {
                    let potential_val = self.graph[potential_point.0 as usize][potential_point.1 as usize] ;
                    let new_grid_point = GridPoint{ val: potential_val , position:potential_point, count: self.grid_point.count + 1 };
                    // determine if the pipe shapes can link pipes -J links but not -|
                    let inverted_directions: Vec<(isize, isize)> = pipe_directions.get(&potential_val).unwrap()
                        .iter()
                        .map(|(x, y)| (-x, -y))  // Apply the negation to both parts of the tuple
                        .collect();
                    if inverted_directions.contains(direction) 
                        && !self.visited_2.contains(&new_grid_point){   
                            valid_points.push(new_grid_point.clone());
                            

                    }
            }  
        }
        // Define return value: Return a grid if there is a valid path, return 0 if it has completed a loop. 
        if valid_points.len() > 0 {
            Some(valid_points[0].clone())
        }
        else{
            None
        }
    }
}
impl Node{
    fn shoelace_formula(& self) -> isize {
        let n = self.visited_2.len();

        let mut sum_1 = 0;
        let mut sum_2 = 0;

        for i in 0..n { 
            let (y1, x1) = self.visited_2[i].position;
            let (y2, x2) = self.visited_2[(i + 1) % n].position;
            sum_1 += x1 * y2;
            sum_2 += x2 * y1; 
        }
        (sum_1 - sum_2 ).abs() / 2  
    }
}

impl Node{
    fn problem_2(&mut self) -> isize {
        self.set_initial_start();
        let mut current_grid = self.get_valid_directions_2(); // tracks remaining paths to visit 

        // Get vector of continous path in order 
        while let Some(grid_point) = current_grid {
            self.grid_point = grid_point.clone();
            self.visited_2.push(grid_point.clone());
            current_grid = self.get_valid_directions_2();
        }

    // Calculate Area of polygon using the Shoelace formula https://en.wikipedia.org/wiki/Shoelace_formula
    let a = self.shoelace_formula();

    // Calculate total points in area using Pick's theorem https://en.wikipedia.org/wiki/Pick%27s_theorem
    let b = self.visited_2.len() as isize; 
    a + 1 - b/2 
}
}

fn main(){
    let file_data:String = fs::read_to_string("./puzzle_inputs/day_10.txt").expect("Failed to read file");
   
    let graph: Vec<Vec<char>> = file_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut node = Node{grid_point: GridPoint{val: '.', position:(0,0), count:0 },   graph:graph.clone(), visited: HashSet::new(), visited_2: Vec::new()} ;
    
    let answer_1 = node.problem_1(); 
    assert_eq!(answer_1, 6864);
    println!("Problem 1 answer {:?}", answer_1 );

    // Problem 2
    let answer_2= node.problem_2();
    assert_eq!(answer_2, 349);
    println!("Problem 2 answer {:?}", answer_2);

    }
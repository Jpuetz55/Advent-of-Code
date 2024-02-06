use std::collections::{ HashSet, VecDeque };
use crate::{ data::load, Error };

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
  #[error("Could not locate digit.")]
  NoDigits,
}

// Main puzzle-solving function
pub fn puzzle(input_data: &str) -> Result<isize, PuzzleErr> {
  calculate_total(input_data)
}

// Function to calculate total winnings
fn calculate_total(input_data: &str) -> Result<isize, PuzzleErr> {
  // Define the size of the matrix
  let rows = 140;
  let cols = 140;

  // Construct a 140x140 matrix filled with zeros
  let mut matrix: Vec<Vec<char>> = vec![vec!['0'; cols]; rows];

  // Convert the string into a 140x140 matrix
  let input_data = input_data.replace('\n', "");
  println!("Input data: {:?}", input_data);
  let mut index = 0;

  for i in 0..rows {
    for j in 0..cols {
      matrix[i][j] = input_data.chars().nth(index).unwrap();
      index += 1;
    }
  }

  // Print the matrix
  println!("Matrix: {:?}", matrix);

  println!("Matrix, row 5, column 5: {}", matrix[5][5]);

  /* struct Graph {
    adj_list: Vec<Vec<usize>>,
  }

  impl Graph {
    fn new(vertices: usize) -> Self {
      Graph {
        adj_list: vec![Vec::new(); vertices],
      }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
      self.adj_list[u].push(v);
    }

    fn dfs(
      &self,
      start: usize,
      visited: &mut HashSet<usize>,
      stack: &mut VecDeque<usize>,
      steps: &mut Vec<usize>
    ) -> Option<usize> {
      visited.insert(start);
      stack.push_back(start);

      for &neighbor in &self.adj_list[start] {
        if !visited.contains(&neighbor) {
          steps[neighbor] = steps[start] + 1;
          if let Some(cycle_length) = self.dfs(neighbor, visited, stack, steps) {
            return Some(cycle_length);
          }
        } else if stack.contains(&neighbor) {
          // Found a back edge, indicating the presence of a cycle
          return Some(steps[start] - steps[neighbor] + 1);
        }
      }

      stack.pop_back();
      None // No cycle found
    }

    fn find_cycle(&self) -> Option<usize> {
      let mut visited = HashSet::new();
      let mut stack = VecDeque::new();
      let mut steps = vec![0; self.adj_list.len()];

      for vertex in 0..self.adj_list.len() {
        if !visited.contains(&vertex) {
          if let Some(cycle_length) = self.dfs(vertex, &mut visited, &mut stack, &mut steps) {
            return Some(cycle_length);
          }
        }
      }

      None // No cycle found
    }
  }

  let mut graph = Graph::new(6);
  graph.add_edge(0, 1);
  graph.add_edge(0, 2);
  graph.add_edge(1, 3);
  graph.add_edge(2, 4);
  graph.add_edge(4, 2); // Introduce a cycle by adding an edge back to vertex 2
  graph.add_edge(2, 5);

  if let Some(cycle_length) = graph.find_cycle() {
    println!("Cycle found with length: {}", cycle_length);
  } else {
    println!("No cycle found.");
  } */

  Ok(0)
}

// Main function to execute the puzzle
pub fn main(data_dir: &str) {
  let data = load(data_dir, 101, None);

  let answer = puzzle(&data);
  match answer {
    Ok(x) => println!(" Puzzle 9-2: {}", x),
    Err(e) => panic!("No solution to puzzle: {}.", e),
  }
  assert_eq!(answer, Ok(1953784198));
}

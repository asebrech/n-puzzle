use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Puzzle {
    state: Vec<u8>,
    empty_pos: usize,
    g: usize, // cost to reach this state
    h: usize, // heuristic cost to reach the goal
}

impl Puzzle {
    fn new(state: Vec<u8>, empty_pos: usize) -> Self {
        let g = 0; // Starting cost
        let h = Self::heuristic(&state);
        Puzzle { state, empty_pos, g, h }
    }

    fn heuristic(state: &[u8]) -> usize {
        // Snake pattern heuristic
        let goal = [
            1, 2, 3,
            8, 0, 4,
            7, 6, 5,
        ];
        let mut h = 0;
        for (i, &value) in state.iter().enumerate() {
            if value != 0 {
                let goal_pos = goal.iter().position(|&x| x == value).unwrap();
                let row_diff = (i / 3) as isize - (goal_pos / 3) as isize;
                let col_diff = (i % 3) as isize - (goal_pos % 3) as isize;
                h += row_diff.abs() as usize + col_diff.abs() as usize;
            }
        }
        h
    }

    fn possible_moves(&self) -> Vec<Puzzle> {
        let mut moves = Vec::new();
        let (x, y) = (self.empty_pos / 3, self.empty_pos % 3);
        let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        
        for (dx, dy) in &directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0 && new_x < 3 && new_y >= 0 && new_y < 3 {
                let new_pos = (new_x * 3 + new_y) as usize;
                let mut new_state = self.state.clone();
                new_state.swap(self.empty_pos, new_pos);
                moves.push(Puzzle::new(new_state, new_pos));
            }
        }
        
        moves
    }
}

impl Ord for Puzzle {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.g + self.h).cmp(&(other.g + other.h)).reverse()
    }
}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(start: Puzzle) -> Option<Vec<Puzzle>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();

    open_set.push(start);

    while let Some(current) = open_set.pop() {
        if current.h == 0 {
            let mut solution_path = Vec::new();
            let mut temp = current;
            while temp.h != 0 {
                solution_path.push(temp.clone());
                let next = temp.possible_moves();
                temp = next.into_iter().min_by_key(|p| p.g + p.h).unwrap();
            }
            solution_path.push(temp);
            solution_path.reverse();
            return Some(solution_path);
        }
        
        closed_set.insert(current.clone());
        
        for next in current.possible_moves() {
            if closed_set.contains(&next) {
                continue;
            }
            let next_g = current.g + 1;
            let mut next_puzzle = next.clone();
            next_puzzle.g = next_g;
            if !open_set.iter().any(|p| p.state == next.state && p.g <= next_g) {
                open_set.push(next_puzzle.clone());
                print_puzzle(&next_puzzle); // Print the puzzle after each move
            }
        }
    }
    None
}

fn print_puzzle(puzzle: &Puzzle) {
    for i in 0..3 {
        for j in 0..3 {
            let value = puzzle.state[i * 3 + j];
            if value == 0 {
                print!("   ");
            } else {
                print!("{:2} ", value);
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let start_state = vec![1, 2, 3, 0, 8, 4, 7, 6, 5];
    let empty_pos = start_state.iter().position(|&x| x == 0).unwrap();
    let start_puzzle = Puzzle::new(start_state, empty_pos);

    println!("Initial Puzzle:");
    print_puzzle(&start_puzzle);

    if let Some(solution) = a_star(start_puzzle) {
        println!("Solution Steps:");
        for step in solution {
            print_puzzle(&step);
        }
    } else {
        println!("No solution found!");
    }
}


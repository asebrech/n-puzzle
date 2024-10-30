use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Puzzle {
    state: Vec<u8>,
    empty_pos: usize,
    g: usize,                    // cost to reach this state
    h: usize,                    // heuristic cost to reach the goal
    parent: Option<Box<Puzzle>>, // pointer to parent state
}

impl Puzzle {
    fn new(state: Vec<u8>, empty_pos: usize, parent: Option<Box<Puzzle>>) -> Self {
        let g = parent.as_ref().map_or(0, |p| p.g + 1); // Update cost based on parent
        let h = Self::heuristic(&state);
        Puzzle {
            state,
            empty_pos,
            g,
            h,
            parent,
        }
    }

    fn heuristic(state: &[u8]) -> usize {
        let goal = [1, 2, 3, 8, 0, 4, 7, 6, 5];
        state
            .iter()
            .enumerate()
            .filter(|(_, &value)| value != 0)
            .map(|(i, &value)| {
                let goal_pos = goal.iter().position(|&x| x == value).unwrap();
                let row_diff = (i / 3) as isize - (goal_pos / 3) as isize;
                let col_diff = (i % 3) as isize - (goal_pos % 3) as isize;
                row_diff.unsigned_abs() + col_diff.unsigned_abs()
            })
            .sum()
    }

    fn possible_moves(&self) -> Vec<Puzzle> {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let (x, y) = (self.empty_pos / 3, self.empty_pos % 3);

        directions
            .iter()
            .filter_map(|(dx, dy)| {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if (0..3).contains(&new_x) && (0..3).contains(&new_y) {
                    let new_pos = (new_x * 3 + new_y) as usize;
                    let mut new_state = self.state.clone();
                    new_state.swap(self.empty_pos, new_pos);
                    Some(Puzzle::new(
                        new_state,
                        new_pos,
                        Some(Box::new(self.clone())),
                    ))
                } else {
                    None
                }
            })
            .collect()
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
            return Some(reconstruct_path(&current));
        }

        closed_set.insert(current.clone());

        for next in current.possible_moves() {
            if !closed_set.contains(&next)
                && !open_set
                    .iter()
                    .any(|p| p.state == next.state && p.g <= next.g)
            {
                open_set.push(next);
            }
        }
    }
    None
}

fn reconstruct_path(goal: &Puzzle) -> Vec<Puzzle> {
    let mut path = vec![goal.clone()];
    let mut current = goal;
    while let Some(parent) = &current.parent {
        path.push((**parent).clone());
        current = parent;
    }
    path.reverse();
    path
}

fn print_puzzle(puzzle: &Puzzle) {
    for i in 0..3 {
        for j in 0..3 {
            if puzzle.state[i * 3 + j] == 0 {
                print!("   ");
            } else {
                print!("{:2} ", puzzle.state[i * 3 + j]);
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let start_state = vec![4, 7, 3, 0, 8, 1, 2, 6, 5];
    let empty_pos = start_state.iter().position(|&x| x == 0).unwrap();
    let start_puzzle = Puzzle::new(start_state, empty_pos, None);

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


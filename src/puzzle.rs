use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Puzzle {
    state: Vec<u8>,
    empty_pos: usize,
    g: usize,
    h: usize,
    parent: Option<Box<Puzzle>>,
    goal: Vec<u8>,
    size: usize,
}

impl Puzzle {
    pub fn new(
        state: Vec<u8>,
        empty_pos: usize,
        parent: Option<Box<Puzzle>>,
        goal: Vec<u8>,
        size: usize,
    ) -> Self {
        let g = parent.as_ref().map_or(0, |p| p.g + 1);
        let h = Self::heuristic(&state, &goal);
        Puzzle {
            state,
            empty_pos,
            g,
            h,
            parent,
            goal,
            size,
        }
    }

    pub fn heuristic(state: &[u8], goal: &[u8]) -> usize {
        state
            .iter()
            .enumerate()
            .filter(|(_, &value)| value != 0)
            .map(|(i, &value)| {
                let goal_pos = goal.iter().position(|&x| x == value).unwrap();
                let row_diff = (i / state.len()).abs_diff(goal_pos / goal.len());
                let col_diff = (i % state.len()).abs_diff(goal_pos % goal.len());
                row_diff + col_diff
            })
            .sum()
    }

    pub fn possible_moves(&self) -> Vec<Puzzle> {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let (x, y) = (self.empty_pos / self.size, self.empty_pos % self.size);

        directions
            .iter()
            .filter_map(|(dx, dy)| {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if (0..self.size as isize).contains(&new_x)
                    && (0..self.size as isize).contains(&new_y)
                {
                    let new_pos = (new_x * self.size as isize + new_y) as usize;
                    let mut new_state = self.state.clone();
                    new_state.swap(self.empty_pos, new_pos);
                    Some(Puzzle::new(
                        new_state,
                        new_pos,
                        Some(Box::new(self.clone())),
                        self.goal.clone(),
                        self.size,
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

pub fn a_star(start: Puzzle) -> Option<Vec<Puzzle>> {
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

pub fn print_puzzle(puzzle: &Puzzle) {
    for i in 0..puzzle.size {
        for j in 0..puzzle.size {
            if puzzle.state[i * puzzle.size + j] == 0 {
                print!("   ");
            } else {
                print!("{:2} ", puzzle.state[i * puzzle.size + j]);
            }
        }
        println!();
    }
    println!();
}

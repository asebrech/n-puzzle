pub mod puzzle;
pub mod utils;

use puzzle::{a_star, print_puzzle, Puzzle};
use std::env;
use utils::{make_goal, make_puzzle};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        utils::print_help(&args[0]);
        return;
    }

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        utils::print_help(&args[0]);
        return;
    }

    let default_solvable = true;
    let default_iterations = 20;

    let mut size: Option<usize> = None;
    let mut solvable = default_solvable;
    let mut iterations = default_iterations;

    for arg in &args[1..] {
        match arg.parse::<usize>() {
            Ok(num) => {
                if size.is_none() {
                    size = Some(num);
                } else {
                    iterations = num;
                }
            }
            Err(_) => {
                if arg == "true" {
                    solvable = true;
                } else if arg == "false" {
                    solvable = false;
                }
            }
        }
    }

    let size = size.expect("Please provide a valid number for size.");

    let goal_state = make_goal(size);
    let start_state = make_puzzle(size, solvable, iterations);

    let start_state: Vec<u8> = start_state.into_iter().map(|x| x as u8).collect();
    let goal_state: Vec<u8> = goal_state.into_iter().map(|x| x as u8).collect();

    let empty_pos = start_state.iter().position(|&x| x == 0).unwrap();
    let start_puzzle = Puzzle::new(start_state, empty_pos, None, goal_state, size);

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

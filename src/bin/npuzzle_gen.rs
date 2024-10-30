use n_puzzle_solver::make_puzzle;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <size> [-s] [-u] [-i <iterations>]", args[0]);
        process::exit(1);
    }

    let size: usize = args[1].parse().expect("Size must be an integer");
    let mut solvable = false;
    let mut unsolvable = false;
    let mut iterations = 10000;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-s" => solvable = true,
            "-u" => unsolvable = true,
            "-i" => {
                i += 1;
                iterations = args[i].parse().expect("Iterations must be an integer");
            }
            _ => {}
        }
        i += 1;
    }

    if solvable && unsolvable {
        eprintln!("Can't be both solvable AND unsolvable, dummy!");
        process::exit(1);
    }

    if size < 3 {
        eprintln!("Can't generate a puzzle with size lower than 2. Dummy.");
        process::exit(1);
    }

    let solv = if solvable {
        true
    } else if unsolvable {
        false
    } else {
        rand::random()
    };

    let puzzle = make_puzzle(size, solv, iterations);

    println!(
        "# This puzzle is {}",
        if solv { "solvable" } else { "unsolvable" }
    );
    println!("{}", size);
    let width = (size * size).to_string().len();

    for y in 0..size {
        for x in 0..size {
            print!("{:width$} ", puzzle[x + y * size], width = width);
        }
        println!();
    }
}

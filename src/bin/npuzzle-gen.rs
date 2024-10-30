use rand::seq::SliceRandom;
use std::env;
use std::process;

pub fn make_puzzle(s: usize, solvable: bool, iterations: usize) -> Vec<usize> {
    fn swap_empty(p: &mut [usize], s: usize) {
        let idx = p.iter().position(|&x| x == 0).unwrap();
        let mut poss = Vec::new();

        if idx % s > 0 {
            poss.push(idx as isize - 1);
        }
        if idx % s < s - 1 {
            poss.push(idx as isize + 1);
        }
        if idx >= s {
            poss.push(idx as isize - s as isize);
        }
        if idx + s < p.len() {
            poss.push(idx as isize + s as isize);
        }

        let &swi = poss.choose(&mut rand::thread_rng()).unwrap();
        p.swap(idx, swi as usize);
    }

    let mut p = make_goal(s);
    for _ in 0..iterations {
        swap_empty(&mut p[..], s);
    }

    if !solvable {
        let len = p.len();
        if p[0] == 0 || p[1] == 0 {
            p.swap(len - 1, len - 2);
        } else {
            p.swap(0, 1);
        }
    }

    p
}

pub fn make_goal(s: usize) -> Vec<usize> {
    let ts = s * s;
    let mut puzzle = vec![-1_isize; ts];
    let mut cur = 1;
    let mut x: isize = 0;
    let mut ix: isize = 1;
    let mut y: isize = 0;
    let mut iy: isize = 0;

    loop {
        puzzle[(x + y * s as isize) as usize] = cur as isize;
        if cur == 0 {
            break;
        }
        cur += 1;

        if x + ix == s as isize
            || x + ix < 0
            || (ix != 0 && puzzle[(x + ix + y * s as isize) as usize] != -1)
        {
            iy = ix;
            ix = 0;
        } else if y + iy == s as isize
            || y + iy < 0
            || (iy != 0 && puzzle[(x + (y + iy) * s as isize) as usize] != -1)
        {
            ix = -iy;
            iy = 0;
        }

        x += ix;
        y += iy;

        if cur == ts {
            cur = 0;
        }
    }

    puzzle.iter().map(|&x| x as usize).collect()
}

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

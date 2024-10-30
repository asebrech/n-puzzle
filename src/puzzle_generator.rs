use rand::seq::SliceRandom;

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

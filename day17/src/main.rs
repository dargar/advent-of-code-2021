use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> i32 {
    let rhs = input.trim().split(": ").skip(1).next().unwrap();
    let mut parts = rhs.split(", ");
    let xs = parts.next().unwrap().split("x=").skip(1).next().unwrap();
    let ys = parts.next().unwrap().split("y=").skip(1).next().unwrap();
    let mut parts = xs.split("..");
    let xmin = parts.next().map(|x| x.parse::<i32>().unwrap()).unwrap();
    let xmax = parts.next().map(|x| x.parse::<i32>().unwrap()).unwrap();
    let mut parts = ys.split("..");
    let ymin = parts.next().map(|y| y.parse::<i32>().unwrap()).unwrap();
    let ymax = parts.next().map(|y| y.parse::<i32>().unwrap()).unwrap();

    let mut target = HashSet::new();
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            target.insert((x, y));
        }
    }

    let mut xhits = HashSet::new();
    let maxdx = xmax.abs();
    for cvx in 0..=maxdx {
        let mut x = 0;
        let mut vx = cvx;
        while x <= xmax && vx > 0 {
            if xmin <= x && x <= xmax {
                xhits.insert(cvx);
            }
            x += vx;
            vx -= 1;
        }
    }

    for cyv in (0..ymin.abs()).rev() {
        for vx in &xhits {
            let mut x = 0;
            let mut y = 0;
            let mut vx = *vx;
            let mut vy = cyv;
            let mut max_height = i32::MIN;
            while x <= xmax && y >= ymin {
                max_height = std::cmp::max(max_height, y);
                if (xmin <= x && x <= xmax) && (ymin <= y && y <= ymax) {
                    return max_height;
                }
                x += vx;
                y += vy;
                vx = std::cmp::max(0, vx - 1);
                vy -= 1;
            }
        }
    }

    unreachable!();
}

fn second_answer(input: &str) -> usize {
    let rhs = input.trim().split(": ").skip(1).next().unwrap();
    let mut parts = rhs.split(", ");
    let xs = parts.next().unwrap().split("x=").skip(1).next().unwrap();
    let ys = parts.next().unwrap().split("y=").skip(1).next().unwrap();
    let mut parts = xs.split("..");
    let xmin = parts.next().map(|x| x.parse::<i32>().unwrap()).unwrap();
    let xmax = parts.next().map(|x| x.parse::<i32>().unwrap()).unwrap();
    let mut parts = ys.split("..");
    let ymin = parts.next().map(|y| y.parse::<i32>().unwrap()).unwrap();
    let ymax = parts.next().map(|y| y.parse::<i32>().unwrap()).unwrap();

    let mut target = HashSet::new();
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            target.insert((x, y));
        }
    }

    let mut xhits = HashSet::new();
    let maxdx = xmax.abs();
    for cvx in 0..=maxdx {
        let mut x = 0;
        let mut vx = cvx;
        while x <= xmax && vx > 0 {
            if xmin <= x && x <= xmax {
                xhits.insert(cvx);
            }
            x += vx;
            vx -= 1;
        }
    }

    let mut hits = Vec::new();
    for cyv in (ymin..ymin.abs()).rev() {
        for cvx in &xhits {
            let mut x = 0;
            let mut y = 0;
            let mut vx = *cvx;
            let mut vy = cyv;
            let mut max_height = i32::MIN;
            while x <= xmax && y >= ymin {
                max_height = std::cmp::max(max_height, y);
                if (xmin <= x && x <= xmax) && (ymin <= y && y <= ymax) {
                    hits.push((cvx, cyv));
                    break;
                }
                x += vx;
                y += vy;
                vx = std::cmp::max(0, vx - 1);
                vy -= 1;
            }
        }
    }
    hits.len()
}

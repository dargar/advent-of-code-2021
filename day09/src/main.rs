use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let width = map[0].len();
    let height = map.len();
    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            let left = if x > 0 { map[y][x - 1] } else { std::u32::MAX };
            let right = if x < width - 1 {
                map[y][x + 1]
            } else {
                std::u32::MAX
            };
            let top = if y > 0 { map[y - 1][x] } else { std::u32::MAX };
            let bottom = if y < height - 1 {
                map[y + 1][x]
            } else {
                std::u32::MAX
            };
            let centre = map[y][x];
            if centre < left && centre < right && centre < top && centre < bottom {
                result += 1 + centre;
            }
        }
    }
    result
}

fn second_answer(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let width = map[0].len();
    let height = map.len();
    let mut low_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let left = if x > 0 { map[y][x - 1] } else { std::u32::MAX };
            let right = if x < width - 1 {
                map[y][x + 1]
            } else {
                std::u32::MAX
            };
            let top = if y > 0 { map[y - 1][x] } else { std::u32::MAX };
            let bottom = if y < height - 1 {
                map[y + 1][x]
            } else {
                std::u32::MAX
            };
            let centre = map[y][x];
            if centre < left && centre < right && centre < top && centre < bottom {
                low_points.push((x, y));
            }
        }
    }
    let mut basins = Vec::new();
    for low_point in low_points {
        let mut basin = HashSet::new();
        let mut visited = HashSet::from([low_point]);
        let mut queue = VecDeque::from([low_point]);
        while let Some(point) = queue.pop_front() {
            basin.insert(point);
            visited.insert(point);
            let neighbours = &neighbours(width, height, point) - &visited;
            for (x, y) in neighbours {
                if map[y][x] < 9 {
                    queue.push_back((x, y));
                }
            }
        }
        basins.push(basin);
    }
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    basins.dedup();
    basins[..3].iter().map(|basin| basin.len()).product()
}

fn neighbours(width: usize, height: usize, (x, y): (usize, usize)) -> HashSet<(usize, usize)> {
    let mut neighbours = HashSet::new();
    if x > 0 {
        neighbours.insert((x - 1, y));
    }
    if x < width - 1 {
        neighbours.insert((x + 1, y));
    }
    if y > 0 {
        neighbours.insert((x, y - 1));
    }
    if y < height - 1 {
        neighbours.insert((x, y + 1));
    }
    neighbours
}

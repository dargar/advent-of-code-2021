fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> u32 {
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        let mut rows = Vec::new();
        for _ in 0..5 {
            rows.push(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
        boards.push(rows);
    }

    let (board, n, marked) = find_winning_bingo_board(&numbers, &boards);
    let mut sum_unmarked = 0;
    for (y, row) in boards[board].iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if !marked.contains(&(board, x, y)) {
                sum_unmarked += col;
            }
        }
    }

    n * sum_unmarked
}

fn find_winning_bingo_board(
    numbers: &[u32],
    boards: &Vec<Vec<Vec<u32>>>,
) -> (usize, u32, Vec<(usize, usize, usize)>) {
    let mut marked = Vec::new();
    for n in numbers {
        for (board, rows) in boards.iter().enumerate() {
            for (row, cols) in rows.iter().enumerate() {
                for (col, value) in cols.iter().enumerate() {
                    if n == value {
                        marked.push((board, col, row));
                        if marked
                            .iter()
                            .filter(|(b, x, _)| *b == board && *x == col)
                            .count()
                            == 5
                        {
                            return (
                                board,
                                *n,
                                marked
                                    .iter()
                                    .filter(|(b, _, _)| *b == board)
                                    .cloned()
                                    .collect(),
                            );
                        }
                        if marked
                            .iter()
                            .filter(|(b, _, y)| *b == board && *y == row)
                            .count()
                            == 5
                        {
                            return (
                                board,
                                *n,
                                marked
                                    .iter()
                                    .filter(|(b, _, _)| *b == board)
                                    .cloned()
                                    .collect(),
                            );
                        }
                    }
                }
            }
        }
    }
    unreachable!();
}

fn second_answer(input: &str) -> u32 {
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        let mut rows = Vec::new();
        for _ in 0..5 {
            rows.push(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
        boards.push(rows);
    }

    let (board, n, marked) = find_losing_bingo_board(&numbers, &boards);
    let mut sum_unmarked = 0;
    for (y, row) in boards[board].iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if !marked.contains(&(board, x, y)) {
                sum_unmarked += col;
            }
        }
    }

    n * sum_unmarked
}

fn find_losing_bingo_board(
    numbers: &[u32],
    boards: &Vec<Vec<Vec<u32>>>,
) -> (usize, u32, Vec<(usize, usize, usize)>) {
    let mut marked = Vec::new();
    let mut winning_boards = Vec::new();
    let mut winning_ns = Vec::new();
    for n in numbers {
        for (board, rows) in boards.iter().enumerate() {
            if winning_boards.contains(&board) {
                continue;
            }
            for (row, cols) in rows.iter().enumerate() {
                for (col, value) in cols.iter().enumerate() {
                    if n == value {
                        marked.push((board, col, row));
                        if marked
                            .iter()
                            .filter(|(b, x, _)| *b == board && *x == col)
                            .count()
                            == 5
                        {
                            winning_boards.push(board);
                            winning_ns.push(*n);
                        }
                        if marked
                            .iter()
                            .filter(|(b, _, y)| *b == board && *y == row)
                            .count()
                            == 5
                        {
                            winning_boards.push(board);
                            winning_ns.push(*n);
                        }
                    }
                }
            }
        }
    }
    let losing_board = winning_boards.last().unwrap();
    (
        *losing_board,
        *winning_ns.last().unwrap(),
        marked
            .iter()
            .filter(|(b, _, _)| *b == *losing_board)
            .cloned()
            .collect(),
    )
}

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> usize {
    let mut positions = input
        .lines()
        .flat_map(|line| line.split(": ").skip(1).flat_map(|p| p.parse::<usize>()))
        .collect::<Vec<usize>>();
    let mut deterministic_die = (1..=100).cycle();
    let mut scores = vec![0; 2];
    for turn in 1.. {
        let player = (turn - 1) % 2;
        let roll1 = deterministic_die.next().unwrap();
        let roll2 = deterministic_die.next().unwrap();
        let roll3 = deterministic_die.next().unwrap();
        positions[player] += roll1 + roll2 + roll3;
        while positions[player] > 10 {
            positions[player] -= 10;
        }
        scores[player] += positions[player];
        if scores[player] >= 1000 {
            return scores[(player + 1) % 2] * (turn * 3);
        }
    }
    unreachable!();
}

fn second_answer(input: &str) -> usize {
    let positions = input
        .lines()
        .flat_map(|line| line.split(": ").skip(1).flat_map(|p| p.parse::<usize>()))
        .collect::<Vec<usize>>();
    let (left, right) = count_wins(positions[0], 0, positions[1], 0);
    std::cmp::max(left, right)
}

fn count_wins(
    curr_pos: usize,
    curr_score: usize,
    next_pos: usize,
    next_score: usize,
) -> (usize, usize) {
    let three_dice = vec![3, 4, 5, 6, 7, 8, 9];
    let likelyhoods = vec![1, 3, 6, 7, 6, 3, 1];
    let outcomes = three_dice
        .into_iter()
        .zip(likelyhoods.into_iter())
        .map(|(dice, likelyhood)| {
            let mut updated_pos = curr_pos + dice;
            while updated_pos > 10 {
                updated_pos -= 10;
            }
            (updated_pos, curr_score + updated_pos, likelyhood)
        })
        .collect::<Vec<_>>();
    let (wins, undecided): (Vec<(usize, usize, usize)>, Vec<(usize, usize, usize)>) =
        outcomes.into_iter().partition(|(_, score, _)| *score >= 21);
    let outright_left_wins: usize = wins.into_iter().map(|(_, _, likelyhood)| likelyhood).sum();
    let (left_wins, right_wins) = undecided
        .into_iter()
        .map(|(updated_pos, updated_score, likelyhood)| {
            (
                count_wins(next_pos, next_score, updated_pos, updated_score),
                likelyhood,
            )
        })
        .map(|((left_wins, right_wins), likelyhood)| {
            (right_wins * likelyhood, left_wins * likelyhood)
        })
        .fold((0, 0), |(lws, rws), (lw, rw)| (lws + lw, rws + rw));
    (outright_left_wins + left_wins, right_wins)
}

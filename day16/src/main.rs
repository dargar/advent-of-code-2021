fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> u64 {
    let chunks = input.trim().chars().map(to_u8).collect::<Vec<u8>>();
    let packet = chunks
        .chunks(2)
        .map(|cs| cs[0] << 4 | cs[1])
        .collect::<Vec<u8>>();
    let bitvec = packet
        .into_iter()
        .flat_map(|b| {
            let mut bits = Vec::new();
            for i in 0..8 {
                bits.push((1 << (7 - i) & b) != 0);
            }
            bits
        })
        .collect::<Vec<bool>>();
    let mut bits = bitvec.into_iter();
    let (exp, _) = pkt(&mut bits);
    exp.version_sum()
}

fn second_answer(input: &str) -> u64 {
    let chunks = input.trim().chars().map(to_u8).collect::<Vec<u8>>();
    let packet = chunks
        .chunks(2)
        .map(|cs| cs[0] << 4 | cs[1])
        .collect::<Vec<u8>>();
    let bitvec = packet
        .into_iter()
        .flat_map(|b| {
            let mut bits = Vec::new();
            for i in 0..8 {
                bits.push((1 << (7 - i) & b) != 0);
            }
            bits
        })
        .collect::<Vec<bool>>();
    let mut bits = bitvec.into_iter();
    let (exp, _) = pkt(&mut bits);
    exp.eval()
}

#[derive(Debug)]
enum Exp {
    Lit(u64, u64),
    Op(u64, u64, Vec<Exp>),
}

impl Exp {
    fn version_sum(&self) -> u64 {
        match self {
            Exp::Lit(v, _) => *v,
            Exp::Op(v, _, es) => *v + es.iter().map(|e| e.version_sum()).sum::<u64>(),
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Exp::Lit(_, l) => *l,
            Exp::Op(_, t, es) => match t {
                0 => es.iter().map(Exp::eval).sum::<u64>(),
                1 => es.iter().map(Exp::eval).product::<u64>(),
                2 => es.iter().map(Exp::eval).min().unwrap(),
                3 => es.iter().map(Exp::eval).max().unwrap(),
                5 => {
                    let vals = es.iter().map(Exp::eval).collect::<Vec<u64>>();
                    if vals[0] > vals[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let vals = es.iter().map(Exp::eval).collect::<Vec<u64>>();
                    if vals[0] < vals[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let vals = es.iter().map(Exp::eval).collect::<Vec<u64>>();
                    if vals[0] == vals[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            },
        }
    }
}

fn pkt(bits: &mut std::vec::IntoIter<bool>) -> (Exp, usize) {
    let version = take(3, bits);
    let type_id = take(3, bits);
    match type_id {
        4 => {
            let mut ls = 0;
            let mut literal = 0u64;
            loop {
                let continues = take(1, bits);
                let value = take(4, bits);
                literal <<= 4 as u64;
                literal |= value as u64;
                ls += 1;
                if continues == 0 {
                    break;
                }
            }
            (Exp::Lit(version, literal), 3 + 3 + 5 * ls)
        }
        _ => {
            let length_type_id = take(1, bits);
            let mut subpkts = Vec::new();
            let mut total_bits = 0;
            if length_type_id == 0 {
                let mut length_subpkts = take(15, bits) as usize;
                while length_subpkts > 0 {
                    let (exp, ls) = pkt(bits);
                    subpkts.push(exp);
                    length_subpkts -= ls;
                    total_bits += ls;
                }
                (
                    Exp::Op(version, type_id, subpkts),
                    3 + 3 + 1 + 15 + total_bits,
                )
            } else {
                let num_subpkts = take(11, bits) as usize;
                for _ in 0..num_subpkts {
                    let (exp, ls) = pkt(bits);
                    subpkts.push(exp);
                    total_bits += ls;
                }
                (
                    Exp::Op(version, type_id, subpkts),
                    3 + 3 + 1 + 11 + total_bits,
                )
            }
        }
    }
}

fn take(n: usize, bits: &mut std::vec::IntoIter<bool>) -> u64 {
    let mut result = 0;
    for i in 0..n {
        if bits.next().unwrap() {
            result |= 1 << n - 1 - i;
        }
    }
    result
}

fn to_u8(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => panic!(),
    }
}

fn main() {
    let input = include_str!("../input");
    println!("First answer: {}", first_answer(input));
    println!("Second answer: {}", second_answer(input));
}

fn first_answer(input: &str) -> u32 {
    let exps = input.lines().map(parse).map(reduce).collect::<Vec<_>>();
    let mut exps = exps.into_iter();
    let mut exp = exps.next().unwrap();
    for next in exps {
        let e = Exp::Pair(Box::new(exp.clone()), Box::new(next));
        exp = reduce(e);
    }
    magnitude(exp)
}

fn second_answer(input: &str) -> u32 {
    let exps = input.lines().map(parse).map(reduce).collect::<Vec<_>>();
    let mut result = 0;
    for (i, a) in exps.iter().enumerate() {
        for (j, b) in exps.iter().enumerate() {
            if i != j {
                let e = Exp::Pair(Box::new(a.clone()), Box::new(b.clone()));
                let ee = reduce(e);
                result = std::cmp::max(result, magnitude(ee));
                let e = Exp::Pair(Box::new(b.clone()), Box::new(a.clone()));
                let ee = reduce(e);
                result = std::cmp::max(result, magnitude(ee));
            }
        }
    }
    result
}

#[derive(Clone, PartialEq, Eq)]
enum Exp {
    Pair(Box<Exp>, Box<Exp>),
    Num(u32),
}

impl Exp {
    fn num(&self) -> Option<u32> {
        match self {
            Exp::Pair(_, _) => None,
            Exp::Num(n) => Some(*n),
        }
    }
}

impl std::fmt::Debug for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Pair(a, b) => f.write_fmt(format_args!("[{:?},{:?}]", a, b)),
            Exp::Num(n) => f.write_fmt(format_args!("{}", n)),
        }
    }
}

fn parse(s: &str) -> Exp {
    let mut output = Vec::new();
    let mut operators = Vec::new();
    for c in s.chars() {
        match c {
            '[' => operators.push(c),
            ']' => {
                let a = output.pop().unwrap();
                let b = output.pop().unwrap();
                output.push(Exp::Pair(Box::new(b), Box::new(a)));
            }
            ',' => (),
            _ => output.push(Exp::Num(c.to_digit(10).unwrap())),
        }
    }
    output.pop().unwrap()
}

fn reduce(exp: Exp) -> Exp {
    let mut e = exp;
    loop {
        let (ee, r) = explode(e.clone(), 0, Reduction::Nop);
        if r == Reduction::Nop {
            let (eee, r) = split(ee, 0, Reduction::Nop);
            if r == Reduction::Nop {
                return eee;
            } else {
                e = eee;
            }
        } else {
            e = ee;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Reduction {
    Explode(Option<u32>, Option<u32>),
    Left(Option<u32>),
    Right(Option<u32>),
    Split,
    Nop,
    Done,
}

fn explode(exp: Exp, depth: usize, reduction: Reduction) -> (Exp, Reduction) {
    assert_eq!(Reduction::Nop, reduction);
    assert!(depth <= 4);
    match exp {
        Exp::Pair(a, b) => {
            if depth >= 4 {
                (Exp::Num(0), Reduction::Explode(a.num(), b.num()))
            } else {
                let (aa, ar) = explode(*a.clone(), depth + 1, reduction);
                match ar {
                    Reduction::Explode(c, d) => {
                        let bb = update_left(*b, d);
                        (Exp::Pair(Box::new(aa), Box::new(bb)), Reduction::Left(c))
                    }
                    Reduction::Left(_) => (Exp::Pair(Box::new(aa), b), ar),
                    Reduction::Right(v) => {
                        let bb = update_left(*b, v);
                        (Exp::Pair(Box::new(aa), Box::new(bb)), Reduction::Done)
                    }
                    Reduction::Split | Reduction::Done => (Exp::Pair(Box::new(aa), b), ar),
                    _ => {
                        let (bb, br) = explode(*b, depth + 1, ar);
                        match br {
                            Reduction::Explode(c, d) => {
                                assert_eq!(*a, aa);
                                let aa = update_right(*a.clone(), c);
                                (Exp::Pair(Box::new(aa), Box::new(bb)), Reduction::Right(d))
                            }
                            Reduction::Left(v) => {
                                let aa = update_right(aa, v);
                                (Exp::Pair(Box::new(aa), Box::new(bb)), Reduction::Done)
                            }
                            Reduction::Right(_) => (Exp::Pair(Box::new(aa), Box::new(bb)), br),
                            _ => (Exp::Pair(Box::new(aa), Box::new(bb)), br),
                        }
                    }
                }
            }
        }
        Exp::Num(_) => (exp, reduction),
    }
}

fn split(exp: Exp, depth: usize, reduction: Reduction) -> (Exp, Reduction) {
    assert_eq!(Reduction::Nop, reduction);
    assert!(depth <= 4);
    match exp {
        Exp::Pair(a, b) => {
            let (aa, ar) = split(*a, depth + 1, reduction);
            if ar == Reduction::Split {
                (Exp::Pair(Box::new(aa), b), ar)
            } else {
                let (bb, br) = split(*b, depth + 1, reduction);
                (Exp::Pair(Box::new(aa), Box::new(bb)), br)
            }
        }
        Exp::Num(n) => {
            if n >= 10 {
                (
                    Exp::Pair(Box::new(Exp::Num(n / 2)), Box::new(Exp::Num((n + 1) / 2))),
                    Reduction::Split,
                )
            } else {
                (exp, reduction)
            }
        }
    }
}

fn update_left(exp: Exp, val: Option<u32>) -> Exp {
    if let Some(v) = val {
        match exp {
            Exp::Pair(a, b) => {
                let aa = update_left(*a, val);
                Exp::Pair(Box::new(aa), b)
            }
            Exp::Num(n) => Exp::Num(n + v),
        }
    } else {
        exp
    }
}

fn update_right(exp: Exp, val: Option<u32>) -> Exp {
    if let Some(v) = val {
        match exp {
            Exp::Pair(a, b) => {
                let bb = update_right(*b, val);
                Exp::Pair(a, Box::new(bb))
            }
            Exp::Num(n) => Exp::Num(n + v),
        }
    } else {
        exp
    }
}

fn magnitude(exp: Exp) -> u32 {
    match exp {
        Exp::Pair(a, b) => 3 * magnitude(*a) + 2 * magnitude(*b),
        Exp::Num(n) => n,
    }
}

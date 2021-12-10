use std::error::Error;

fn get_score_1(c: char) -> i64 {
    match c {
        '(' | ')' => 3,
        '[' | ']' => 57,
        '{' | '}' => 1197,
        '<' | '>' => 25137,
        _ => 0,
    }
}

fn get_score_2(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn is_opening(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn is_closing(c: char) -> bool {
    matches!(c, ')' | ']' | '}' | '>')
}

fn get_match(c: &char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => panic!("Unexpected char: {}", c),
    }
}

fn is_matching(c1: char, c2: char) -> bool {
    match c1 {
        '(' => c2 == ')',
        '[' => c2 == ']',
        '{' => c2 == '}',
        '<' => c2 == '>',
        _ => false,
    }
}

fn line_corrupted_at(line: &str) -> Option<char> {
    let mut stack = Vec::new();

    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else if is_closing(c) {
            let k = stack.pop().unwrap();
            if !is_matching(k, c) {
                return Some(c);
            }
        }
    }

    None
}

fn complete_line(line: &str) -> Vec<char> {
    let mut stack = Vec::new();

    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else if is_closing(c) {
            let k = stack.pop().unwrap();
            if !is_matching(k, c) {
                panic!("Current char {} does not match previous char {}", c, k);
            }
        }
    }

    stack.iter().map(get_match).rev().collect()
}

pub fn solve_2() -> Result<i64, Box<dyn Error>> {
    let mut scores = include_str!("../data/10.in")
        .lines()
        .filter(|line| line_corrupted_at(*line).is_none())
        .map(|incomplete_line| {
            complete_line(incomplete_line)
                .iter()
                .fold(0, |acc, c| acc * 5 + get_score_2(*c))
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();

    Ok(scores[scores.len() / 2])
}

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let lines = include_str!("../data/10.in").lines().collect::<Vec<_>>();

    let mut score = 0i64;
    for line in lines {
        if let Some(c) = line_corrupted_at(line) {
            score += get_score_1(c);
        }
    }

    Ok(score)
}
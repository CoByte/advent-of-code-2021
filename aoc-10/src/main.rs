use std::{collections::{VecDeque, HashMap}, fmt::{Display, write}};

use itertools::Itertools;


#[derive(Clone, Copy, Debug)]
enum Symbol {
    Paren,
    Square,
    Curly,
    Arrow,
}

impl Symbol {
    fn get_reciprocal(&self) -> char {
        match self {
            Symbol::Paren => ')',
            Symbol::Square => ']',
            Symbol::Curly => '}',
            Symbol::Arrow => '>',
        }
    }

    fn is_reciprocal(&self, test: &char) -> bool {
        self.get_reciprocal() == *test
    }
}

impl TryFrom<char> for Symbol {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Symbol::Paren),
            '[' => Ok(Symbol::Square),
            '{' => Ok(Symbol::Curly),
            '<' => Ok(Symbol::Arrow),
            _ => Err("Invalid char")
        }
    }
}

fn parse_line(input: &str) -> Result<VecDeque<Symbol>, char> {
    let mut symbols: VecDeque<Symbol> = VecDeque::new();

    for char in input.chars() {
        if let Ok(symbol) = Symbol::try_from(char) {
            symbols.push_front(symbol);
        } else if let Some(symbol) = symbols.get(0) {
            if symbol.is_reciprocal(&char) { 
                symbols.pop_front(); 
            } else {
                return Err(char);
            }
        } else {
            return Err(char);
        }
    }

    Ok(symbols)
}

fn problem_1(input: &str) -> u32 {
    let scores = input.lines()
        .map(parse_line)
        .filter_map(Result::err)
        .fold(HashMap::<char, u32>::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    let paren = scores.get(&')').unwrap_or(&0) * 3;
    let square = scores.get(&']').unwrap_or(&0) * 57;
    let curly = scores.get(&'}').unwrap_or(&0) * 1197;
    let arrow = scores.get(&'>').unwrap_or(&0) * 25137;

    paren + square + curly + arrow
}

fn problem_2(input: &str) -> u64 {
    let sorted_scores: Vec<u64> = input.lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .map(|v| v.iter().fold(0, |acc: u64, s| {
            acc * 5 + match s {
                Symbol::Paren => 1,
                Symbol::Square => 2,
                Symbol::Curly => 3,
                Symbol::Arrow => 4,
            }
        }))
        .sorted()
        .collect();

    let middle_index = (sorted_scores.len() as f32 / 2.0).floor() as usize;

    sorted_scores[middle_index]
}

fn main() {
    let input = include_str!("data.txt");

    // let out = problem_1(input);
    let out = problem_2(input);

    println!("{}", out);
}

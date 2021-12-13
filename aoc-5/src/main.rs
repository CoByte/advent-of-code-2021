use std::{fmt::Display, collections::HashMap, iter};

use nom::{IResult, character::complete::{digit1, line_ending}, bytes::complete::tag, sequence::separated_pair, multi::separated_list1};
use num::range_step_inclusive;


#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

struct Line {
    a: Point,
    b: Point,
}

fn step_between(a: u32, b: u32) -> Box<dyn Iterator<Item = u32>> {
    let (i_a, i_b) = (a as i32, b as i32);
    let step = (i_b - i_a).signum();    

    if a == b {
        Box::new(iter::repeat(a))
    } else {
        Box::new(range_step_inclusive(i_a, i_b, step)
            .map(|n| n as u32))
    }
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }

    fn iter_intersecting(&self) -> impl Iterator<Item = Point> {
        Iterator::zip(
            step_between(self.a.x, self.b.x),
            step_between(self.a.y, self.b.y)
        )
            .map(|(x,y)| Point::new(x, y))
    }
}


impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.a, self.b)
    }
}


fn get_overlap_count<'a, I>(lines: I) -> u32
where
    I: Iterator<Item = &'a Line>
{
    let mut overlaps: HashMap<Point, u32> = HashMap::new();

    for line in lines {
        for point in line.iter_intersecting() {
            *overlaps.entry(point).or_insert(0) += 1
        }
    }

    overlaps.values()
        .filter(|&&overlap_num| overlap_num >= 2)
        .count() as u32
}

fn problem_1(lines: &[Line]) -> u32 {
    let lines = lines.iter()
        .filter(|&l| l.a.x == l.b.x || l.a.y == l.b.y);
    get_overlap_count(lines)
}

fn problem_2(lines: &[Line]) -> u32 { 
    get_overlap_count(lines.iter()) 
}


fn parse_point(input: &str) -> IResult<&str, Point> {
    let (output, (x, y)) = separated_pair(digit1, tag(","), digit1)(input)?;
    Ok((output, Point::new(
        x.parse().unwrap(), 
        y.parse().unwrap()
    )))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (output, (a, b)) = separated_pair(parse_point, tag(" -> "), parse_point)(input)?;
    Ok((output, Line::new(a, b)))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(line_ending, parse_line)(input)
}


fn main() {
    let input = include_str!("data.txt");

    let (_, lines) = parse_lines(input).unwrap();

    // let out = problem_1(&lines);
    let out = problem_2(&lines);

    println!("{}", out);
}

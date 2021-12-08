use nom::{IResult, multi::{separated_list0, separated_list1}, character::complete::{alpha1, char, line_ending}, sequence::separated_pair, bytes::complete::tag};

type RawDisplay<'a> = (Digits<'a>, Vec<&'a str>);

struct Digits<'a>(Vec<&'a str>);

impl<'a> Digits<'a> {
    fn find_by_size(&mut self, size: u32) -> String {
        let index = self.0.iter()
            .position(|n| n.len() == size as usize)
            .expect("Invalid size");
        self.0.remove(index).into()
    }

    fn find_by_subset(&mut self, subset: &str) -> String {
        let index = self.0.iter()
            .position(|n| subset.chars().all(|m| n.contains(m)))
            .expect("Invalid subset");
        self.0.remove(index).into()
    }

    fn find_by_superset(&mut self, superset: &str) -> String {
        let index = self.0.iter()
            .position(|n| n.chars().all(|m| superset.contains(m)))
            .expect("Invalid superset");
        self.0.remove(index).into()
    }

    fn find_by_size_and_subset(&mut self, size: u32, subset: &str) -> String {
        let index = self.0.iter()
            .position(|n| subset.chars().all(|m| n.contains(m)) && n.len() == size as usize)
            .expect("Invalid size or subset");
        self.0.remove(index).into()
    }
}


fn problem_1(input: Vec<RawDisplay>) -> u32 {
    input.iter()
        .fold(0, |acc, (_, out)| {
            acc + out.iter()
                .filter(|n| matches!(n.len(), 2 | 3 | 4 | 7))
                .count() as u32
        })
}


fn get_display_code(digits: &mut Digits) -> Vec<String> {
    let mut mapping = vec![String::from("");10];
    mapping[1] = digits.find_by_size(2);
    mapping[4] = digits.find_by_size(4);
    mapping[7] = digits.find_by_size(3);
    mapping[8] = digits.find_by_size(7);
    mapping[9] = digits.find_by_subset(&mapping[4]);
    mapping[0] = digits.find_by_size_and_subset(6, &mapping[7]);
    mapping[6] = digits.find_by_size(6);
    mapping[3] = digits.find_by_subset(&mapping[7]);
    mapping[5] = digits.find_by_superset(&mapping[9]);
    mapping[2] = digits.find_by_size(5);
    mapping
}

fn decode_display(mapping: &[String], display: &str) -> u32 {
    mapping.iter().position(|n| {
        n.chars().all(|m| display.contains(m)) && display.chars().all(|d| n.contains(d))
    }).unwrap() as u32
}

fn decode_line((digits, displays): &mut RawDisplay) -> u32 {
    let mapping = get_display_code(digits);
    displays.iter()
        .rev()
        .enumerate()
        .map(|(count, n)| (10_u32.pow(count as u32)) * decode_display(&mapping, n))
        .sum()
}

fn problem_2(mut input: Vec<RawDisplay>) -> u32 {
    input.iter_mut()
        .fold(0, |acc, raw_display| acc + decode_line(raw_display))
}


fn parse_words(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(char(' '), alpha1)(input)
}

fn parse_line(input: &str) -> IResult<&str, RawDisplay> {
    let (out, (digits, display)) = separated_pair(
        parse_words, 
        tag(" | "), 
        parse_words
    )(input)?;

    let digits = Digits(digits);
    Ok((out, (digits, display)))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<RawDisplay>> {
    separated_list1(line_ending, parse_line)(input)
}


fn main() {
    let input = include_str!("data.txt");
    let (_, parsed) = parse_lines(input).unwrap();

    // let out = problem_1(parsed);
    let out = problem_2(parsed);

    println!("{}", out);
}

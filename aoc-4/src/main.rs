use nom::{IResult, bytes::complete::tag, multi::separated_list1, character::{complete::{digit1, space1, line_ending, multispace0, space0}}};


const BOARD_WIDTH: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<u32>,
    chosen: Vec<u32>,
}

impl Board {
    fn new(numbers: Vec<u32>) -> Self {
        Board { numbers, chosen: Vec::new() }
    }

    fn score(&self) -> u32 {
        let unmarked_sum: u32 = self.numbers.iter()
            .filter(|n| !self.chosen.contains(n))
            .sum();

        self.chosen.last().unwrap() * unmarked_sum
    }

    fn add_chosen(&mut self, test_num: u32) {
        if self.numbers.contains(&test_num) { self.chosen.push(test_num) }
    }

    fn is_cleared(&self) -> bool {
        (0..BOARD_WIDTH)
            .any(|n| self.check_row(n) || self.check_column(n))
    }

    fn check_row(&self, row: usize) -> bool {
        self.numbers[row * BOARD_WIDTH..(row + 1) * BOARD_WIDTH]
            .iter()
            .all(|n| self.chosen.contains(n))
    }

    fn check_column(&self, column: usize) -> bool {
        let mut pos = column;

        while let Some(item) = self.numbers.get(pos) {
            if !self.chosen.contains(item) { return false }
            pos += BOARD_WIDTH;
        }
        
        true
    }
}


fn get_draws(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, working) = separated_list1(tag(","), digit1)(input)?;
    let working = working.iter()
        .map(|&n| n.parse().unwrap())
        .collect();

    Ok((input, working))
}

fn drop_break(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

fn get_boards(input: &str) -> IResult<&str, Vec<Board>> {
    separated_list1(multispace0, get_board)(input)
}

fn get_board(input: &str) -> IResult<&str, Board> {
    let (input, working) = separated_list1(line_ending, get_board_line)(input)?;
    let working = working.iter()
        .flatten()
        .cloned()
        .collect();

    let board = Board::new(working);

    Ok((input, board))
}

fn get_board_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = space0(input)?;
    let (input, working) = separated_list1(space1, digit1)(input)?;
    let working = working.iter()
        .map(|&n| n.parse().unwrap())
        .collect();

    Ok((input, working))
}


fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let (input, draws) = get_draws(input).unwrap();
    let (input, _) = drop_break(input).unwrap();
    let (_, boards) = get_boards(input).unwrap();

    (draws, boards)
}


fn find_winning_board(draws: Vec<u32>, mut boards: Vec<Board>) -> Board {
    for draw in draws {
        boards.iter_mut().for_each(|b| b.add_chosen(draw));
        if let Some(winner) = boards.iter().find(|&b| b.is_cleared()) {
            return winner.clone()
        }
    }
    
    panic!("no winning board")
}

fn find_losingest_board(draws: Vec<u32>, mut boards: Vec<Board>) -> Board {
    for draw in draws {
        boards.iter_mut().for_each(|b| b.add_chosen(draw));
        if boards.len() == 1 && boards[0].is_cleared() {
            return boards.remove(0);
        }
        boards.retain(|b| !b.is_cleared())
    }

    panic!("something has gone wrong idk")
}

fn problem_1(draws: Vec<u32>, boards: Vec<Board>) -> u32 {    
    find_winning_board(draws, boards).score()
}

fn problem_2(draws: Vec<u32>, boards: Vec<Board>) -> u32 {
    find_losingest_board(draws, boards).score()
}

fn main() {
    let input = include_str!("data.txt");

    let (draws, boards) = parse_input(input);

    // let out = problem_1(draws, boards);
    let out = problem_2(draws, boards);

    println!("{}", out);
}

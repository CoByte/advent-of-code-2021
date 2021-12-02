use itertools::Itertools;


#[derive(Debug)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn deserialize(input: &str) -> Direction {
    let words = input.split(' ');
    let (direction, distance) = words.collect_tuple().unwrap();
    let distance = distance.parse().unwrap();
    match direction {
        "forward" => Direction::Forward(distance),
        "down" => Direction::Down(distance),
        "up" => Direction::Up(distance),
        _ => panic!("direction is invalid"),
    }
}

#[derive(Default, Debug)]
struct Position {
    x: u32,
    y: u32,
    aim: u32,
}

fn problem_1(input: &str) -> u32 {
    let final_position = input.lines()
        .map(deserialize)
        .fold(Position::default(), |mut acc, x| {
            match x {
                Direction::Forward(distance) => acc.x += distance,
                Direction::Down(distance) => acc.y += distance,
                Direction::Up(distance) => acc.y -= distance,
            };
            acc
        });

    final_position.x * final_position.y
}

fn problem_2(input: &str) -> u32 {
    let final_position = input.lines()
        .map(deserialize)
        .fold(Position::default(), |mut acc, x| {
            match x {
                Direction::Forward(distance) => {
                    acc.x += distance;
                    acc.y += acc.aim * distance;
                },
                Direction::Down(distance) => acc.aim += distance,
                Direction::Up(distance) => acc.aim -= distance,
            };
            acc
        });

    final_position.x * final_position.y
}



fn main() {
    let input = include_str!("data.txt");

    // let out = problem_1(input);
    let out = problem_2(input);

    println!("{}", out);
}

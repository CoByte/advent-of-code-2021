use itertools::Itertools;

fn problem_1(input: &str) -> usize {
    input.lines()
        .map(|n| n.parse::<u32>().unwrap())
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| b > a)
        .count()
}


fn problem_2(input: &str) -> usize {
    input.lines()
        .map(|n| n.parse::<u32>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| b > a)
        .count()
}


fn main() {
    let input = include_str!("data.txt");

    // let out = problem_1(input);
    let out = problem_2(input);

    println!("{}", out);
}

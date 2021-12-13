
fn problem_1(input: Vec<i32>) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    (min..max).fold(i32::MAX,|acc, n| {
        i32::min(acc, input.iter()
            .map(|v| (n - v).abs())
            .sum())
    })
}

fn problem_2(input: Vec<i32>) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    (min..max).fold(i32::MAX,|acc, n| {
        i32::min(acc, input.iter()
            .map(|v| (n - v).abs())
            .map(|v| v * (v + 1) / 2)
            .sum())
    })
}

fn main() {
    let input = include_str!("data.txt");
    let input = input.split(',')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>,_>>()
        .unwrap();

    // let out = problem_1(input);
    let out = problem_2(input);

    println!("{}", out);
}

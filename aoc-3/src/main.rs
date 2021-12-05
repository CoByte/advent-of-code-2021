

fn parse_input(input: &str) -> Vec<u32> {
    input.lines()
        .map(|n| u32::from_str_radix(n, 2))
        .map(Result::unwrap)
        .collect()
}

fn count_ones(words: &[u32], size: usize) -> Vec<u32> {
    words.iter()
        .fold(
            vec![0; size],
            |mut acc, &word| {
                for (index, current) in acc.iter_mut().enumerate() {
                    *current += ((word as usize) >> (size - index - 1) & 1) as u32
                }
                acc
            }
        )
}

fn get_rounded_distribution(ones: &[u32], total: u32) -> Vec<u32> {
    ones.iter()
        .map(|&n| total as i32 - n as i32 * 2)
        .map(|n| if n <= 0 { 1 } else { 0 })
        .collect()
}

fn bit_array_to_int(arr: &[u32]) -> u32 {
    arr.iter()
        .fold(0, |acc, n| acc << 1 | n & 1)
}

fn problem_1(input: &str) -> u32 {
    let words = parse_input(input);
    let word_size = input.lines().next().unwrap().len();
    let word_num = words.len();

    let ones = count_ones(&words, word_size);
    let distr = get_rounded_distribution(&ones, word_num as u32);

    let gamma = bit_array_to_int(&distr);
    let epsilon = !gamma & (2_u32.pow(word_size as u32) - 1);

    println!("gamma: {}, epsilon: {}", gamma, epsilon);

    gamma * epsilon
}

fn most_common_bit(numbers: &[u32], place: u32) -> u32 {
    let (total_zeros, total_ones) = numbers.iter().fold(
        (0, 0), 
        |(zeros, ones), n| match n >> place & 1 {
            0 => (zeros + 1, ones),
            1 => (zeros, ones + 1),
            _ => panic!("something has gone horribly wrong"),
        });

    if total_ones >= total_zeros { 1 } else { 0 }
}

fn filter_nums(mut numbers: Vec<u32>, mut place: u32, inverted: bool) -> u32 {
    while numbers.len() > 1 {
        let mut valid_bit = most_common_bit(&numbers, place as u32);
        if inverted { valid_bit = !valid_bit & 1 }
        numbers = numbers.iter()
            .filter(|&&n| n >> place & 1 == valid_bit)
            .cloned()
            .collect();

        if place == 0 { break }

        place -= 1;
    }

    match numbers.len() {
        0 => panic!("no numbers somehow"),
        1 => numbers[0],
        _ => panic!("too many numbers!")
    }
}

fn problem_2(input: &str) -> u32 {
    let numbers = parse_input(input);
    let place = input.lines().next().unwrap().len() - 1;

    let oxy_rating = filter_nums(numbers.clone(), place as u32, false);
    let co2_rating = filter_nums(numbers, place as u32, true);

    println!("Oxygen Rating: {}, CO2 Rating: {}", oxy_rating, co2_rating);

    oxy_rating * co2_rating
}

fn main() {
    let input = include_str!("data.txt");

    // let out = problem_1(input);
    let out = problem_2(input);

    println!("{}", out);
}

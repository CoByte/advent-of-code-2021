

#[derive(Debug)]
struct Fishes(Vec<u64>);

impl Fishes {
    fn next(&mut self) {
        let new_fish = self.0.remove(0);
        self.0[6] += new_fish;
        self.0.push(new_fish);
    }

    fn count(&self) -> u64 {
        self.0.iter().sum()
    }
}

impl FromIterator<u8> for Fishes {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut fishes = vec![0_u64;9];
        for fish in iter {
            *fishes.get_mut(fish as usize).unwrap() += 1;
        }
        Fishes(fishes)
    }
}   

fn get_fish(input: &str) -> Fishes {
    input.split(',')
        .into_iter()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn problem_1(mut fishes: Fishes) -> u64 {
    for _ in 0..80 {
        fishes.next();
    }

    fishes.count()
}

fn problem_2(mut fishes: Fishes) -> u64 {
    for _ in 0..256 {
        fishes.next()
    }

    fishes.count()
}

fn main() {
    let input = include_str!("data.txt");
    let input = get_fish(input);

    // let out = problem_1(input);
    let out = problem_2(input);

    println!("{}", out);
}

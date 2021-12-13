use std::fmt::Display;

use itertools::Itertools;


#[derive(PartialEq, Eq, Clone, Copy)]
enum OctopusState {
    Building(u8),
    Flashing,
}

impl Display for OctopusState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            &Self::Building(n) => n.to_string(),
            &Self::Flashing => "#".to_string(),
        })
    }
}
struct Octopus(OctopusState);

impl Display for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Octopus {
    fn incr(&mut self) -> usize {
        match self.0 {
            OctopusState::Building(n) if n < 9 => {
                self.0 = OctopusState::Building(n + 1);
                0
            },
            OctopusState::Building(_) => {
                self.0 = OctopusState::Flashing;
                1
            }
            _ => 0
        }
    }

    fn reset(&mut self) {
        self.0 = if self.0 == OctopusState::Flashing {
            OctopusState::Building(0)
        } else { 
            self.0
        }
    }
}

impl From<u8> for Octopus {
    fn from(input: u8) -> Self {
        if input < 10 { Octopus(OctopusState::Building(input)) }
        else { Octopus(OctopusState::Flashing) }
    }
}

struct FlatGrid<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> FlatGrid<T> {
    fn new(grid: Vec<T>, width: usize, height: usize) -> Self {
        FlatGrid { grid, width, height }
    }

    fn index(&self, x: isize, y: isize) -> Option<usize> {
        if (0..self.width as isize).contains(&x) 
            && (0..self.height as isize).contains(&y) {
            Some((x + y * self.width as isize) as _)
        } else {
            None
        }
    }

    fn coords(&self, index: usize) -> (isize, isize) {
        ((index % self.width) as _, (index / self.width) as _)
    }

    fn get_neighbor_positions(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        let (x, y) = self.coords(index);
        (-1..=1)
            .map(|y| (-1..=1).map(move |x| (x, y)))
            .flatten()
            .filter(|&(x, y)| x != 0 || y != 0)
            .map(move |(offset_x, offset_y)| (offset_x + x, offset_y + y))
            .filter_map(|(x, y)| self.index(x, y))
    }
}

impl<T> Display for FlatGrid<T>
where
    T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        let out: String = self.grid.iter()
            .map(|n| n.to_string())
            .intersperse_with(|| {
                let out = if i + 1 == self.width { "\n"} else { "" };
                i = (i + 1) % self.width;
                out.into()
            })
            .collect();
            
        write!(f, "{}", out)
    }
}


impl FlatGrid<Octopus> {
    fn step(&mut self) -> usize {
        let mut total_flashes = self.grid.iter_mut()
            .map(|o| o.incr())
            .sum();
        let mut flashing: Vec<usize> = self.grid.iter()
            .enumerate()
            .filter(|(_, o)| o.0 == OctopusState::Flashing)
            .map(|(i, _)| i)
            .collect();

        while !flashing.is_empty() {
            let neighbors: Vec<usize> = flashing.iter()
                .map(|n| self.get_neighbor_positions(*n))
                .flatten()
                .filter(|&n| self.grid.get(n).unwrap().0 != OctopusState::Flashing)
                .collect();
            
            total_flashes += neighbors.iter()
                .map(|n| self.grid.get_mut(*n).unwrap().incr())
                .sum::<usize>();

            flashing = neighbors.iter()
                .filter(|&&n| self.grid.get(n).unwrap().0 == OctopusState::Flashing)
                .cloned()
                .unique()
                .collect();
        }

        for o in self.grid.iter_mut() {
            o.reset();
        }

        total_flashes
    }
}


fn parse_input(input: &str) -> FlatGrid<Octopus> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid: Vec<Octopus> = input.lines()
        .map(str::chars)
        .flatten()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .map(Octopus::from)
        .collect();

    FlatGrid::new(grid, width, height)
}

fn problem_1(mut input: FlatGrid<Octopus>) -> usize {
    (0..100).map(|_| input.step()).sum()
}

fn problem_2(mut input: FlatGrid<Octopus>) -> usize {
    let flash_num = input.grid.len();

    for c in 1.. {
        if input.step() == flash_num { return c }
    }

    unreachable!()
}

fn main() {
    let input = include_str!("data.txt");
    let input = parse_input(input);

    // let out = problem_1(input);
    let out = problem_2(input);

    println!("{}", out);
}
use std::{slice::SliceIndex, ops::RangeBounds};

use itertools::Itertools;

struct FlatGrid<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> FlatGrid<T> {
    const NEIGHBORS: &'static [(isize, isize)] = &[
        ( 1, 0),
        (-1, 0),
        ( 0, 1),
        ( 0,-1)
    ];

    fn new(grid: Vec<T>, width: usize, height: usize) -> Self {
        FlatGrid { grid, width, height }
    }

    fn index(&self, x: isize, y: isize) -> Option<usize> {
        if (0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y) {
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
        FlatGrid::<T>::NEIGHBORS.iter()
            .map(move |(offset_x, offset_y)| (offset_x + x, offset_y + y))
            .filter_map(|(x, y)| self.index(x, y))
    }

    fn get_neighbors(&self, index: usize) -> impl Iterator<Item = &T> + '_ {
        self.get_neighbor_positions(index)
            .filter_map(|i| self.grid.get(i))
    }
}

fn parse(input: &str) -> FlatGrid<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid: Vec<u32> = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .flatten()
        .collect();

    FlatGrid::new(grid, width, height)
}

fn problem_1(input: FlatGrid<u32>) -> u32 {
    input.grid.iter()
        .enumerate()
        .filter(|(pos, height)| input.get_neighbors(*pos).all(|n| n > height))
        .map(|(_, height)| height + 1)
        .sum()
}

trait RemoveBy<T> {
    fn remove_eq(&mut self, other: &T)
    where
        T: PartialEq;
}

impl<T> RemoveBy<T> for Vec<T> {
    fn remove_eq(&mut self, other: &T)
    where
        T: PartialEq 
    {
        let index = self.iter().position(|x| *x == *other);
        if let Some(index) = index { self.remove(index); }
    }
}


fn problem_2(input: FlatGrid<u32>) -> u32 {
    let mut positions: Vec<usize> = (0..input.grid.len()).collect();
    let mut basins: Vec<usize> = vec![];

    while !positions.is_empty() {
        let test_pos = *positions.get(0).unwrap();

        if *input.grid.get(test_pos).unwrap() == 9 {
            positions.remove(0);
            continue;
        }

        let mut removals = vec![test_pos];
        let mut debug_thing = removals.clone();

        let mut working_size = 0;
        while !removals.is_empty() {
            working_size += removals.len();

            for pos in removals.iter() {
                positions.remove_eq(pos);
            }

            removals = removals.iter()
                .map(|r| input.get_neighbor_positions(*r))
                .flatten()
                .filter(|p| positions.contains(p))
                .filter(|p| *input.grid.get(*p).unwrap() != 9)
                .unique()
                .collect();
            debug_thing.extend(removals.clone());
        }
        basins.push(working_size);
    }

    basins.sort_by(|a, b| b.cmp(a));

    (basins[0] * basins[1] * basins[2]) as _
}


fn main() {
    let input = include_str!("data.txt");
    let input = parse(input);

    // let out = problem_1(input);
    let out = problem_2(input);

    println!("{}", out);
}

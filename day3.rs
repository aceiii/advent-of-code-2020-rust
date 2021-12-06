#![allow(unused_imports, dead_code)]

use std::io::{self, BufRead, stdin};

type Error = String;

fn stdin_lines() -> Vec<String> {
    io::stdin().lock().lines().collect::<Result<Vec<String>,_>>().unwrap()
}

#[derive(Debug)]
struct TreeMap {
    lines: Vec<String>,
    height: usize,
}

#[derive(Debug, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Slope(usize, usize);

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }

    fn add(&self, slope: &Slope) -> Self {
        Self { x: self.x + slope.0, y: self.y + slope.1 }
    }
}

fn is_tree(t: char) -> bool {
    match t {
        '#' => true,
        _ => false,
    }
}

impl TreeMap {
    fn new(lines: &Vec<String>) -> Result<Self, Error> {
        Ok(TreeMap {
            lines: lines.clone(),
            height: lines.len(),
        })
    }

    fn get(&self, pos: &Pos) -> Option<char> {
        let line = self.lines.get(pos.y)?;
        let width = line.len();

        line.chars().nth(pos.x % width)
    }

    fn count_trees(&self, slope: &Slope) -> usize {
        let mut pos = Pos::new(0, 0);
        let mut trees = 0;

        while pos.y < self.height {
            trees += match self.get(&pos) {
                Some(t) => if is_tree(t) { 1 } else { 0 },
                _ => 0,
            };
            pos = pos.add(slope);
        }

        trees
    }
}

macro_rules! slopes {
    () => (
        Vec::<Slope>::new()
    );
    ($(($x:expr,$y:expr)),+) => {
        {
            let mut vec: Vec<Slope> = vec![];
            $(vec.push(Slope($x,$y));)+
            vec
        }
    };
}

fn part1(lines: &Vec<String>) -> Result<usize, Error> {
    let tree_map = TreeMap::new(lines)?;
    let slope = Slope(3, 1);
    Ok(tree_map.count_trees(&slope))
}

fn part2(lines: &Vec<String>) -> Result<usize, Error> {
    let tree_map = TreeMap::new(lines)?;
    let slopes = slopes![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    Ok(slopes.iter().fold(1, |accum, slope| tree_map.count_trees(&slope) * accum))
}

fn main() -> Result<(), Error> {
    let lines = stdin_lines();
    println!("Part1: {}", part1(&lines)?);
    println!("Part2: {}", part2(&lines)?);
    Ok(())
}


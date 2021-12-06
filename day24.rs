#![allow(unused_imports, dead_code)]

use std::io::{self, BufRead, stdin};
use std::cmp;
use std::fmt;
use std::collections::{self, LinkedList, HashSet, HashMap};
use std::convert::{From, TryInto};
use std::str::FromStr;

fn stdin_lines() -> Vec<String> {
    io::stdin().lock().lines().collect::<Result<Vec<String>,_>>().unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&mut self, pos: &Pos) {
        self.x += pos.x;
        self.y += pos.y;
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

impl Eq for Pos {
}

#[derive(Debug)]
struct Tile {
    inner: Pos,
    orig: String,
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tile {
            inner: match s {
                "e" => Ok(Pos::new(2, 0)),
                "w" => Ok(Pos::new(-2, 0)),
                "se" => Ok(Pos::new(1, 1)),
                "sw" => Ok(Pos::new(-1, 1)),
                "ne" => Ok(Pos::new(1, -1)),
                "nw" => Ok(Pos::new(-1,-1)),
                _ => Err(format!("Invalid tile: {}", s)),
            }?,
            orig: s.to_string(),
        })
    }
}

impl From<&Tile> for Pos {
    fn from(tile: &Tile) -> Pos {
        tile.inner
    }
}

fn flip_initial_tiles(lines: &Vec<String>) -> Result<HashSet<Pos>, String> {
    let mut black_tiles: HashSet<Pos> = HashSet::new();

    for line in lines.iter() {
        let mut line: &str = &line[..];
        let mut pos = Pos::default();
        loop {
            if line.len() == 0 {
                break;
            }

            let tile = match line.len() {
                2..=usize::MAX => {
                    match (&line[..2]).parse::<Tile>() {
                        Ok(tile) => tile,
                        Err(_) => (&line[..1]).parse::<Tile>()?,
                    }
                },
                _ => (&line[..1]).parse::<Tile>()?
            };

            pos.add(&Pos::from(&tile));

            line = &line[tile.orig.len()..];
        };

        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    }

    Ok(black_tiles)
}

fn get_neighbours(tile: &Pos) -> [Pos; 6] {
    let mut neighbours = [Pos::default(); 6];
    let dirs = &["e", "w", "se", "sw", "ne", "nw"];
    for (i, dir) in dirs.iter().enumerate() {
        let neighbour: Tile = dir.parse().unwrap();

        let mut pos = tile.clone();
        pos.add(&Pos::from(&neighbour));

        neighbours[i] = pos;
    }
    neighbours
}

fn flip_tiles(black_tiles: &mut HashSet<Pos>) -> Result<(), String> {
    let mut black_tiles_to_flip: Vec<Pos> = vec![];
    let mut white_tiles: HashMap<Pos, u8> = HashMap::new();

    for tile in black_tiles.iter() {
        let mut black_neighbour_count = 0;
        let neighbours = get_neighbours(&tile);

        for n in neighbours.iter() {
            if black_tiles.contains(n) {
                black_neighbour_count += 1;
            } else {
                let count = white_tiles.entry(*n).or_insert(0);
                *count += 1;
            }
        }

        if black_neighbour_count == 0 || black_neighbour_count > 2 {
            black_tiles_to_flip.push(*tile);
        }

    }

    for tile in black_tiles_to_flip.iter() {
        black_tiles.remove(tile);
    }

    for (tile, count) in white_tiles.iter() {
        if *count == 2 {
            black_tiles.insert(*tile);
        }
    }

    Ok(())
}

fn part1(lines: &Vec<String>) -> Result<usize, String> {
    let black_tiles = flip_initial_tiles(lines)?;
    Ok(black_tiles.len())
}

fn part2(lines: &Vec<String>) -> Result<usize, String> {
    let mut black_tiles = flip_initial_tiles(lines)?;
    let target = 100;

    for _ in 0..target {
        flip_tiles(&mut black_tiles)?;
    }

    Ok(black_tiles.len())
}

fn main() -> Result<(), String> {
    let lines = stdin_lines();
    println!("Part1: {}", part1(&lines)?);
    println!("Part2: {}", part2(&lines)?);
    Ok(())
}


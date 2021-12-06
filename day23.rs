#![allow(unused_imports, dead_code)]

use std::io::{self, BufRead, stdin};
use std::cmp;
use std::fmt;
use std::collections::{self, LinkedList, HashSet};
use std::convert::TryInto;

fn stdin_lines() -> Vec<String> {
    io::stdin().lock().lines().collect::<Result<Vec<String>,_>>().unwrap()
}

#[derive(Debug, Clone)]
struct Cup {
    label: u8,
    next: Option<Box<Cup>>,
}

impl Cup {
    fn new(label: u8) -> Self {
        Self {
            label,
            next: None,
        }
    }

    fn insert(&mut self, label: u8) -> Option<&mut Box<Cup>> {
        self.next = Some(Box::new(Self {
            label,
            next: self.next.clone(),
        }));
        self.next.as_mut()
    }
}

impl fmt::Display for Cup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self.next {
            Some(_) => write!(f, "{} {}", self.label, self.next.as_ref().unwrap()),
            _ => write!(f, "{}", self.label),
        }
    }
}

#[derive(Debug)]
struct Cups {
    head: Box<Cup>,
    max: u8,
}

impl Cups {
    fn new(labels: &[u8]) -> Result<Self, String> {
        let mut max = *labels.get(0).ok_or_else(|| "Not enough labels")?;
        let mut min = max;

        let mut head = Box::new(Cup::new(max));
        let mut current = &mut head;

        for label in &labels[1..] {
            let label = *label;
            max = cmp::max(max, label);
            min = cmp::min(min, label);
            current = current
                .insert(label)
                .ok_or_else(|| format!("Failed to insert: {}", label))?;
        }

        Ok(Cups {
            head,
            max,
        })
    }

    fn shuffle(&mut self) -> Result<(), String> {
        /*
        let mut current_head = &self.head;
        let mut skip_cups = &current_head.next;
        let mut new_next = skip_cups;
        for _ in 0..2 {
            new_next = match new_next {
                Some(next) => &Some(next),
                _ => &None,
            };
        }
        self.head = current_head;
        self.head.next = new_next;
        */
        Ok(())
    }
}

impl fmt::Display for Cups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "cups: {}", self.head)?;
        /*
        match self.head.next {
            Some(_) => write!(f, "{} {}", self.head.label, self.head.next.unwrap()),
            _ => write!(f, "{}", self.head.label),
        };
        */
        Ok(())
    }
}

#[derive(Debug)]
struct CrabCups {
    list: LinkedList<u8>,
    max: u64,
}

impl CrabCups {
    fn new(labels: &[u8]) -> Result<Self, String> {
        let mut cups = Self {
            list: LinkedList::new(),
            max: labels.len().try_into().map_err(|_| "Too many labels")?,
        };

        cups.list.extend(labels);

        Ok(cups)
    }

    fn shuffle(&mut self) {
        let (target, mut head, mut picked_up) = {
            let mut prev_head = self.list.split_off(0);
            let mut picked_up = prev_head.split_off(1);
            let mut new_head = picked_up.split_off(3);
            let target = *prev_head.front().unwrap();
            new_head.append(&mut prev_head);

            (target, new_head, picked_up)
        };

        let dest = {
            let mut dest = match target {
                1 => self.max,
                val => val - 1,
            };

            let set = picked_up.iter().collect::<Vec<_>>();

            while set.contains(&&dest) {
                dest = match dest {
                    1 => self.max,
                    val => val - 1,
                };
            }

            dest
        };

        let mut index = 0;
        for (i, label) in head.iter().enumerate() {
            if dest == *label {
                index = i;
                break;
            }
        }

        self.list = {
            let mut split = head.split_off(index + 1);
            head.append(&mut picked_up);
            head.append(&mut split);
            head
        };
    }

    fn reorder(&mut self) {
        let mut index = 0;
        for (i, val) in self.list.iter().enumerate() {
            if *val == 1 {
                index = i;
                break;
            }
        }

        self.list = {
            let mut new_head = self.list.split_off(index + 1);
            new_head.append(&mut self.list);
            new_head
        };
    }

    fn result(&self) -> String {
        let mut vec = self.list
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>();
        vec.pop();

        return vec.join("");
    }
}

impl fmt::Display for CrabCups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "cups:")?;
        for label in self.list.iter() {
            write!(f, " {}", label)?;
        }
        Ok(())
    }
}

fn part1(lines: &Vec<String>) -> Result<String, String> {
    let first = lines.first().ok_or_else(|| "Not enough lines")?;
    let loops = 100;

    let cups = first
        .chars()
        .map(|c| c.to_string().parse::<u8>())
        .collect::<Result<Vec<_>,_>>()
        .map_err(|_| format!("Invalid line: {}", first))?;

    let mut cups = CrabCups::new(&cups[..])?;

    for _ in 0..loops {
        cups.shuffle();
    }

    cups.reorder();

    Ok(cups.result())
}

fn part2(_lines: &Vec<String>) -> Result<usize, String> {
    Ok(0)
}

fn main() -> Result<(), String> {
    let lines = stdin_lines();

    println!("Part1: {}", part1(&lines)?);
    println!("Part2: {}", part2(&lines)?);

    Ok(())
}


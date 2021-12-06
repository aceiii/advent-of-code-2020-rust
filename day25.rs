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

fn transform_subject(subject_number: usize, loop_size: usize) -> usize {
    let mut target = 1;

    for _ in 0..loop_size {
        target = target * subject_number % 20201227;
    }

    target
}

fn discover_loop_size(key: usize, subject_number: usize) -> usize {
    let mut loop_size = 0;
    let mut target = 1;
    loop {
        loop_size += 1;
        target = target * subject_number % 20201227;

        if target == key {
            break loop_size;
        }
    }
}

fn part1(lines: &Vec<String>) -> Result<usize, String> {
    let keys = &lines[..2];
    if keys.len() != 2 {
        return Err("Invalid input size".into());
    }

    let card_public_key = keys.get(0).unwrap()
        .parse::<usize>()
        .map_err(|_| "Invalid card public key")?;

    let door_public_key = keys.get(1).unwrap()
        .parse::<usize>()
        .map_err(|_| "Invalid door public key")?;

    let card_loop_size = discover_loop_size(card_public_key, 7);
    let door_loop_size = discover_loop_size(door_public_key, 7);

    let enc_key1 = transform_subject(card_public_key, door_loop_size);
    let enc_key2 = transform_subject(door_public_key, card_loop_size);

    assert!(enc_key1 == enc_key2, format!("encryption keys dont match: {} != {}", enc_key1, enc_key2));

    Ok(enc_key1)
}

fn main() -> Result<(), String> {
    let lines = stdin_lines();
    println!("Part1: {}", part1(&lines)?);
    Ok(())
}


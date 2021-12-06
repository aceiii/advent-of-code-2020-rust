#![allow(unused_imports, dead_code)]

use std::fs::File;
use std::io::{self, BufRead, stdin};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use std::env;


fn stdin_lines() -> Vec<String> {
    io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

/*
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
*/

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|x| x.unwrap()).collect())

    /*
    Ok(io::BufReader::new(file).lines().map(|x| {
        let Ok(line) = x;
        if line {
            return line;
        }
        return Err("Blah")
    }).collect())
    */
}

//fn parse_numbers(lines: &io::Lines<io::BufReader<File>>) -> Vec<i32> {
fn parse_numbers(lines: &Vec<String>) -> Vec<i32> {

    let numbers: Vec<i32> = lines.iter().map(|x| {
        x.parse::<i32>().unwrap()
    }).collect();
    return numbers;

    /*
    let mut numbers: Vec<i32> = Vec::new();
    for line in lines.iter() {
        let num = line.parse::<i32>().unwrap();
        numbers.push(num);
    }
    //println!("{:?}", numbers);
    numbers
    */
}

fn find_pairs(numbers: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut prev_numbers: HashSet<i32> = HashSet::new();
    for number in numbers {
        let diff = target - number;
        if prev_numbers.contains(&diff) {
            return (*number, diff);
        }
        prev_numbers.insert(*number);
    }
    panic!()
}

fn find_triples(numbers: &Vec<i32>, target: i32) -> (i32, i32, i32) {
    let mut pairs: HashMap<i32, (i32, i32)> = HashMap::new();
    /*
    for num1 in &numbers {
        println!("{:?}", num1);
    }
    */

    /*
    let x = numbers.iter().enumerate();
    println!("{:?}", x);
    */

    for (index1, num1) in numbers.iter().enumerate() {
        for num2 in numbers[index1..].iter() {
            //println!("{} x {}", num1, num2);
            pairs.insert(num1 + num2, (*num1, *num2));
        }
    }

    for number in numbers {
        let diff = target - number;
        if pairs.contains_key(&diff) {
            let (num1, num2) = pairs.get(&diff).unwrap();
            return (*number, *num1, *num2);
        }
    }
    panic!();
}

//fn part1(lines: &io::Lines<io::BufReader<File>>) {
fn part1(lines: &Vec<String>) {
    let numbers = parse_numbers(&lines);
    let pair = find_pairs(&numbers, 2020);
    let answer = pair.0 * pair.1;

    println!("Part1: {}", answer);
}


//fn part2(lines: &io::Lines<io::BufReader<File>>) {
fn part2(lines: &Vec<String>) {
    let numbers = parse_numbers(&lines);
    let triple = find_triples(&numbers, 2020);
    let answer = triple.0 * triple.1 * triple.2;

    println!("Part2: {}", answer);
}

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    for arg in args {
        println!("{}", arg);
    }
    */

    /*
    if let Ok(lines) = read_lines("day2.txt") {
        part1(&lines);
        part2(&lines);
    } else {
        println!("File not found!");
    }
    */

    let lines: Vec<String> = stdin_lines();
    part1(&lines);
    part2(&lines);
}


#![allow(unused_imports, dead_code)]

use std::io::{self, BufRead, Lines, StdinLock};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display};


#[derive(Debug)]
struct ParsedLine(i32, i32, char, String);

impl Display for ParsedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsedLine({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

/*
impl<T> Display for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[");
        for item in self.iter() {
            write!(f, "{}", item);
        }
        write!(f, "]");
        Ok(())
    }
}
*/

#[derive(Debug)]
struct ParseLineError(String);

impl Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseLineError({})", self.0)
    }
}

impl Error for ParseLineError {}


fn validate_line(line: &ParsedLine) -> bool {
    let ParsedLine(min, max, letter, password) = line;
    let count: i32 = password.chars().fold(0, |count, c| {
        count + if c == *letter { 1 } else { 0 }
    });

    count >= *min && count <= *max
}

fn validate_line2(line: &ParsedLine) -> bool {
    let ParsedLine(num1, num2, letter, password) = line;
    let password = password.chars().collect::<Vec<char>>();

    let index1 = usize::try_from(*num1);
    let index2 = usize::try_from(*num2);

    if let (Ok(index1), Ok(index2)) = (index1, index2) {
        let match1 = password[index1 - 1usize] == *letter;
        let match2 = password[index2 - 1usize] == *letter;
        return match1 != match2;
    }

    false
}

/*
fn invalid_line(line: &String) -> String {
    format!("Invalid line: '{}'", line)
}
*/

fn parse_line(line: &String) -> Result<ParsedLine, Box<dyn Error>> {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let err = || ParseLineError(line.to_string());
    let left = parts.get(0).ok_or_else(err)?;
    let password = parts.get(1).ok_or_else(err)?.to_string();

    let parts = left.split(" ").collect::<Vec<&str>>();
    let left = parts.get(0).ok_or_else(err)?;
    let c = parts.get(1).ok_or_else(err)?.parse::<char>().map_err(|_| err())?;

    let parts = left.split("-").collect::<Vec<&str>>();
    let num1 = parts.get(0).ok_or_else(err)?.parse::<i32>().map_err(|_| err())?;
    let num2 = parts.get(1).ok_or_else(err)?.parse::<i32>().map_err(|_| err())?;

    Ok(ParsedLine(num1, num2, c, password))
}

fn parse_lines(lines: &Vec<String>) -> Result<Vec<ParsedLine>, Box<dyn Error>> {
    lines.iter().map(parse_line).collect()
}

fn stdin_lines() -> Vec<String> {
    io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

fn part1(lines: &Vec<String>) -> Result<usize, Box<dyn Error>> {
    //let result = lines.iter()
        /*
        .map(|x| parse_line(x))
        .filter(|x| {
            //validate_line(&x.as_ref().unwrap()))
            match x {
                Some(x) => *x,
                _ => false,
            }
        })
        */
        /*
        .filter_map(|x| {
            //println!("filter_map: {:?}", x);
            None
        })
        */
    /*
        .map(|x| parse_line(x))
        .filter(|x| validate_line(x))
        .collect::<Result<Vec<ParsedLine>,Box<dyn Error>>>();
        */

    Ok(parse_lines(lines)?
       .iter()
       .filter(|x| validate_line(x))
       .count())

    /*
    println!("Part1: {}", match result {
        Ok(answer) => answer.len().to_string(),
        Err(e) => format!("{:?}", e),
    });
    */

}

fn part2(lines: &Vec<String>) -> Result<usize, Box<dyn Error>> {
    let result = lines.iter()
        .map(|x| parse_line(x))
        //.filter(|x| validate_line2(&x.as_ref().unwrap()))
        .filter(|x| validate_line2(x.as_ref().unwrap()))
        .collect::<Result<Vec<ParsedLine>,_>>();

    /*
    println!("Part2: {}", match result {
        Ok(answer) => answer.len().to_string(),
        Err(e) => e.to_string()
    });
    */

    Ok(result?.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin_lines();
    println!("Part1: {}", part1(&lines)?);
    println!("Part2: {}", part2(&lines)?);
    Ok(())
}


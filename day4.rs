#![allow(unused_imports, dead_code)]

use std::io::{self, BufRead, stdin};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

type Error = String;

fn stdin_lines() -> Vec<String> {
    io::stdin().lock().lines().collect::<Result<Vec<String>,_>>().unwrap()
}

#[derive(Debug, Copy, Clone)]
enum Unit {
    Cm,
    In,
}

impl FromStr for Unit {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Self::Cm),
            "in" => Ok(Self::In),
            _ => Err("Invalid unit"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum PassportFieldKey {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

#[derive(Debug, Clone)]
enum Color {
    Hex(u32),
    Named(String),
}


#[derive(Debug, Clone)]
enum PassportFieldValue {
    Num(i32),
    Measure(usize, Unit),
    Color(Color),
    IdNum(String),
}

#[derive(Debug, Clone)]
struct PassportField {
    key: PassportFieldKey,
    value: Option<PassportFieldValue>,
    str: String,
}

#[derive(Debug)]
struct PassportFieldError {
    key: String,
    value: String,
}

impl PassportField {
    fn parse_number(key: PassportFieldKey, s: &str) -> Self {
        Self {
            key,
            value: match s.parse::<i32>() {
                Ok(val) => Some(PassportFieldValue::Num(val)),
                _ => None
            },
            str: s.to_string(),
        }
    }

    fn parse_measure(key: PassportFieldKey, s: &str) -> Self {
        Self {
            key,
            value: {
                let val = s[..s.len()-2].parse::<usize>().ok();
                let unit = s[s.len()-2..].parse::<Unit>().ok();

                match (val, unit) {
                    (Some(v), Some(u)) => Some(PassportFieldValue::Measure(v, u)),
                    _ => None
                }
            },
            str: s.to_string(),
        }
    }

    fn parse_color(key: PassportFieldKey, s: &str) -> Self {
        Self {
            key,
            value: {
                if &s[0..1] == "#" {
                    match u32::from_str_radix(&s[1..], 16) {
                        Ok(hex) => Some(PassportFieldValue::Color(Color::Hex(hex))),
                        _ => None
                    }
                } else {
                    Some(PassportFieldValue::Color(Color::Named(s.to_string())))
                }
            },
            str: s.to_string(),
        }
    }

    fn parse_id(key: PassportFieldKey, s: &str) -> Self {
        Self {
            key,
            value: {
                match s.parse::<u32>() {
                    Ok(_) => Some(PassportFieldValue::IdNum(s.to_string())),
                    _ => None,
                }
            },
            str: s.to_string(),
        }
    }
}

impl TryFrom<(&str, &str)> for PassportField {
    type Error = PassportFieldError;

    fn try_from(field: (&str, &str)) -> Result<Self, Self::Error> {
        let err = || PassportFieldError {
            key: field.0.to_string(),
            value: field.1.to_string(),
        };

        let num = |key| Self::parse_number(key, field.1);
        let measure = |key| Self::parse_measure(key, field.1);
        let color = |key| Self::parse_color(key, field.1);
        let id = |key| Self::parse_id(key, field.1);

        Ok(match field.0 {
            "byr" => num(PassportFieldKey::Byr),
            "iyr" => num(PassportFieldKey::Iyr),
            "eyr" => num(PassportFieldKey::Eyr),
            "hgt" => measure(PassportFieldKey::Hgt),
            "hcl" => color(PassportFieldKey::Hcl),
            "ecl" => color(PassportFieldKey::Ecl),
            "pid" => id(PassportFieldKey::Pid),
            "cid" => id(PassportFieldKey::Cid),
            _ => Err(err())?
        })
    }
}


fn parse_passport_fields(line: &String) -> impl Iterator<Item = PassportField> + '_ {
    line.split(' ').filter_map(|x| {
        let mut field_split = x.split(':');
        let tuple = (field_split.next(), field_split.next());
        if let (Some(key), Some(value)) = tuple {
            return PassportField::try_from((key, value)).ok()
        }
        None
    })
}

#[derive(Debug)]
struct Passport {
    inner: HashMap<PassportFieldKey, PassportField>
}

impl Passport {
    fn new() -> Self {
        Passport {
            inner: std::collections::HashMap::new()
        }
    }

    fn from_lines(lines: Vec<&String>) -> Result<Self, Error> {
        let mut hashmap = std::collections::HashMap::new();

        for line in lines {
            for field in parse_passport_fields(line) {
                //hashmap.insert(field.0, field.1);
                if let Ok(passport_field) = PassportField::try_from(field) {
                    hashmap.insert(passport_field.key, passport_field);
                }
            }
        }

        Ok(Passport {
            inner: hashmap
        })
    }

    fn has_key(&self, key: PassportFieldKey) -> bool {
        self.inner.contains_key(&key)
    }

    fn fields(&self) -> impl Iterator<Item = (&PassportFieldKey, &PassportField)> + '_ {
        self.inner.iter()
    }
}

fn parse_passport_batch(lines: &Vec<String>) -> Result<Vec<Passport>, Error> {
    let mut passports = vec![];
    let mut batched_lines: Vec<&String> = Vec::new();

    for line in lines {
        if *line == "".to_string() && batched_lines.len() > 0 {
            passports.push(Passport::from_lines(batched_lines)?);
            batched_lines = Vec::new();
            continue;
        }
        batched_lines.push(&line);
    }

    if batched_lines.len() > 0 {
        passports.push(Passport::from_lines(batched_lines)?);
    }

    Ok(passports)
}

fn has_required_fields(passport: &Passport) -> bool {
    let required_fields = vec![
        PassportFieldKey::Byr,
        PassportFieldKey::Iyr,
        PassportFieldKey::Eyr,
        PassportFieldKey::Hgt,
        PassportFieldKey::Hcl,
        PassportFieldKey::Ecl,
        PassportFieldKey::Pid,
    ];

    for key in required_fields {
        if !passport.has_key(key) {
            return false;
        }
    }

    true
}

fn is_valid_byr(value: &Option<PassportFieldValue>) -> bool {
    if let Some(PassportFieldValue::Num(year)) = value {
        return *year >= 1920 && *year <= 2002;
    }
    false
}

fn is_valid_iyr(value: &Option<PassportFieldValue>) -> bool {
    if let Some(PassportFieldValue::Num(year)) = value {
        return *year >= 2010 && *year <= 2020;
    }
    false
}

fn is_valid_eyr(value: &Option<PassportFieldValue>) -> bool {
    if let Some(PassportFieldValue::Num(year)) = value {
        return *year >= 2020 && *year <= 2030;
    }
    false
}

fn is_valid_hgt(value: &Option<PassportFieldValue>) -> bool {
    if let Some(PassportFieldValue::Measure(height, unit)) = value {
        return match unit {
            Unit::Cm => *height >= 150 && *height <= 193,
            Unit::In => *height >= 59 && *height <= 76,
        }
    }
    false
}

fn is_valid_hcl(value: &Option<PassportFieldValue>) -> bool {
    if let Some(PassportFieldValue::Color(Color::Hex(_))) = value {
        return true;
    }
    false
}

fn is_valid_ecl(value: &Option<PassportFieldValue>) -> bool {
    if let Some(PassportFieldValue::Color(Color::Named(col))) = value {
        return match col.as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        }
    }
    false
}

fn is_valid_pid(value: &Option<PassportFieldValue>) -> bool {
    if let Some(PassportFieldValue::IdNum(id)) = value {
        return id.len() == 9;
    }
    false
}

fn is_valid_cid(_value: &Option<PassportFieldValue>) -> bool {
    true
}

fn is_valid_field(field: &PassportField) -> bool {
    match field.key {
        PassportFieldKey::Byr => is_valid_byr(&field.value),
        PassportFieldKey::Iyr => is_valid_iyr(&field.value),
        PassportFieldKey::Eyr => is_valid_eyr(&field.value),
        PassportFieldKey::Hgt => is_valid_hgt(&field.value),
        PassportFieldKey::Hcl => is_valid_hcl(&field.value),
        PassportFieldKey::Ecl => is_valid_ecl(&field.value),
        PassportFieldKey::Pid => is_valid_pid(&field.value),
        PassportFieldKey::Cid => is_valid_cid(&field.value),
    }
}

fn is_valid_passport(passport: &Passport) -> bool {
    if !has_required_fields(passport) {
        return false;
    }

    for (_, field) in passport.fields() {
        if !is_valid_field(field) {
            return false;
        }
    }

    true
}

fn part1(lines: &Vec<String>) -> Result<usize, Error> {
    let passports = parse_passport_batch(lines)?;
    Ok(passports.iter().filter(|x| has_required_fields(x)).count())
}

fn part2(lines: &Vec<String>) -> Result<usize, Error> {
    let passports = parse_passport_batch(lines)?;
    //Ok(passports.iter().filter(|x| is_valid_passport(x)).count())

    let mut count: usize = 0;
    for passport in passports {
        let valid = is_valid_passport(&passport);
        count += if valid { 1 } else { 0 };
        //println!("{} => {:#?}: {}\n\n", valid, passport, valid);
    }

    Ok(count)
}

fn main() -> Result<(), Error> {
    let lines = stdin_lines();

    println!("Part1: {}", part1(&lines)?);
    println!("Part2: {}", part2(&lines)?);

    Ok(())
}


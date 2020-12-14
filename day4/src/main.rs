extern crate nom;

use nom::character::complete::{alpha1, anychar, digit1, space0};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;

use std::fs::read_to_string;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::str::FromStr;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Index<&'_ str> for Passport {
    type Output = Option<String>;
    fn index(&self, s: &str) -> &Option<String> {
        match s {
            "byr" => &self.byr,
            "iyr" => &self.iyr,
            "eyr" => &self.eyr,
            "hgt" => &self.hgt,
            "hcl" => &self.hcl,
            "ecl" => &self.ecl,
            "pid" => &self.pid,
            "cid" => &self.cid,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for Passport {
    fn index_mut(&mut self, s: &str) -> &mut Option<String> {
        match s {
            "byr" => &mut self.byr,
            "iyr" => &mut self.iyr,
            "eyr" => &mut self.eyr,
            "hgt" => &mut self.hgt,
            "hcl" => &mut self.hcl,
            "ecl" => &mut self.ecl,
            "pid" => &mut self.pid,
            "cid" => &mut self.cid,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

impl FromStr for Passport {
    type Err = std::string::ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();
        for token in input.split_whitespace() {
            let key_value: Vec<&str> = token.split(':').collect();
            passport[key_value[0]] = Some(String::from(key_value[1]))
        }
        Ok(passport)
    }
}

impl Passport {
    pub fn from_file(filename: &Path) -> Result<Vec<Passport>, std::io::Error> {
        Ok(read_to_string(filename)?
            .split("\n\n")
            .map(|s| Passport::from_str(s).expect("Failed to parse passport"))
            .collect())
    }

    pub fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid2(&self) -> bool {
        if !self.is_valid() {
            return false;   
        }

        if self.byr.is_none()
            || self.iyr.is_none()
            || self.eyr.is_none()
            || self.hgt.is_none()
            || self.hcl.is_none()
            || self.ecl.is_none()
            || self.pid.is_none() 
        {
                return false;
        }
        if let Some(birth_year) = self.byr {
            
        }

        let birth_year = self.byr.unwrap().parse::<u32>();
        let issue_year = self.iyr.unwrap().parse::<u32>();
        let expiration_year = self.eyr.unwrap().parse::<u32>();
        let height = self.hgt.
        return false if birth_year < 1920 || birth_year > 2020;


        true
}

fn main() -> Result<(), std::io::Error> {
    let passports = Passport::from_file(Path::new("input.txt"))?;

    let num_valid_passports = passports.iter().filter(|p| p.is_valid()).count();
    println!(
        "Total Passports: {}. Valid Passports: {}",
        passports.len(),
        num_valid_passports
    );

    Ok(())
}

extern crate nom;

use nom::sequence::tuple;
use nom::IResult;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, multispace0},
};

use std::fs::read_to_string;

fn is_valid1(passport: &str) -> bool {
    match nom::branch::permutation((
        tuple((tag("byr:"), alpha0, multispace0)),
        tuple((tag("iyr:"), alpha0, multispace0)),
        tuple((tag("eyr:"), alpha0, multispace0)),
        tuple((tag("hgt:"), alpha0, multispace0)),
        tuple((tag("hcl:"), alpha0, multispace0)),
        tuple((tag("ecl:"), alpha0, multispace0)),
        tuple((tag("pid:"), alpha0, multispace0)),
    ))(passport)
    {
        Ok((_, ((_, _, _), (_, _, _), (_, _, _), (_, _, _), (_, _, _), (_, _, _), (_, _, _)))) => {
            true
        }
        Err(_) => false,
    }
}

fn is_valid2(passport: &str) -> bool {
    false
}

fn main() -> Result<(), std::io::Error> {
    let input = read_to_string("input.txt")?;
    let num_valid_passports = input.split("\n\n").map(is_valid1).filter(|b| *b).count();
    println!("Valid Passports: {}", num_valid_passports);

    //let num_valid_passports2 = input.split("\n\n").flat_map(is_valid2).filter(|b| b== true).count();
    Ok(())
}

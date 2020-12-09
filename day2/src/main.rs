extern crate nom;

use nom::character::complete::{alpha1, anychar, digit1, space0};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;
use std::fs::File;
use std::io::{self, BufRead, Error};

// 4-5 m: mmpth
// first implement the basic parsers
#[derive(Debug)]
pub struct Password {
    pub min: u32,
    pub max: u32,
    pub letter: char,
    pub word: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let letter_count = self.word.chars().filter(|l| *l == self.letter).count();
        letter_count >= (self.min as usize) && letter_count <= (self.max as usize)
    }

    fn is_valid2(&self) -> bool {
        (self.word.as_bytes()[(self.min - 1) as usize] == self.letter as u8)
            ^ (self.word.as_bytes()[(self.max - 1) as usize] == self.letter as u8)
    }
}

fn parse_line(i: &str) -> IResult<&str, Password> {
    let (input, (min, _, max, _, letter, _, _, word)) = tuple((
        map_res(digit1, |s: &str| s.parse::<u32>()), // min
        nom::character::complete::char('-'),         // hyphen
        map_res(digit1, |s: &str| s.parse::<u32>()), // max
        space0,                                      // space
        anychar,                                     // char
        nom::character::complete::char(':'),         // colon
        space0,                                      // space
        alpha1,                                      // word
    ))(i)?;

    let password_line = Password {
        min,
        max,
        letter,
        word: String::from(word),
    };

    Ok((input, password_line))
}

fn read_input_file(filename: &str) -> Result<Vec<Password>, std::io::Error> {
    Ok(io::BufReader::new(File::open(filename)?)
        .lines()
        .map(|l| {
            let (_, line) = parse_line(l.expect("Could not parse line").as_str())
                .expect("Could not parse number");
            line
        })
        .collect())
}

fn main() -> Result<(), Error> {
    let input = read_input_file("input.txt")?;

    let part1_count = input.iter().filter(|pass| pass.is_valid()).count();
    println!("Part1 Valid Passwords: {}", part1_count);

    let part2_count = input.iter().filter(|pass| pass.is_valid2()).count();
    println!("Part2 Valid Passwords: {}", part2_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        let password1 = Password {
            min: 4,
            max: 5,
            letter: 'm',
            word: String::from("mmpth"),
        };
        let (_, password2) = parse_line("4-5 m: mmpth").unwrap();
        assert_eq!(password1.min, password2.min);
        assert_eq!(password1.max, password2.max);
        assert_eq!(password1.letter, password2.letter);
        assert_eq!(password1.word, password2.word);
    }
}

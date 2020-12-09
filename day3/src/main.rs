use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, Error};

#[derive(PartialEq)]
pub enum Tile {
    Open,
    Tree,
}

pub struct Forest {
    pub ground: Vec<Vec<Tile>>,
}

impl Forest {
    fn parse_line(line: &str) -> Vec<Tile> {
        line.as_bytes()
            .iter()
            .map(|c| match c {
                b'#' => Tile::Tree,
                _ => Tile::Open,
            })
            .collect()
    }

    pub fn from_input_file(filename: &str) -> Result<Forest, std::io::Error> {
        Ok(Forest {
            ground: io::BufReader::new(File::open(filename)?)
                .lines()
                .map(|l| l.expect("Could not read line"))
                .filter(|l| !l.is_empty())
                .map(|l| Forest::parse_line(l.as_str()))
                .collect(),
        })
    }

    pub fn count_tree_hits(&self, slope: (usize, usize)) -> usize {
        let mut count = 0;
        assert!(self.ground.windows(2).all(|w| w[0].len() == w[1].len()));
        for (i, row) in self.ground.iter().enumerate() {
            if i % slope.1 == 0 {
                let infinite_index = slope.0 * (i / slope.1) % row.len();
                if row[infinite_index] == Tile::Tree {
                    count += 1
                }
            }
        }
        count
    }
}

fn main() -> Result<(), Error> {
    let forest = Forest::from_input_file("input.txt")?;

    println!("Hit {} trees!", forest.count_tree_hits((3, 1)));

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let answers: Vec<usize> = slopes
        .iter()
        .map(|slope| forest.count_tree_hits(*slope))
        .collect();
    println!("Answers: {}", answers.iter().join(" "));
    let answer: usize = answers.iter().product();

    println!("Final Answer: {}", answer);

    Ok(())
}

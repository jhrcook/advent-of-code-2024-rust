use std::{
    cmp::{max, min},
    iter::zip,
};

use crate::data::load;
use log;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

#[derive(Debug, Clone)]
struct LocationLists {
    left: Vec<isize>,
    right: Vec<isize>,
}

impl LocationLists {
    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }
}

fn parse_location_lists(input_str: &str) -> LocationLists {
    let (mut left, mut right) = (Vec::new(), Vec::new());
    for pair in input_str.trim().lines() {
        let mut pair_split = pair.trim().splitn(2, " ");
        let a = pair_split.next().unwrap().trim().parse::<isize>().unwrap();
        let b = pair_split.next().unwrap().trim().parse::<isize>().unwrap();
        left.push(a);
        right.push(b);
    }
    LocationLists { left, right }
}

pub fn puzzle_1(input_data: &str) -> Result<isize, PuzzleErr> {
    log::debug!("Input data: {}", input_data);
    let mut lists = parse_location_lists(input_data);
    log::debug!("Unsorted lists: {:?}", lists);
    lists.sort();
    log::debug!("Sorted lists: {:?}", lists);
    let mut tally = 0;
    for (a, b) in zip(lists.left, lists.right) {
        tally += max(a, b) - min(a, b);
    }
    Ok(tally)
}

pub fn puzzle_2(input_data: &str) -> Result<isize, PuzzleErr> {
    let mut lists = parse_location_lists(input_data);
    let mut tally = 0;
    for x in lists.left {
        let right_count = lists.right.iter().filter(|&n| *n == x).count() as isize;
        tally += x * right_count;
    }
    Ok(tally)
}

pub fn main() {
    log::info!("Day 1: Historian Hysteria");
    let data = load(1, None);
    let result_1 = puzzle_1(&data).unwrap();
    log::info!("Puzzle 1 solution: {}", result_1);
    let result_2 = puzzle_2(&data).unwrap();
    log::info!("Puzzle 2 solution: {}", result_2);
}

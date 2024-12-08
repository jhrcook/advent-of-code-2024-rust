use std::cmp::{max, min};

use crate::data::load;
use log;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Could not locate digit.")]
    NoDigits,
}

enum Safety {
    Safe,
    Unsafe,
}

#[derive(Clone, Debug)]
struct Report {
    levels: Vec<isize>,
}

use itertools::Itertools;

impl Report {
    fn parse_from_input(input_data: &str) -> Self {
        let levels = input_data
            .split(' ')
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect();
        Self { levels }
    }

    fn is_safe(&self) -> bool {
        log::debug!("Checking safety of level: {:?}", self.levels);
        let diffs = self
            .levels
            .iter()
            .tuple_windows()
            .map(|(a, b)| a - b)
            .collect::<Vec<isize>>();

        log::debug!("Differences: {:?}", diffs);

        // Check no diffs greater than 3.
        if diffs.iter().map(|x| x.abs()).max().unwrap() > 3 {
            log::debug!("Found diffs larger than 3 -> UNSAFE");
            return false;
        }

        // Either all pos. or all neg.
        if (diffs.iter().all(|x| x > &0) || diffs.iter().all(|x| x < &0)) {
            log::debug!("All either positive or negative -> SAFE");
            return true;
        } else {
            log::debug!("Not all pos. or neg. -> UNSAFE");
            false
        }
    }
}

fn parse_input(input_data: &str) -> Vec<Report> {
    input_data
        .trim()
        .lines()
        .map(|line| Report::parse_from_input(line))
        .collect()
}

pub fn puzzle_1(input_data: &str) -> Result<isize, PuzzleErr> {
    let reports = parse_input(input_data);
    log::debug!("Reports: {:?}", reports);
    Ok(reports
        .iter()
        .map(|r| if r.is_safe() { 1 } else { 0 })
        .sum())
}

pub fn puzzle_2(input_data: &str) -> Result<isize, PuzzleErr> {
    Ok(0)
}

pub fn main() {
    log::info!("Day 2: Red-Nosed Reports");
    let data = load(2, None);
    let result_1 = puzzle_1(&data).unwrap();
    log::info!("Puzzle 1 solution: {}", result_1);
    // 374 too high

    // let result_2 = puzzle_2(&data).unwrap();
    // log::info!("Puzzle 2 solution: {}", result_2);
}

use aoc_2021::*;
use std::convert::TryFrom;
use thiserror::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let steps: Result<Vec<Step>, StepParseErr> =
        read_input(2)?.lines().map(Step::try_from).collect();
    let steps = steps?;

    println!("{}", part1(&steps));
    println!("{}", part2(&steps));
    Ok(())
}

fn part1(steps: &Vec<Step>) -> isize {
    let mut depth: isize = 0;
    let mut horizontal: usize = 0;

    for &step in steps {
        match step {
            Step::Forward(n) => horizontal += n,
            Step::Down(n) => depth += n as isize,
            Step::Up(n) => depth -= n as isize,
        }
    }

    horizontal as isize * depth
}

fn part2(steps: &Vec<Step>) -> isize {
    let mut depth: isize = 0;
    let mut aim: isize = 0;
    let mut horizontal: usize = 0;

    for &step in steps {
        match step {
            Step::Forward(n) => {
                horizontal += n;
                depth += aim * n as isize;
            }
            Step::Down(n) => aim += n as isize,
            Step::Up(n) => aim -= n as isize,
        }
    }

    horizontal as isize * depth
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl TryFrom<&str> for Step {
    type Error = StepParseErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let split = s.split(' ').collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(StepParseErr::new("input was not two words"));
        }
        let n = match split[1].parse::<usize>() {
            Err(_) => return Err(StepParseErr::new("second word was not a number")),
            Ok(n) => n,
        };

        let step = match split[0] {
            "down" => Step::Down(n),
            "up" => Step::Up(n),
            "forward" => Step::Forward(n),
            other => return Err(StepParseErr::new(format!("unknown command: `{}`", other))),
        };
        Ok(step)
    }
}

#[derive(Clone, Debug, Error)]
#[error("Failed to parse string into a Step: {msg}")]
struct StepParseErr {
    msg: String,
}

impl StepParseErr {
    fn new<S: Into<String>>(s: S) -> Self {
        StepParseErr { msg: s.into() }
    }
}

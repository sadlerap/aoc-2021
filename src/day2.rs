use std::error::Error;

use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut depth = 0;
    let mut pos = 0;
    for s in input.split('\n') {
        let (cmd, val) = s.split_once(' ').unwrap_or_default();
        let val = val.parse::<i32>()?;
        match cmd {
            "up" => depth -= val,
            "down" => depth += val,
            "forward" => pos += val,
            _ => {}
        }
    }
    Ok(depth * pos)
}

#[aoc(day2, part2)]
fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for s in input.split('\n') {
        let (cmd, val) = s.split_once(' ').unwrap_or_default();
        let val = val.parse::<i32>()?;
        match cmd {
            "up" => aim -= val,
            "down" => aim += val,
            "forward" => {
                pos += val;
                depth += aim * val;
            }
            _ => {}
        }
    }
    Ok(depth * pos)
}

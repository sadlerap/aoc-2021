use std::error::Error;

use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut depth = 0;
    let mut pos = 0;
    for s in input.split('\n') {
        let (cmd, val) = s.split_once(' ').unwrap_or_default();
        match cmd {
            "up" => {depth -= val.parse::<i32>()?}
            "down" => {depth += val.parse::<i32>()?}
            "forward" => {pos += val.parse::<i32>()?}
            _ => {}
        }
    }
    Ok(depth * pos)
}

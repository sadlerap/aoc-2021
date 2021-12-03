use aoc_runner_derive::aoc;
use std::error::Error;

#[aoc(day1, part1)]
fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let nums = input.split('\n').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let first = &nums[0..(nums.len()-1)];
    let second = &nums[1..];
    Ok(first
        .iter()
        .zip(second.iter())
        .map(|(x, y)| if (y - x) > 0 {1} else {0})
        .sum::<i32>())
}

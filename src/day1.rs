use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generator(input: &str) -> Vec<i32> {
    input
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> i32 {
    input.windows(2).map(|x| (x[0] < x[1]) as i32).sum()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> i32 {
    input.windows(4).map(|x| (x[0] < x[3]) as i32).sum()
}

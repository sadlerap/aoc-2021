use std::collections::HashSet;

use aoc_runner_derive::*;

#[derive(PartialEq, Eq, Debug)]
struct Problem {
    pub num_bits: usize,
    pub data: Vec<u32>,
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Problem {
    let mut num_bits = 0;
    let data = input
        .split('\n')
        .map(|n| {
            if num_bits == 0 {
                num_bits = n.len();
            }
            let mut x = 0;
            for c in n.bytes() {
                x <<= 1;
                x |= (c == b'1') as u32;
            }
            x
        })
        .collect::<Vec<u32>>();

    Problem { num_bits, data }
}

#[aoc(day3, part1)]
fn part1(input: &Problem) -> i64 {
    let mut gamma = 0;

    for i in (0..input.num_bits).rev() {
        let mask = 1 << i;
        let mut ones = 0;
        let mut zeros = 0;
        for x in input.data.iter() {
            if (x & mask) == 0 {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        let res = if ones > zeros { 1 } else { 0 };
        gamma |= (res as i64) << i;
    }
    let epsilon = (!gamma) & ((1 << input.num_bits) - 1);

    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2(input: &Problem) -> u32 {
    let mut oxygen = None;
    let mut co2 = None;
    let mut o2_set: HashSet<u32> = HashSet::from_iter(input.data.iter().cloned());
    let mut co2_set = o2_set.clone();

    for i in (0..input.num_bits).rev() {
        let mut ones = 0;
        let mut zeros = 0;
        let mask = 1 << i;

        for x in o2_set.iter().cloned() {
            if x & mask == 0 {
                zeros += 1
            } else {
                ones += 1
            }
        }

        if ones >= zeros {
            o2_set.retain(|x| x & mask != 0);
        } else {
            o2_set.retain(|x| x & mask == 0);
        };

        if o2_set.len() == 1 {
            oxygen = o2_set.iter().cloned().next();
            break;
        }
    }

    for i in (0..input.num_bits).rev() {
        let mut ones = 0;
        let mut zeros = 0;
        let mask = 1 << i;
        for x in co2_set.iter().cloned() {
            if x & mask == 0 {
                zeros += 1
            } else {
                ones += 1
            }
        }

        if zeros <= ones {
            co2_set.retain(|x| x & mask == 0);
        } else {
            co2_set.retain(|x| x & mask != 0);
        };

        if co2_set.len() == 1 {
            co2 = co2_set.iter().cloned().next();
            break;
        }
    }

    oxygen.unwrap() * co2.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1_sample_data() {
        let data = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let num_bits = 5;
        let input = Problem { num_bits, data };
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn test_part2_sample_data() {
        let data = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let num_bits = 5;
        let input = Problem { num_bits, data };
        assert_eq!(part2(&input), 230);
    }

    #[test]
    fn test_generator() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

        let data = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let num_bits = 5;
        let problem = Problem { num_bits, data };

        assert_eq!(problem, generator(input));
    }
}

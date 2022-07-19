// use aoc_runner_derive::{aoc, aoc_generator};
// use std::iter::*;

// #[derive(PartialEq, Eq)]
// enum Number {
//     Unmarked(u8),
//     Marked(u8),
// }

// struct Board {
//     board: Vec<Number>,
// }

// struct Bingo {
//     numbers: Vec<u8>,
//     boards: Vec<Board>,
// }

// #[aoc_generator(day4)]
// fn generator(input: &str) -> Bingo {
//     let (first, rest) = input.split_once('\n').unwrap();
//     let numbers = first
//         .split(',')
//         .map(|s| s.parse().unwrap())
//         .collect::<Vec<u8>>();

//     let boards = rest
//         .split('\n')
//         .collect::<Vec<_>>()
//         .as_slice()
//         .chunks(5)
//         .map(|a| {
//             a.iter()
//                 .flat_map(|s| {
//                     s.split_ascii_whitespace()
//                         .map(|i| Number::Unmarked(i.parse().unwrap()))
//                 })
//                 .collect::<Vec<_>>()
//         })
//         .map(|s| Board { board: s })
//         .collect::<Vec<_>>();

//     Bingo { numbers, boards }
// }

// #[aoc(day4, part1)]
// fn part1(input: &Bingo) -> i32 {
//     // for number in input.numbers.into_iter() {
//     //     input.boards
//     //         .iter_mut()
//     //         .filter(|i| Number::Unmarked(number))
//     //         .for_each(|x| {
//     //             *x = Number::Marked(number)
//     //         })
//     // }

//     todo!()
// }

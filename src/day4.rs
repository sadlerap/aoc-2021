use aoc_runner_derive::aoc;
use std::iter::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
struct Number {
    pub number: u8,
    pub marked: bool,
}

impl Number {
    fn is_marked(&self) -> bool {
        self.marked
    }

    fn is_unmarked(&self) -> bool {
        !self.marked
    }
}

#[derive(Debug)]
struct Board {
    board: [Number; 25],
}

impl Board {
    /// Marks `called_number` if it exists on the board.  Returns true if it was marked, false
    /// otherwise.
    fn update(&mut self, called_number: u8) -> bool {
        if let Some(cell) = self
            .board
            .iter_mut()
            .find(|i| i.number == called_number && !i.marked)
        {
            cell.marked = true;
            true
        } else {
            false
        }
    }

    fn check(&self) -> bool {
        const ROW_OFFSETS: [usize; 5] = [0, 5, 10, 15, 20];
        const COLUMN_OFFSETS: [usize; 5] = [0, 1, 2, 3, 4];
        for i in 0..5 {
            if ROW_OFFSETS
                .iter()
                .map(|x| self.board[x + i])
                .all(|x| x.is_marked())
            {
                return true;
            }
            if COLUMN_OFFSETS
                .iter()
                .map(|x| self.board[5 * i + x])
                .all(|x| x.is_marked())
            {
                return true;
            }
        }

        false
    }

    fn score(&self, last_called: u8) -> u32 {
        let remaining_sum: u32 = self
            .board
            .iter()
            .filter(|num| num.is_unmarked())
            .map(|num| num.number as u32)
            .sum();
        remaining_sum * (last_called as u32)
    }
}

pub struct Bingo {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

// #[aoc_generator(day5)]
fn generator(input: &str) -> Bingo {
    let (first, rest) = input.split_once('\n').unwrap();
    let numbers = first
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u8>>();

    let boards = rest
        .split('\n')
        .collect::<Vec<_>>()
        .as_slice()
        .chunks(6)
        .map(|a| {
            let mut board = [Number::default(); 25];

            a.iter()
                .skip(1) //ignore leading whitespace
                .flat_map(|s| {
                    s.split_ascii_whitespace().map(|i| Number {
                        number: i.parse().unwrap(),
                        marked: false,
                    })
                })
                .enumerate()
                .for_each(|(i, num)| {
                    board[i] = num;
                });

            Board { board }
        })
        .collect::<Vec<_>>();

    Bingo { numbers, boards }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let mut bingo = generator(input);
    for number in bingo.numbers.into_iter() {
        if let Some(matching_board) = bingo.boards.iter_mut().find_map(|board| {
            board.update(number);
            if board.check() {
                Some(board)
            } else {
                None
            }
        }) {
            return matching_board.score(number);
        }
    }

    panic!("Unable to find a winning board!")
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let mut bingo = generator(input);
    let num_boards = bingo.boards.len();
    let mut winning_boards = vec![false; num_boards];
    let mut num_false = num_boards;
    for number in bingo.numbers.into_iter() {
        for (i, board) in bingo.boards.iter_mut().enumerate() {
            if winning_boards[i] {
                continue;
            }
            board.update(number);
            if board.check() {
                if num_false == 1 {
                    return board.score(number);
                } else {
                    winning_boards[i] = true;
                    num_false -= 1;
                }
            }
        }
    }

    panic!("Board not found")
}

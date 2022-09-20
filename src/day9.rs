use rayon::prelude::*;
use std::{
    borrow::Cow,
    collections::{BTreeSet, VecDeque},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

struct Grid {
    width: isize,
    height: isize,
    data: Vec<u8>,
}

impl Grid {
    pub fn find_low_points(&self) -> Vec<u8> {
        let mut points = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(i) = self.is_low_point(x, y) {
                    points.push(i)
                }
            }
        }
        points
    }

    pub fn is_low_point(&self, x: isize, y: isize) -> Option<u8> {
        let value = self.try_get(x, y)?;
        if value == 9 {
            return None;
        };
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(p) = self.try_get(x + dx, y + dy) {
                if value >= p {
                    return None;
                };
            }
        }
        Some(value)
    }

    fn try_get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            self.data
                .get((y * self.width + x) as usize)
                .copied()
        }
    }

    pub fn basin_at(&self, x_pos: isize, y_pos: isize) -> BTreeSet<(isize, isize)> {
        // starting at (x, y), flood fill until we hit 9s.
        // keep track of points we've visit and want to visit; we don't want to check points twice.
        let mut point_queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        point_queue.push_back((x_pos, y_pos));

        while let Some((x, y)) = point_queue.pop_front() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_x = x + dx;
                let new_y = y + dy;
                if visited.contains(&(new_x, new_y)) {
                    continue;
                }

                if let Some(v) = self.try_get(new_x, new_y) {
                    if v != 9 {
                        // barrier
                        point_queue.push_back((new_x, new_y))
                    }
                }
            }
            visited.insert((x, y));
        }

        visited
    }

    pub fn basin_sizes(&self) -> Vec<u32> {
        let mut basin_sizes = (0..self.height)
            .par_bridge()
            .flat_map(|y| (0..self.width).map(move |x| (x, y)).collect::<Vec<_>>())
            .filter(|(x, y)| self.is_low_point(*x, *y).is_some())
            .map(|(x, y)| self.basin_at(x, y).len() as u32)
            .collect::<Vec<_>>();
        basin_sizes.sort_by(|x, y| y.cmp(x));
        basin_sizes
    }
}

#[aoc_generator(day9)]
fn generator(input: &str) -> Grid {
    // super annoying bug: aoc_runner strips trailing newlines from inputs, which makes calculating
    // the height here tricky.
    let input = if input.ends_with('\n') {
        let mut x = String::from_str(input).unwrap();
        x.push('\n');
        Cow::Owned(x)
    } else {
        Cow::Borrowed(input)
    };

    let height = input.lines().count().try_into().unwrap();
    let width = input
        .split_once('\n')
        .expect("newlines")
        .0
        .len()
        .try_into()
        .unwrap();
    let data = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| {
            c.to_digit(10)
                .map(|x| x as u8)
                .unwrap_or_else(|| panic!("failed to parse grid entry {c}"))
        })
        .collect::<Vec<_>>();

    Grid {
        width,
        height,
        data,
    }
}

#[aoc(day9, part1)]
fn part1(input: &Grid) -> anyhow::Result<u32> {
    let low_points = input.find_low_points();
    Ok(low_points
        .iter()
        .map(|x| Into::<u32>::into(*x))
        .sum::<u32>()
        + TryInto::<u32>::try_into(low_points.len())?)
}

#[aoc(day9, part2)]
fn part2(input: &Grid) -> u32 {
    let basin_sizes = input.basin_sizes();
    let b1 = basin_sizes[0];
    let b2 = basin_sizes[1];
    let b3 = basin_sizes[2];

    b1 * b2 * b3
}

#[cfg(test)]
mod test {
    use super::*;
    static EXAMPLE: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678\n";
    #[test]
    fn example_data() {
        let data = generator(EXAMPLE);
        let result = part1(&data).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn example_data_result() {
        let data = generator(EXAMPLE);
        let low_points = data.find_low_points();
        assert_eq!(&low_points, &[1, 0, 5, 5]);
    }

    #[test]
    fn parse_dimensions() {
        let data = generator(EXAMPLE);
        assert_eq!(data.width, 10);
        assert_eq!(data.height, 5);
    }

    #[test]
    fn from_data_1() {
        let string_data = "43212\n54101\n65212\n";
        let data = generator(string_data);
        let low_points = data.find_low_points();
        assert_eq!(&low_points, &[0]);
    }

    #[test]
    fn from_data_2() {
        let string_data = "99199\n92019\n";
        let data = generator(string_data);
        let low_points = data.find_low_points();
        assert_eq!(&low_points, &[0]);
    }

    #[test]
    fn parse_real_input_dimensions() {
        let data = generator(include_str!("../input/2021/day9.txt"));
        assert_eq!(data.width, 100);
        assert_eq!(data.height, 100);
    }

    #[test]
    fn example_basins() {
        let data = generator(EXAMPLE);
        let result = part2(&data);
        assert_eq!(result, 1134);
    }

    #[test]
    fn example_basin_sizes() {
        let data = generator(EXAMPLE);
        let result = data.basin_sizes();
        assert_eq!(&result, &[14, 9, 9, 3]);
    }
}

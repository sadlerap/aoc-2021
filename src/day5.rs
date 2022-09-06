use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
    Other,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn parse(data: &str) -> Line {
        let (first, rest) = data.split_once(' ').expect("improperly formatted data");
        let (_, last) = rest.rsplit_once(' ').expect("improperly formatted data");
        let (a, b) = first.split_once(',').unwrap();
        let p1 = Point {
            x: a.parse::<u32>().unwrap(),
            y: b.parse::<u32>().unwrap(),
        };
        let (a, b) = last.split_once(',').unwrap();
        let p2 = Point {
            x: a.parse::<u32>().unwrap(),
            y: b.parse::<u32>().unwrap(),
        };
        Line { p1, p2 }
    }

    fn direction(&self) -> Direction {
        if self.p1.x == self.p2.x {
            Direction::Vertical
        } else if self.p1.y == self.p2.y {
            Direction::Horizontal
        } else if (self.p1.x.abs_diff(self.p2.x)) == (self.p1.y.abs_diff(self.p2.y)) {
            Direction::Diagonal
        } else {
            Direction::Other
        }
    }

    /// Returns every point lying on the line (including the start and end).  Starts at one point
    /// of the line and ends at the other.
    fn line_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        match self.direction() {
            Direction::Vertical => {
                let min = self.p1.y.min(self.p2.y);
                let max = self.p1.y.max(self.p2.y);
                Box::new((min..=max).into_iter().map(|y| Point { x: self.p1.x, y }))
            }
            Direction::Horizontal => {
                let min = self.p1.x.min(self.p2.x);
                let max = self.p1.x.max(self.p2.x);
                Box::new((min..=max).into_iter().map(|x| Point { x, y: self.p1.y }))
            }
            Direction::Diagonal => {
                // start at p1, walk to p2.
                let x_dir = if self.p1.x < self.p2.x { 1 } else { -1_i32 };
                let y_dir = if self.p1.y < self.p2.y { 1 } else { -1_i32 };
                Box::new(
                    (0..=(self.p1.x.abs_diff(self.p2.x)))
                        .into_iter()
                        .map(|x| x as i32)
                        .map(move |x| Point {
                            x: ((self.p1.x as i32) + x * x_dir) as u32,
                            y: ((self.p1.y as i32) + x * y_dir) as u32,
                        }),
                )
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Map {
    lines: Vec<Line>,
    dim_x: u32,
    dim_y: u32,
}

struct Grid {
    data: Vec<u8>,
    dim_y: u32,
}

impl Grid {
    fn new(x: u32, y: u32) -> Grid {
        Grid {
            data: vec![0_u8; ((x+1) as usize) * ((y+1) as usize)],
            dim_y: y,
        }
    }

    fn increment_point(&mut self, point: &Point) {
        self.data[usize::try_from(point.x * self.dim_y + point.y).unwrap()] += 1;
    }

    #[cfg(test)]
    fn get(&self, point: &Point) -> Option<u8> {
        self.data.get(usize::try_from(point.x * self.dim_y + point.y).unwrap()).cloned()
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Map {
    let lines = input.lines().map(Line::parse).collect::<Vec<_>>();
    let max_x = lines.iter().map(|l| l.p1.x.max(l.p2.x)).max().unwrap();
    let max_y = lines.iter().map(|l| l.p1.y.max(l.p2.y)).max().unwrap();

    Map { lines, dim_x: max_x, dim_y: max_y }
}

#[aoc(day5, part1)]
fn part1(input: &Map) -> usize {
    let mut grid = Grid::new(input.dim_x, input.dim_y);
    for line in &input.lines {
        let direction = line.direction();
        if direction != Direction::Horizontal && direction != Direction::Vertical {
            continue;
        }
        for p in line.line_iter() {
            grid.increment_point(&p);
        }
    }

    grid.data.iter().filter(|x| **x >= 2).count()
}

#[aoc(day5, part2)]
fn part2(input: &Map) -> usize {
    let mut grid = Grid::new(input.dim_x, input.dim_y);
    for line in &input.lines {
        if line.direction() == Direction::Other {
            continue;
        }
        for p in line.line_iter() {
            grid.increment_point(&p);
        }
    }

    grid.data.iter().filter(|x| **x >= 2).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_increment_point() {
        let point = Point { x: 1, y: 2 };
        let mut grid = Grid::new(1, 2);

        grid.increment_point(&point);
        assert_eq!(grid.get(&point), Some(1));

        grid.increment_point(&point);
        assert_eq!(grid.get(&point), Some(2));
    }

    #[test]
    fn test_parse_line() {
        let input = "445,187 -> 912,654";
        let line = Line::parse(&input);
        assert_eq!(line.p1, Point { x: 445, y: 187 });
        assert_eq!(line.p2, Point { x: 912, y: 654 });
    }

    #[test]
    fn test_example_part1() {
        let input = include_str!("../input/2021/day5_test.txt");
        let map = generator(input);
        assert_eq!(part1(&map), 5)
    }

    #[test]
    fn test_example_part2() {
        let input = include_str!("../input/2021/day5_test.txt");
        let map = generator(input);
        assert_eq!(part2(&map), 12)
    }

    #[test]
    fn test_line_iter_horizontal() {
        let line = Line::parse("4,5 -> 8,5");
        let mut iter = line.line_iter();
        assert_eq!(iter.next(), Some(Point { x: 4, y: 5 }));
        assert_eq!(iter.next(), Some(Point { x: 5, y: 5 }));
        assert_eq!(iter.next(), Some(Point { x: 6, y: 5 }));
        assert_eq!(iter.next(), Some(Point { x: 7, y: 5 }));
        assert_eq!(iter.next(), Some(Point { x: 8, y: 5 }));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_line_iter_vertical() {
        let line = Line::parse("4,5 -> 4,8");
        let mut iter = line.line_iter();
        assert_eq!(iter.next(), Some(Point { x: 4, y: 5 }));
        assert_eq!(iter.next(), Some(Point { x: 4, y: 6 }));
        assert_eq!(iter.next(), Some(Point { x: 4, y: 7 }));
        assert_eq!(iter.next(), Some(Point { x: 4, y: 8 }));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_line_iter_diagonal() {
        let line = Line::parse("3,5 -> 6,8");
        let mut iter = line.line_iter();
        assert_eq!(iter.next(), Some(Point { x: 3, y: 5 }));
        assert_eq!(iter.next(), Some(Point { x: 4, y: 6 }));
        assert_eq!(iter.next(), Some(Point { x: 5, y: 7 }));
        assert_eq!(iter.next(), Some(Point { x: 6, y: 8 }));
        assert_eq!(iter.next(), None);
    }
}

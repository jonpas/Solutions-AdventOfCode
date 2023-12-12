use std::collections::HashMap;
use std::ops::Add;

fn main() {
    let input = include_str!("./d10_input");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Any,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Ground,
    Start,
}

impl Tile {
    fn from_char(character: char) -> Tile {
        match character {
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::BendNorthEast,
            'J' => Tile::BendNorthWest,
            '7' => Tile::BendSouthWest,
            'F' => Tile::BendSouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("invalid tile type"),
        }
    }
}

fn process(input: &str) -> String {
    let mut tiles: HashMap<Position, Tile> = HashMap::new();

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, character)| {
            tiles
                .entry(Position {
                    row: row as i32,
                    col: col as i32,
                })
                .or_insert(Tile::from_char(character));
        });
    });

    let start = tiles
        .iter()
        .find_map(|(position, tile)| {
            (tile == &Tile::Start).then_some((Direction::Any, *position, Direction::Any))
        })
        .expect("start position should always exist");
    //dbg!(&start);

    let mut nexts: Vec<(Direction, Position, Direction)> = vec![start.clone(), start.clone()];

    let mut steps = 0;
    while &nexts[0].1 != &nexts[1].1 || steps == 0 {
        nexts = nexts
            .iter()
            .enumerate()
            .map(|(index, (_, pos, dir_to))| {
                //dbg!((&pos, &dir_to));

                let north_pos = *pos + Position { row: -1, col: 0 };
                let south_pos = *pos + Position { row: 1, col: 0 };
                let east_pos = *pos + Position { row: 0, col: 1 };
                let west_pos = *pos + Position { row: 0, col: -1 };

                let mut nexts = vec![];
                if dir_to == &Direction::North || dir_to == &Direction::Any {
                    nexts.push(
                        tiles
                            .get(&north_pos)
                            .is_some_and(|tile| match tile {
                                Tile::VerticalPipe | Tile::BendSouthWest | Tile::BendSouthEast => {
                                    true
                                }
                                _ => false,
                            })
                            .then_some((Direction::South, north_pos)),
                    );
                }
                if dir_to == &Direction::South || dir_to == &Direction::Any {
                    nexts.push(
                        tiles
                            .get(&south_pos)
                            .is_some_and(|tile| match tile {
                                Tile::VerticalPipe | Tile::BendNorthWest | Tile::BendNorthEast => {
                                    true
                                }
                                _ => false,
                            })
                            .then_some((Direction::North, south_pos)),
                    );
                }
                if dir_to == &Direction::East || dir_to == &Direction::Any {
                    nexts.push(
                        tiles
                            .get(&east_pos)
                            .is_some_and(|tile| match tile {
                                Tile::HorizontalPipe
                                | Tile::BendSouthWest
                                | Tile::BendNorthWest => true,
                                _ => false,
                            })
                            .then_some((Direction::West, east_pos)),
                    );
                }
                if dir_to == &Direction::West || dir_to == &Direction::Any {
                    nexts.push(
                        tiles
                            .get(&west_pos)
                            .is_some_and(|tile| match tile {
                                Tile::HorizontalPipe
                                | Tile::BendNorthEast
                                | Tile::BendSouthEast => true,
                                _ => false,
                            })
                            .then_some((Direction::East, west_pos)),
                    );
                }

                nexts = nexts.into_iter().filter(|n| n.is_some()).collect();
                //dbg!(&nexts);

                let next = match nexts.len() {
                    // Move in different directions from start
                    2 => nexts
                        .into_iter()
                        .nth(index)
                        .expect("should have a valid next position")
                        .expect("should be a valid connecting pipe"),
                    1 => nexts
                        .into_iter()
                        .nth(0)
                        .expect("should have a valid next position")
                        .expect("should be a valid connecting pipe"),
                    _ => {
                        panic!("should never have move in more than 1 direction (or 2 from start)")
                    }
                };

                let next_tile = tiles.get(&next.1).expect("position should exist");
                let dir_to_go = match (next.0, next_tile) {
                    (Direction::North, Tile::VerticalPipe) => Direction::South,
                    (Direction::South, Tile::VerticalPipe) => Direction::North,
                    (Direction::West, Tile::HorizontalPipe) => Direction::East,
                    (Direction::East, Tile::HorizontalPipe) => Direction::West,
                    (Direction::North, Tile::BendNorthEast) => Direction::East,
                    (Direction::East, Tile::BendNorthEast) => Direction::North,
                    (Direction::North, Tile::BendNorthWest) => Direction::West,
                    (Direction::West, Tile::BendNorthWest) => Direction::North,
                    (Direction::South, Tile::BendSouthWest) => Direction::West,
                    (Direction::West, Tile::BendSouthWest) => Direction::South,
                    (Direction::South, Tile::BendSouthEast) => Direction::East,
                    (Direction::East, Tile::BendSouthEast) => Direction::South,
                    _ => unreachable!("should not land on ground or reach start"),
                };

                //dbg!(&next);
                (next.0, next.1, dir_to_go)
            })
            .collect();

        steps += 1;
    }

    dbg!(&steps);
    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "4"
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        "8"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }
}

use std::collections::{HashMap, HashSet};
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

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
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

    fn to_char(&self) -> char {
        match self {
            Tile::VerticalPipe => '│',
            Tile::HorizontalPipe => '─',
            Tile::BendNorthEast => '└',
            Tile::BendNorthWest => '┘',
            Tile::BendSouthWest => '┐',
            Tile::BendSouthEast => '┌',
            Tile::Ground => '·',
            Tile::Start => 'S',
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
    let mut network_tiles: HashSet<Position> = HashSet::from([start.1]);
    let mut start_neighbours = vec![Direction::Any, Direction::Any];

    while &nexts[0].1 != &nexts[1].1 || nexts[0].0 == Direction::Any {
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

                let mut is_start = false;
                let next = match nexts.len() {
                    // Move in different directions from start
                    2 => {
                        is_start = true;
                        nexts
                            .into_iter()
                            .nth(index)
                            .expect("should have a valid next position")
                            .expect("should be a valid connecting pipe")
                    }
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

                if is_start {
                    //tiles.get(&start.1).and_modify();
                    start_neighbours[index] = next.0;
                }

                network_tiles.insert(next.1);

                //dbg!(&next);
                (next.0, next.1, dir_to_go)
            })
            .collect();
    }

    dbg!(&start_neighbours);
    let start_mimics_type = match (start_neighbours[0], start_neighbours[1]) {
        (Direction::North, Direction::South) => Tile::HorizontalPipe,
        (Direction::South, Direction::North) => Tile::HorizontalPipe,
        (Direction::East, Direction::West) => Tile::VerticalPipe,
        (Direction::West, Direction::East) => Tile::VerticalPipe,
        (Direction::North, Direction::West) => Tile::BendSouthEast,
        (Direction::West, Direction::North) => Tile::BendSouthEast,
        (Direction::South, Direction::West) => Tile::BendSouthWest,
        (Direction::West, Direction::South) => Tile::BendSouthWest,
        (Direction::North, Direction::East) => Tile::BendNorthEast,
        (Direction::East, Direction::North) => Tile::BendNorthEast,
        (Direction::South, Direction::East) => Tile::BendNorthWest,
        (Direction::East, Direction::South) => Tile::BendNorthWest,
        _ => unreachable!("all directions should be accounted for"),
    };

    let max_row = input.lines().count() as i32;
    let max_col = input.lines().next().unwrap().len() as i32;
    dbg!((&max_row, &max_col), &network_tiles.len());

    let mut enclosed = 0;

    for row in 0..max_row {
        let mut crossings = 0;

        for col in 0..max_col {
            //dbg!(row, col);
            let pos = Position { row, col };
            let tile = tiles.get(&pos).expect("should be a valid position");

            match network_tiles.contains(&pos) {
                true => {
                    print!("{}", tile.to_char());

                    if tile == &Tile::VerticalPipe
                        || tile == &Tile::BendSouthEast
                        || tile == &Tile::BendSouthWest
                    {
                        crossings += 1;
                    }

                    // Special handling for Start tile
                    if tile == &Tile::Start
                        && (start_mimics_type == Tile::VerticalPipe
                            || start_mimics_type == Tile::BendSouthEast
                            || start_mimics_type == Tile::BendSouthWest)
                    {
                        crossings += 1;
                    }
                }
                false => {
                    if crossings % 2 != 0 {
                        print!("·");
                        enclosed += 1;
                    } else {
                        print!(" ");
                    }
                }
            }
        }
        print!("\n");
    }

    dbg!(&enclosed);
    enclosed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        "4"
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        "8"
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        "10"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }
}

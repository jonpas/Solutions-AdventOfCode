use std::collections::HashSet;

fn main() {
    let input = include_str!("./d16_input");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq)]
enum Mirror {
    ReflectLeft,     // '\'
    ReflectRight,    // '/'
    SplitVertical,   // '|'
    SplitHorizontal, // '-'
    Empty,
}

impl Mirror {
    fn from_char(character: char) -> Mirror {
        match character {
            '\\' => Mirror::ReflectLeft,
            '/' => Mirror::ReflectRight,
            '|' => Mirror::SplitVertical,
            '-' => Mirror::SplitHorizontal,
            '.' => Mirror::Empty,
            _ => unreachable!("all mirror types should be accounted for"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Mirror::ReflectLeft => '\\',
            Mirror::ReflectRight => '/',
            Mirror::SplitVertical => '|',
            Mirror::SplitHorizontal => '-',
            Mirror::Empty => '.',
        }
    }
}

struct Grid {
    grid: Vec<Vec<Mirror>>,
}

impl Grid {
    fn new() -> Self {
        Grid { grid: vec![] }
    }

    fn dimensions(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    fn max_dimensions(&self) -> (usize, usize) {
        (self.grid.len() - 1, self.grid[0].len() - 1)
    }

    fn get(&self, beam: &Beam) -> &Mirror {
        &self.grid[beam.position.0][beam.position.1]
    }

    fn get_by_xy(&self, xy: &(usize, usize)) -> &Mirror {
        &self.grid[xy.0][xy.1]
    }

    fn start_beams(&self) -> Vec<Beam> {
        let dimensions = self.dimensions();
        let max_dimensions = self.max_dimensions();

        let mut start_beams: Vec<Beam> = vec![];

        for row in 0..dimensions.0 {
            start_beams.push(Beam {
                position: (row, 0),
                direction: Direction::Right,
                max_dimensions,
            });
            start_beams.push(Beam {
                position: (row, max_dimensions.1),
                direction: Direction::Left,
                max_dimensions,
            });
        }

        for col in 0..dimensions.1 {
            start_beams.push(Beam {
                position: (0, col),
                direction: Direction::Down,
                max_dimensions,
            });
            start_beams.push(Beam {
                position: (max_dimensions.0, col),
                direction: Direction::Up,
                max_dimensions,
            });
        }

        start_beams
    }

    fn print(&self) {
        let dimensions = self.dimensions();
        //dbg!(&dimensions);

        for row in 0..dimensions.0 {
            for col in 0..dimensions.1 {
                print!("{}", self.get_by_xy(&(row, col)).to_char());
            }
            println!();
        }
    }

    fn print_energized(&self, energized: &HashSet<(usize, usize)>) {
        let dimensions = self.dimensions();
        //dbg!(&dimensions);

        for row in 0..dimensions.0 {
            for col in 0..dimensions.1 {
                if energized.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn print_beams(&self, beams: &Vec<Beam>) {
        let dimensions = self.dimensions();
        //dbg!(&dimensions);

        for row in 0..dimensions.0 {
            for col in 0..dimensions.1 {
                match beams.iter().find(|&&beam| beam.position == (row, col)) {
                    Some(_) => print!("S"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

impl FromIterator<Vec<Mirror>> for Grid {
    fn from_iter<I: IntoIterator<Item = Vec<Mirror>>>(iter: I) -> Self {
        let mut c = Grid::new();
        for i in iter {
            c.grid.push(i);
        }
        c
    }
}

#[derive(Debug, Copy, Clone)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
    max_dimensions: (usize, usize),
}

impl Beam {
    fn split(&mut self, directions: (Direction, Direction)) -> Self {
        let new_beam = Beam {
            position: self.position,
            direction: directions.1,
            max_dimensions: self.max_dimensions,
        };

        self.direction = directions.0;
        new_beam
    }

    fn step(&mut self) -> bool {
        //dbg!(self.position);
        match self.direction {
            Direction::Left => {
                if self.position.1 == 0 {
                    return false;
                }
                self.position.1 -= 1;
            }
            Direction::Right => {
                if self.position.1 == self.max_dimensions.1 {
                    return false;
                }
                self.position.1 += 1;
            }
            Direction::Up => {
                if self.position.0 == 0 {
                    return false;
                }
                self.position.0 -= 1;
            }
            Direction::Down => {
                if self.position.0 == self.max_dimensions.0 {
                    return false;
                }
                self.position.0 += 1;
            }
        }
        true
    }
}

fn process(input: &str) -> String {
    let grid: Grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Mirror::from_char(c))
                .collect::<Vec<Mirror>>()
        })
        .collect();
    grid.print();

    let mut beams = grid.start_beams();
    grid.print_beams(&beams);

    let energy = beams
        .iter_mut()
        .map(|mut beam| {
            let mut energized: HashSet<(usize, usize)> = HashSet::from([beam.position]);
            walk(&grid, &mut beam, &mut energized);
            //grid.print_energized(&energized);
            dbg!(&energized.len());
            energized.len()
        })
        .max()
        .expect("should find an energy");

    dbg!(&energy);
    energy.to_string()
}

fn walk(grid: &Grid, beam: &mut Beam, energized: &mut HashSet<(usize, usize)>) {
    let mirror = grid.get(&beam);
    //dbg!(&mirror);

    if (mirror == &Mirror::SplitVertical || mirror == &Mirror::SplitHorizontal)
        && energized.contains(&beam.position)
    {
        return;
    }

    energized.insert(beam.position);

    match mirror {
        Mirror::ReflectLeft => {
            beam.direction = match beam.direction {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            };
            if beam.step() {
                walk(grid, &mut beam.clone(), energized);
            }
        }
        Mirror::ReflectRight => {
            beam.direction = match beam.direction {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            };
            if beam.step() {
                walk(grid, &mut beam.clone(), energized);
            }
        }
        Mirror::SplitVertical => {
            match beam.direction {
                Direction::Left | Direction::Right => {
                    let mut new_beam = beam.split((Direction::Up, Direction::Down));
                    if new_beam.step() {
                        walk(grid, &mut new_beam.clone(), energized);
                    }
                    if beam.step() {
                        walk(grid, &mut beam.clone(), energized);
                    }
                }
                Direction::Up | Direction::Down => {
                    if beam.step() {
                        walk(grid, &mut beam.clone(), energized);
                    }
                }
            };
        }
        Mirror::SplitHorizontal => {
            match beam.direction {
                Direction::Up | Direction::Down => {
                    let mut new_beam = beam.split((Direction::Left, Direction::Right));
                    if new_beam.step() {
                        walk(grid, &mut new_beam.clone(), energized);
                    }
                    if beam.step() {
                        walk(grid, &mut beam.clone(), energized);
                    }
                }
                Direction::Left | Direction::Right => {
                    if beam.step() {
                        walk(grid, &mut beam.clone(), energized);
                    }
                }
            };
        }
        Mirror::Empty => {
            if beam.step() {
                walk(grid, &mut beam.clone(), energized);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!("51", process(input));
    }
}

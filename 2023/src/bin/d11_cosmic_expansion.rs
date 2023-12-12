use itertools::Itertools;

fn main() {
    let input = include_str!("./d11_input");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Galaxy,
    Empty,
}

impl Tile {
    fn from_char(character: char) -> Tile {
        match character {
            '#' => Tile::Galaxy,
            '.' => Tile::Empty,
            _ => unreachable!("all tile types should be accounted for"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Galaxy => '#',
            Tile::Empty => '.',
        }
    }
}

fn print_cosmos(cosmos: &Vec<Tile>, dimensions: (usize, usize)) {
    for row in 0..dimensions.0 {
        for col in 0..dimensions.1 {
            print!("{}", cosmos[row * dimensions.1 + col].to_char());
        }
        print!("\n");
    }
}

fn process(input: &str) -> String {
    // Parse input
    let mut dimensions = (input.lines().count(), input.lines().next().unwrap().len());

    let mut cosmos: Vec<Tile> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| Tile::from_char(c)))
        .collect();

    dbg!(&dimensions);
    print_cosmos(&cosmos, dimensions);

    // Expand cosmos (duplicate empty rows and columns)
    let mut insert_rows: Vec<usize> = vec![];
    let mut insert_cols: Vec<usize> = vec![];
    for row in 0..dimensions.0 {
        if cosmos
            .iter()
            .skip(row * dimensions.0)
            .take(dimensions.1)
            .all(|tile| tile == &Tile::Empty)
        {
            //dbg!("empty row");
            insert_rows.push(row);
        }
    }
    for col in 0..dimensions.1 {
        if cosmos
            .iter()
            .skip(col)
            .step_by(dimensions.0)
            .all(|tile| tile == &Tile::Empty)
        {
            //dbg!("empty col");
            insert_cols.push(col);
        }
    }

    for insert_row in insert_rows.iter().rev() {
        for col in (0..dimensions.1).rev() {
            cosmos.insert(insert_row * dimensions.1 + col, Tile::Empty);
        }
        dimensions.0 += 1;
    }

    for insert_col in insert_cols.iter().rev() {
        for row in (0..dimensions.0).rev() {
            cosmos.insert(row * dimensions.1 + insert_col, Tile::Empty);
        }
        dimensions.1 += 1;
    }

    dbg!(&dimensions, &cosmos.len());
    print_cosmos(&cosmos, dimensions);

    // Calculate distances between all pairs
    let galaxies: Vec<(usize, usize)> = cosmos
        .iter()
        .enumerate()
        .filter(|(_, tile)| *tile == &Tile::Galaxy)
        .map(|(index, _)| (index % dimensions.1, index / dimensions.1))
        .collect();
    //dbg!(&galaxies);

    let distance: usize = galaxies
        .iter()
        .combinations(2)
        .map(|c| {
            let (a, b, c, d) = (c[0].0 as i64, c[0].1 as i64, c[1].0 as i64, c[1].1 as i64);
            let manhattan_distance = (a - c).abs() + (b - d).abs();
            manhattan_distance as usize
        })
        .sum();

    dbg!(&distance);
    distance.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input));
    }
}

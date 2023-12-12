use itertools::Itertools;

fn main() {
    let input = include_str!("./d11_input");
    let output = process(input, 1000000);
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

fn process(input: &str, expand_factor: usize) -> String {
    // Parse input
    let dimensions = (input.lines().count(), input.lines().next().unwrap().len());

    let cosmos: Vec<Tile> = input
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

    insert_rows.sort();
    insert_cols.sort();

    dbg!(&insert_rows, &insert_cols);
    dbg!((
        &insert_rows.len() * (expand_factor - 1),
        &insert_cols.len() * (expand_factor - 1)
    ));

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
            let (a_col, a_row, b_col, b_row) = (c[0].0, c[0].1, c[1].0, c[1].1);

            let empty_rows = insert_rows
                .iter()
                .filter(|row| *row >= &a_row.min(b_row) && *row <= &b_row.max(a_row))
                .count()
                * (expand_factor - 1);
            let empty_cols = insert_cols
                .iter()
                .filter(|col| *col >= &a_col.min(b_col) && *col <= &b_col.max(a_col))
                .count()
                * (expand_factor - 1);
            //dbg!(&c, (&empty_rows, &empty_cols));

            let (a, b, c, d) = (a_col as i64, a_row as i64, b_col as i64, b_row as i64);

            let manhattan_distance =
                (a - c).abs() + (b - d).abs() + empty_rows as i64 + empty_cols as i64;
            //dbg!(&manhattan_distance);
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
        assert_eq!("1030", process(input, 10));
    }

    #[test]
    fn test_simple() {
        let input = "#...
....
....
....
...#";
        assert_eq!("70", process(input, 10));
    }
}

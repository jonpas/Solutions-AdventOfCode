use std::collections::VecDeque;

fn main() {
    let input = include_str!("./d14_input");
    let output = process(input);
    dbg!(output);
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let dimensions = (grid[0].len(), grid.len());
    dbg!(&dimensions);

    for col in 0..dimensions.0 {
        for row in 0..dimensions.1 {
            print!("{}", grid[row][col]);
        }
        print!("\n");
    }
}

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..grid[0].len())
        .map(|i| {
            grid.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn process(input: &str) -> String {
    let grid = transpose(to_grid(input));
    print_grid(&grid);

    let grid_slide = slide_rocks(grid);
    print_grid(&grid_slide);

    let load: usize = grid_slide
        .iter()
        .map(|col| {
            col.iter()
                .rev()
                .enumerate()
                .map(|(index, tile)| if tile == &'O' { index + 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();

    load.to_string()
}

fn slide_rocks(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    println!("\nslide");
    for col in 0..grid.len() {
        let mut move_to: VecDeque<usize> = VecDeque::new();

        for row in 0..(grid[0].len()) {
            let tile = grid[col][row];
            //println!("{}", tile);
            match tile {
                '.' => {
                    move_to.push_back(row);
                }
                '#' => {
                    move_to.clear();
                }
                'O' => match move_to.pop_front() {
                    Some(move_to_row) => {
                        grid[col][move_to_row] = 'O';

                        grid[col][row] = '.';
                        move_to.push_back(row);
                    }
                    None => (),
                },
                _ => unreachable!("no other character should be on the grid"),
            }
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let expected = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        assert_eq!(
            transpose(to_grid(expected)),
            slide_rocks(transpose(to_grid(input)))
        );
    }

    #[test]
    fn test_process() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("136", process(input));
    }
}

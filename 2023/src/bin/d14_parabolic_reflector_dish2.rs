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

// Rotate by -90 (transpose -> reverse each column)
fn rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(grid).into_iter().rev().collect()
}

fn process(input: &str) -> String {
    let grid = transpose(to_grid(input));
    print_grid(&grid);

    let grid_slide = slide_n_cycles(grid, 1000000000);
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

fn slide_n_cycles(mut grid: Vec<Vec<char>>, cycles: usize) -> Vec<Vec<char>> {
    let mut iterations: Vec<String> = vec![];

    let mut remaining = 0;
    for i in 0..cycles {
        grid = slide_cycle(grid);

        let grid_flat: String = grid.iter().flatten().collect();
        match iterations.iter().position(|it| it == &grid_flat) {
            Some(index) => {
                let period = iterations.len() - index;
                remaining = (1000000000 - i - 1) % period;
                dbg!(&period, &remaining);
                break;
            }
            None => {
                iterations.push(grid.iter().flatten().collect());
            }
        }
    }

    for _ in 0..remaining {
        grid = slide_cycle(grid);
    }

    grid
}

fn slide_cycle(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    //println!("\nslide north");
    grid = slide_rocks(grid);
    //print_grid(&grid);

    //println!("\nrotate & slide west");
    grid = slide_rocks(rotate(grid));
    //print_grid(&grid);

    //println!("\nrotate & slide south");
    grid = slide_rocks(rotate(grid));
    //print_grid(&grid);

    //println!("\nrotate & slide east");
    grid = slide_rocks(rotate(grid));
    //print_grid(&grid);

    grid = rotate(grid);
    //print_grid(&grid);

    grid
}

fn slide_rocks(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
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
    fn test_cycle_1() {
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
        let expected = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        assert_eq!(
            transpose(to_grid(expected)),
            slide_n_cycles(transpose(to_grid(input)), 1)
        );
    }

    #[test]
    fn test_cycle_2() {
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
        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        assert_eq!(
            transpose(to_grid(expected)),
            slide_n_cycles(transpose(to_grid(input)), 2)
        );
    }

    #[test]
    fn test_cycle_3() {
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
        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        assert_eq!(
            transpose(to_grid(expected)),
            slide_n_cycles(transpose(to_grid(input)), 3)
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
        assert_eq!("64", process(input));
    }
}

use pathfinding::prelude::dijkstra;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("./d17_input");
    let output = process(input);
    dbg!(output);
}

fn print_grid(grid: &Vec<Vec<i32>>, dimensions: &(i32, i32)) {
    for row in 0..dimensions.0 {
        for col in 0..dimensions.1 {
            print!("{}", grid[row as usize][col as usize]);
        }
        println!();
    }
}

fn print_visited(grid: &Vec<Vec<i32>>, dimensions: &(i32, i32), visited: &HashSet<(i32, i32)>) {
    for row in 0..dimensions.0 {
        for col in 0..dimensions.1 {
            if visited.contains(&(row, col)) {
                print!("#");
            } else {
                print!("{}", grid[row as usize][col as usize]);
            }
        }
        println!();
    }
}

fn process(input: &str) -> String {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("should be a number") as i32)
                .collect()
        })
        .collect();
    let dimensions = (grid.len() as i32, grid[0].len() as i32);

    let goal: (i32, i32) = (dimensions.0 - 1, dimensions.1 - 1);
    print_grid(&grid, &dimensions);
    dbg!(&goal);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    //let mut last: VecDeque<(i32, i32)> = VecDeque::from([(0, 0)]);
    let result = dijkstra(
        &((0, 0), VecDeque::from([(0, 0)])),
        |&((x, y), last)| {
            vec![
                (x as i32, y as i32 + 1),
                (x as i32, y as i32 - 1),
                (x as i32 + 1, y as i32),
                (x as i32 - 1, y as i32),
            ]
            .into_iter()
            .filter_map(|p| {
                if p.0 < 0 || p.1 < 0 || p.0 >= dimensions.0 || p.1 >= dimensions.1 {
                    return None;
                }

                let mut new_last = last.clone();
                new_last.push_front(p);
                if new_last.len() == 4 {
                    new_last.pop_back();
                }

                Some((p, new_last))
            })
            .map(|p| {
                let cost = grid[p.0 .0 as usize][p.0 .1 as usize];
                (p, cost)
            })
        },
        |(p, last)| {
            visited.insert(*p);
            if *p == goal {
                true
            } else {
                false
            }
        },
    )
    .expect("should find a path")
    .1;

    print_visited(&grid, &dimensions, &visited);

    dbg!(&result);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!("102", process(input));
    }
}

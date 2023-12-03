use std::collections::HashMap;

fn input() -> String {
    String::from(
        r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "#,
    )
}

fn parse_number(mut grid: Vec<Vec<char>>, y: usize, x: usize) -> (Vec<Vec<char>>, i64, usize) {
    let mut digits: Vec<char> = vec![];
    let mut d = grid[y][x];
    let mut dx = x;
    while d.is_numeric() {
        digits.push(d);
        // update grid to avoid double-counting
        grid[y][dx] = '.';
        if dx < grid[y].len() - 1 {
            dx += 1;
            d = grid[y][dx];
        } else {
            break;
        }
    }
    let part_number: i64 = digits.iter().collect::<String>().as_str().parse().unwrap();
    (grid, part_number, dx)
}

fn get_neighbors(y: usize, x: usize, dx: usize, ncols: usize) -> Vec<(usize, usize)> {
    let start: usize = if x > 0 { x - 1 } else { x };
    let end: usize = dx;
    let mut neighbors: Vec<(usize, usize)> = vec![];
    for col in start..=end {
        if y > 0 {
            neighbors.push((y - 1, col));
        }
        neighbors.push((y, col));
        if y < (ncols - 1) {
            neighbors.push((y + 1, col));
        }
    }
    neighbors
}

pub fn part1() {
    let mut total: i64 = 0;
    let input = input();
    let input_str = input.as_str();
    let mut grid: Vec<Vec<char>> = input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let nrows = grid.len();
    let ncols = grid[0].len();
    for y in 0..nrows {
        for x in 0..ncols {
            let item = grid[y][x];
            if item.is_numeric() {
                // get part number
                let (new_grid, part_number, dx) = parse_number(grid, y, x);
                grid = new_grid;
                // check neighbors
                let neighbors = get_neighbors(y, x, dx, ncols);
                for (y2, x2) in neighbors {
                    let neighbor_item = grid[y2][x2];
                    if !neighbor_item.is_numeric() && neighbor_item != '.' {
                        total += part_number;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", total)
}

pub fn part2() {
    let input = input();
    let input_str = input.as_str();
    let mut grid: Vec<Vec<char>> = input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let nrows = grid.len();
    let ncols = grid[0].len();
    // id => (# of adjacent parts, gear ratio if # of adjacent parts is 2 otherwise garbage)
    let mut gear_ratios = HashMap::new();
    for y in 0..nrows {
        for x in 0..ncols {
            let item = grid[y][x];
            if item.is_numeric() {
                // get part number
                let (new_grid, part_number, dx) = parse_number(grid, y, x);
                grid = new_grid;
                // check neighbors
                let neighbors = get_neighbors(y, x, dx, ncols);
                // update gear ratios
                for (y2, x2) in neighbors {
                    let neighbor_item = grid[y2][x2];
                    if !neighbor_item.is_numeric() && neighbor_item != '.' {
                        let id = x2 + y2 * ncols;
                        match gear_ratios.get(&id) {
                            Some((count, ratio)) => {
                                gear_ratios.insert(
                                    id,
                                    (
                                        *count + 1,
                                        if *count < 2 { *ratio * part_number } else { 0 },
                                    ),
                                );
                            }
                            _ => {
                                gear_ratios.insert(id, (1, part_number));
                            }
                        }
                    }
                }
            }
        }
    }
    let total: i64 = gear_ratios
        .values()
        .filter(|(count, _)| count == &2)
        .map(|(_, ratio)| ratio)
        .sum();
    println!("{}", total)
}

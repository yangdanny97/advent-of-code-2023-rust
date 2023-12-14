fn input() -> String {
    String::from(
        r#"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "#,
    )
}

fn parse_grid(input_str: &str) -> Vec<Vec<char>> {
    input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn load(grid: &Vec<Vec<char>>) -> i64 {
    let mut total: i64 = 0;
    for col in 0..grid[0].len() {
        let mut next = 0;
        for row in 0..grid.len() {
            match grid[row][col] {
                'O' => {
                    total += grid.len() as i64 - next;
                    next += 1;
                }
                '#' => next = row as i64 + 1,
                _ => {}
            }
        }
    }
    total
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let grid = parse_grid(input_str);
    let total = load(&grid);
    println!("{}", total)
}

fn map_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = grid.clone();
    for col in 0..grid[0].len() {
        let mut next = 0;
        for row in 0..grid.len() {
            match grid[row][col] {
                'O' => {
                    result[next][col] = 'O';
                    if row != next {
                        result[row][col] = '.';
                    }
                    next += 1;
                }
                '#' => {
                    next = row + 1;
                }
                _ => {}
            }
        }
    }
    result
}

// rotates grid right 90 degrees
fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![vec!['.'; grid.len()]; grid[0].len()];
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            result[col][grid[0].len() - 1 - row] = grid[row][col];
        }
    }
    result
}

fn load2(grid: &Vec<Vec<char>>) -> i64 {
    let mut total: i64 = 0;
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            match grid[row][col] {
                'O' => {
                    total += grid.len() as i64 - row as i64;
                }
                _ => {}
            }
        }
    }
    total
}

pub fn part2() {
    let input = input();
    let input_str = input.as_str();
    let mut grid = parse_grid(input_str);
    let mut results = Vec::new();
    // assumes the cycle is reached within 500 iters
    loop {
        let total = load2(&grid);
        results.push(total);
        grid = map_grid(&grid);
        grid = rotate_grid(&grid);
        grid = map_grid(&grid);
        grid = rotate_grid(&grid);
        grid = map_grid(&grid);
        grid = rotate_grid(&grid);
        grid = map_grid(&grid);
        grid = rotate_grid(&grid);
        if results.len() > 500 {
            break;
        }
    }
    // assumes the last number only occurs once in the cycle
    let mut cycle_length = 0;
    for i in (0..results.len() - 2).rev() {
        if results[i] == results[results.len() - 1] {
            cycle_length = (results.len() - 1) - i;
            break;
        }
    }
    let rem = 1000000000 % cycle_length;
    for i in (0..results.len() - 1).rev() {
        if i % cycle_length == rem {
            println!("{}", results[i]);
            return;
        }
    }
}

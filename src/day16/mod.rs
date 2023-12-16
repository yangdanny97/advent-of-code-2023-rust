fn input() -> String {
    String::from(
        r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....        
    "#,
    )
}

#[repr(usize)]
#[derive(Clone, Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
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

fn energize(
    y: i64,
    x: i64,
    direction: Direction,
    grid: &Vec<Vec<char>>,
    mut light: Vec<Vec<Vec<bool>>>,
) -> Vec<Vec<Vec<bool>>> {
    if x < 0 || y < 0 || x >= grid[0].len() as i64 || y >= grid.len() as i64 {
        return light;
    }
    let yi = y as usize;
    let xi = x as usize;
    let dir = direction.clone() as usize;
    if light[yi][xi][dir] {
        return light;
    }
    light[yi][xi][dir] = true;
    match (direction, grid[yi][xi]) {
        (Direction::North, '.') => energize(y - 1, x, Direction::North, grid, light),
        (Direction::South, '.') => energize(y + 1, x, Direction::South, grid, light),
        (Direction::North, '|') => energize(y - 1, x, Direction::North, grid, light),
        (Direction::South, '|') => energize(y + 1, x, Direction::South, grid, light),
        (Direction::West, '.') => energize(y, x - 1, Direction::West, grid, light),
        (Direction::East, '.') => energize(y, x + 1, Direction::East, grid, light),
        (Direction::West, '-') => energize(y, x - 1, Direction::West, grid, light),
        (Direction::East, '-') => energize(y, x + 1, Direction::East, grid, light),
        (Direction::North, '/') => energize(y, x + 1, Direction::East, grid, light),
        (Direction::South, '/') => energize(y, x - 1, Direction::West, grid, light),
        (Direction::West, '/') => energize(y + 1, x, Direction::South, grid, light),
        (Direction::East, '/') => energize(y - 1, x, Direction::North, grid, light),
        (Direction::North, '\\') => energize(y, x - 1, Direction::West, grid, light),
        (Direction::South, '\\') => energize(y, x + 1, Direction::East, grid, light),
        (Direction::West, '\\') => energize(y - 1, x, Direction::North, grid, light),
        (Direction::East, '\\') => energize(y + 1, x, Direction::South, grid, light),
        (Direction::North, '-') => {
            let light2 = energize(y, x + 1, Direction::East, grid, light);
            energize(y, x - 1, Direction::West, grid, light2)
        }
        (Direction::South, '-') => {
            let light2 = energize(y, x + 1, Direction::East, grid, light);
            energize(y, x - 1, Direction::West, grid, light2)
        }
        (Direction::West, '|') => {
            let light2 = energize(y - 1, x, Direction::North, grid, light);
            energize(y + 1, x, Direction::South, grid, light2)
        }
        (Direction::East, '|') => {
            let light2 = energize(y - 1, x, Direction::North, grid, light);
            energize(y + 1, x, Direction::South, grid, light2)
        }
        _ => light,
    }
}

fn energize_count(y: i64, x: i64, direction: Direction, grid: &Vec<Vec<char>>) -> i64 {
    let mut total = 0;
    let mut light = vec![vec![vec![false; 4]; grid[0].len()]; grid.len()];
    light = energize(y, x, direction, grid, light);
    for row in light {
        for col in row.iter() {
            if col.iter().any(|&x| x) {
                total += 1;
            }
        }
    }
    total
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let grid = parse_grid(input_str);
    let total = energize_count(0, 0, Direction::East, &grid);
    println!("{}", total)
}

pub fn part2() {
    let mut largest: i64 = 0;
    let input = input();
    let input_str = input.as_str();
    let grid = parse_grid(input_str);
    for i in 0..grid[0].len() {
        largest = largest.max(energize_count(0, i as i64, Direction::South, &grid));
        largest = largest.max(energize_count(
            grid.len() as i64 - 1,
            i as i64,
            Direction::North,
            &grid,
        ));
    }
    for i in 0..grid.len() {
        largest = largest.max(energize_count(i as i64, 0, Direction::East, &grid));
        largest = largest.max(energize_count(
            i as i64,
            grid[0].len() as i64 - 1,
            Direction::West,
            &grid,
        ));
    }
    println!("{}", largest)
}

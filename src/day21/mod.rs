use std::collections::HashMap;
use std::collections::HashSet;

fn parse_grid(input_str: &str) -> (HashMap<(i64, i64), char>, (i64, i64), usize) {
    let grid: Vec<Vec<char>> = input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let mut result = HashMap::new();
    let mut start = None;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            result.insert((x as i64, y as i64), grid[y][x]);
            if grid[y][x] == 'S' {
                start = Some((x as i64, y as i64));
            }
        }
    }
    (result, start.unwrap(), grid.len())
}

fn input() -> String {
    String::from(
        r#"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "#,
    )
}

fn step(grid: &HashMap<(i64, i64), char>, previous: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut result = HashSet::new();
    for &(x, y) in previous.iter() {
        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for n in neighbors.iter() {
            match grid.get(n) {
                Some(&'.' | &'S') => {}
                _ => continue,
            }
            result.insert(*n);
        }
    }
    result
}

fn step_2(
    grid: &HashMap<(i64, i64), char>,
    step: i64,
    previous: HashMap<(i64, i64), i64>,
) -> HashMap<(i64, i64), i64> {
    let mut result = previous.clone();
    for (&(x, y), d) in previous.iter() {
        if d % 2 == step % 2 {
            continue;
        }
        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for n in neighbors.iter() {
            match grid.get(n) {
                Some(&'.' | &'S') => {}
                _ => continue,
            }
            if !result.contains_key(n) {
                result.insert(*n, step);
            }
        }
    }
    result
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let (grid, start, _) = parse_grid(input_str);
    let mut reachable = HashSet::new();
    reachable.insert(start);
    for _ in 0..64 {
        reachable = step(&grid, reachable);
    }
    println!("{}", reachable.len())
}

pub fn part2() {
    // Solved using the approach defined in
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let input = input();
    let input_str = input.as_str();
    let (grid, start, size) = parse_grid(input_str);
    let mut reachable_map = HashMap::new();
    reachable_map.insert(start, 0);
    for i in 1..size + 1 {
        reachable_map = step_2(&grid, i as i64, reachable_map);
    }
    let n = 26501365 / size;
    let rem = 26501365 % size;
    println!("{} {}", n, rem);
    let even_corners = reachable_map
        .iter()
        .filter(|&(_, &v)| v > rem as i64 && v % 2 == 0)
        .count();
    let odd_corners = reachable_map
        .iter()
        .filter(|&(_, &v)| v > rem as i64 && v % 2 == 1)
        .count();
    let odd = reachable_map.iter().filter(|&(_, &v)| v % 2 == 1).count();
    let even = reachable_map.iter().filter(|&(_, &v)| v % 2 == 0).count();
    let result = (n + 1) * (n + 1) * odd + n * n * even - (n + 1) * odd_corners + n * even_corners;
    println!("{}", result)
}

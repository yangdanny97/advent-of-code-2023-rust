use std::collections::HashSet;

fn input() -> String {
    String::from(
        r#"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
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

fn range_set(v1: i64, v2: i64) -> HashSet<i64> {
    let start = v1.min(v2) + 1;
    let end = v1.max(v2);
    (start..end).collect::<HashSet<_>>()
}

fn total_distance(expansion: i64) -> i64 {
    let input = input();
    let input_str = input.as_str();
    let grid = parse_grid(input_str);
    let mut empty_cols = (0_i64..(grid[0].len() as i64))
        .collect::<HashSet<_>>();
    let mut empty_rows = HashSet::new();
    let mut galaxies = vec![];
    for y in 0..grid.len() {
        let mut empty_row = true;
        for x in 0..grid[0].len() {
            let item = grid[y][x];
            if item == '#' {
                galaxies.push((x as i64, y as i64));
                empty_row = false;
                empty_cols.remove(&(x as i64));
            }
        }
        if empty_row {
            empty_rows.insert(y as i64);
        }
    }
    let mut total: i64 = 0;
    for i in 0..(galaxies.len() - 1) {
        for j in i + 1..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let xr = range_set(x1, x2).intersection(&empty_cols).count() as i64;
            let yr = range_set(y1, y2).intersection(&empty_rows).count() as i64;
            let dist =
                (xr * expansion) + (yr * expansion) + (x2 - x1).abs() + (y2 - y1).abs() - xr - yr;
            total += dist;
        }
    }
    total
}

pub fn part1() {
    println!("{}", total_distance(2))
}

pub fn part2() {
    println!("{}", total_distance(1000000))
}

use pathfinding::prelude::dijkstra;

fn input() -> String {
    String::from(
        r#"
        2413432311323
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
        4322674655533
    "#,
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_grid(input_str: &str) -> Vec<Vec<usize>> {
    input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, i64, Direction);

impl Pos {
    fn successors(&self, grid: &Vec<Vec<usize>>, min: i64, max: i64) -> Vec<(Pos, usize)> {
        let &Pos(x, y, count, dir) = self;
        let directions = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];
        let mut results = vec![];
        for d in directions.iter() {
            if *d == dir && count == max {
                continue;
            }
            if *d != dir && count < min {
                continue;
            }
            match (*d, dir) {
                (Direction::North, Direction::South) => {
                    continue;
                }
                (Direction::South, Direction::North) => {
                    continue;
                }
                (Direction::East, Direction::West) => {
                    continue;
                }
                (Direction::West, Direction::East) => {
                    continue;
                }
                _ => {}
            };
            if *d == Direction::North && y == 0 {
                continue;
            }
            if *d == Direction::West && x == 0 {
                continue;
            }
            if *d == Direction::South && y == grid.len() - 1 {
                continue;
            }
            if *d == Direction::East && x == grid[0].len() - 1 {
                continue;
            }
            let next_count = if *d == dir { count + 1 } else { 1 };
            let (nx, ny) = match d {
                Direction::North => (x, y - 1),
                Direction::South => (x, y + 1),
                Direction::West => (x - 1, y),
                Direction::East => (x + 1, y),
            };
            let cost = grid[ny][nx];
            results.push((Pos(nx, ny, next_count, *d), cost));
        }
        results
    }

    fn is_goal(&self, grid: &Vec<Vec<usize>>) -> bool {
        let &Pos(x, y, _, _) = self;
        x == grid[0].len() - 1 && y == grid.len() - 1
    }
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let grid = parse_grid(input_str);
    let min_dist = dijkstra(
        &Pos(0, 0, 0, Direction::East),
        |p| p.successors(&grid, 0, 3),
        |p| p.is_goal(&grid),
    )
    .unwrap()
    .1;
    println!("{}", min_dist);
}

pub fn part2() {
    let input = input();
    let input_str = input.as_str();
    let grid = parse_grid(input_str);
    let min_dist_e = dijkstra(
        &Pos(0, 0, 0, Direction::East),
        |p| p.successors(&grid, 4, 10),
        |p| p.is_goal(&grid),
    )
    .unwrap()
    .1;
    let min_dist_s = dijkstra(
        &Pos(0, 0, 0, Direction::South),
        |p| p.successors(&grid, 4, 10),
        |p| p.is_goal(&grid),
    )
    .unwrap()
    .1;
    println!("{}", min_dist_e.min(min_dist_s));
}

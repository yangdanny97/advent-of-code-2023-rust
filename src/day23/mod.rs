use std::collections::HashMap;
use std::collections::HashSet;

fn input() -> String {
    String::from(
        r#"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "#,
    )
}

fn parse_grid(input_str: &str) -> (Vec<Vec<char>>, HashMap<(i64, i64), Vec<(i64, i64)>>) {
    let grid = input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut adjacency = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut adj = vec![];
            if grid[y][x] == '#' {
                continue;
            }
            if x > 0 && (grid[y][x] == '<' || grid[y][x] == '.') && grid[y][x - 1] != '#' {
                adj.push((x as i64 - 1, y as i64));
            }
            if x < grid[0].len() - 1
                && (grid[y][x] == '>' || grid[y][x] == '.')
                && grid[y][x + 1] != '#'
            {
                adj.push((x as i64 + 1, y as i64));
            }
            if y > 0 && (grid[y][x] == '^' || grid[y][x] == '.') && grid[y - 1][x] != '#' {
                adj.push((x as i64, y as i64 - 1));
            }
            if y < grid.len() - 1
                && (grid[y][x] == 'v' || grid[y][x] == '.')
                && grid[y + 1][x] != '#'
            {
                adj.push((x as i64, y as i64 + 1));
            }
            adjacency.insert((x as i64, y as i64), adj);
        }
    }
    adjacency.insert((1, 0), vec![(1, 1)]);
    adjacency.insert((grid[0].len() as i64 - 2, grid.len() as i64 - 1), vec![]);
    (grid, adjacency)
}

fn parse_grid_2(input_str: &str) -> (Vec<Vec<char>>, HashMap<(i64, i64), Vec<(i64, i64)>>) {
    let grid = input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut adjacency = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut adj = vec![];
            if grid[y][x] == '#' {
                continue;
            }
            if x > 0 && grid[y][x - 1] != '#' {
                adj.push((x as i64 - 1, y as i64));
            }
            if x < grid[0].len() - 1 && grid[y][x + 1] != '#' {
                adj.push((x as i64 + 1, y as i64));
            }
            if y > 0 && grid[y - 1][x] != '#' {
                adj.push((x as i64, y as i64 - 1));
            }
            if y < grid.len() - 1 && grid[y + 1][x] != '#' {
                adj.push((x as i64, y as i64 + 1));
            }
            adjacency.insert((x as i64, y as i64), adj);
        }
    }
    adjacency.insert((1, 0), vec![(1, 1)]);
    adjacency.insert((grid[0].len() as i64 - 2, grid.len() as i64 - 1), vec![]);
    (grid, adjacency)
}

fn search(
    pos: (i64, i64),
    dest: (i64, i64),
    adjacency: &HashMap<(i64, i64), Vec<(i64, i64)>>,
    mut path: HashSet<(i64, i64)>,
) -> (i64, HashSet<(i64, i64)>) {
    let mut max = 0;
    if pos == dest {
        return (path.len() as i64, path);
    }
    path.insert(pos);
    let neighbors = adjacency.get(&pos).unwrap();
    for n in neighbors.iter() {
        if path.contains(n) {
            continue;
        }
        let (result, path2) = search(*n, dest, adjacency, path);
        max = max.max(result);
        path = path2;
    }
    path.remove(&pos);
    (max, path)
}

fn search_with_dist(
    curr: (i64, i64),
    dest: (i64, i64),
    adjacency: &HashMap<(i64, i64), Vec<((i64, i64), i64)>>,
    mut path: HashSet<(i64, i64)>,
    dist: i64,
) -> (i64, HashSet<(i64, i64)>) {
    let mut max = 0;
    if curr == dest {
        return (dist, path);
    }
    path.insert(curr);
    let neighbors = adjacency.get(&curr).unwrap();
    for &(n, d) in neighbors.iter() {
        if path.contains(&n) {
            continue;
        }
        let (result, path2) = search_with_dist(n, dest, adjacency, path, dist + d);
        max = max.max(result);
        path = path2;
    }
    path.remove(&curr);
    (max, path)
}

fn simplify_graph(
    mut curr: (i64, i64),
    prev_intersection: (i64, i64),
    dest: (i64, i64),
    adjacency: &HashMap<(i64, i64), Vec<(i64, i64)>>,
    mut new_adjacency: HashMap<(i64, i64), Vec<((i64, i64), i64)>>,
    mut visited: HashSet<(i64, i64)>,
    mut dist_to_prev: i64,
) -> (
    HashMap<(i64, i64), Vec<((i64, i64), i64)>>,
    HashSet<(i64, i64)>,
) {
    visited.insert(curr);
    let mut neighbors = adjacency.get(&curr).unwrap();
    while let Some(next) = neighbors.iter().find(|&n| {
        (adjacency.get(n).unwrap().len() > 2 && *n != prev_intersection) || !visited.contains(n)
    }) {
        visited.insert(*next);
        curr = *next;
        neighbors = adjacency.get(&curr).unwrap();
        dist_to_prev += 1;
        if neighbors.len() > 2 {
            break;
        }
    }
    if curr == dest {
        new_adjacency.insert(dest, vec![(prev_intersection, dist_to_prev)]);
        new_adjacency
            .entry(prev_intersection)
            .and_modify(|v| v.push((dest, dist_to_prev)));
        return (new_adjacency, visited);
    }
    if neighbors.len() > 2 {
        // curr is an intersection
        new_adjacency
            .entry(curr)
            .and_modify(|v| v.push((prev_intersection, dist_to_prev)))
            .or_insert(vec![(prev_intersection, dist_to_prev)]);
        new_adjacency
            .entry(prev_intersection)
            .and_modify(|v| v.push((curr, dist_to_prev)))
            .or_insert(vec![(curr, dist_to_prev)]);
        for n in neighbors.iter() {
            if visited.contains(n) {
                continue;
            }
            (new_adjacency, visited) =
                simplify_graph(*n, curr, dest, adjacency, new_adjacency, visited, 1);
        }
    }
    (new_adjacency, visited)
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let (grid, adjacency) = parse_grid(input_str);
    let (dist, _) = search(
        (1, 0),
        (grid[0].len() as i64 - 2, grid.len() as i64 - 1),
        &adjacency,
        HashSet::new(),
    );
    println!("{}", dist)
}

pub fn part2() {
    let input = input();
    let input_str = input.as_str();
    let (grid, adjacency) = parse_grid_2(input_str);
    let dest = (grid[0].len() as i64 - 2, grid.len() as i64 - 1);
    let mut new_adj = HashMap::new();
    new_adj.insert((1, 0), vec![]);
    let mut starting_path = HashSet::new();
    starting_path.insert((1, 0));
    let (simplified_adjacency, _) =
        simplify_graph((1, 1), (1, 0), dest, &adjacency, new_adj, starting_path, 1);
    let mut starting_path = HashSet::new();
    starting_path.insert((1, 0));
    let (dist, _) = search_with_dist((1, 0), dest, &simplified_adjacency, starting_path, 0);
    println!("{}", dist)
}

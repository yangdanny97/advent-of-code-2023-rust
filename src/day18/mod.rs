use std::collections::HashSet;

fn input() -> String {
    String::from(
        r#"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "#,
    )
}

pub fn part1() {
    // this one is just a simple flooding/search approach
    let input = input();
    let input_str = input.as_str();
    let (mut x, mut y) = (0, 0);
    let mut points = HashSet::new();
    points.insert((0, 0));

    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts = trimmed.split_whitespace().collect::<Vec<_>>();
        let n = parts[1].parse::<i64>().unwrap();
        match parts[0] {
            "U" => {
                for i in 1..=n {
                    points.insert((x, y - i));
                }
                y -= n;
            },
            "D" => {
                for i in 1..=n {
                    points.insert((x, y + i));
                }
                y += n;
            },
            "L" => {
                for i in 1..=n {
                    points.insert((x - i, y));
                }
                x -= n;
            },
            "R" => {
                for i in 1..=n {
                    points.insert((x + i, y));
                }
                x += n;
            },
            _ => {}
        }
    }
    // this assumes 1,1  is inside the loop
    let mut stack = vec![(1, 1)];
    while let Some(next) = stack.pop() {
        if points.contains(&next) {
            continue;
        }
        points.insert(next);
        stack.push((next.0 - 1, next.1));
        stack.push((next.0 + 1, next.1));
        stack.push((next.0, next.1 - 1));
        stack.push((next.0, next.1 + 1));
    }
    println!("{}", points.len())
}

pub fn part2() {
    let input = input();
    let input_str = input.as_str();
    let (mut x, mut y) = (0, 0);
    let mut points = vec![];
    points.push((0, 0));
    let mut perimeter = 0;
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts = trimmed.split_whitespace().collect::<Vec<_>>();
        let hex = &parts[2][2..8];
        // math trick to make perimeter accurately reflect the 0.5m buffer
        let n = i64::from_str_radix(&hex[0..5], 16).unwrap() * 2;
        let dir = hex.chars().last().unwrap();
        perimeter += n;
        match dir {
            '3' => {
                points.push((x, y - n));
                y -= n;
            },
            '1' => {
                points.push((x, y + n));
                y += n;
            },
            '2' => {
                points.push((x - n, y));
                x -= n;
            },
            '0' => {
                points.push((x + n, y));
                x += n;
            },
            _ => {}
        }
    }
    let mut sum = 0;
    // https://en.wikipedia.org/wiki/Shoelace_formula
    // the points array includes (0, 0) twice at the beginning & end
    for i in 0..points.len() - 2 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i+1];
        sum += (y1 + y2) * (x1 - x2);
    }
    let area = (sum / 2).abs();
    println!("{}", (area + perimeter) / 4 + 1)
}

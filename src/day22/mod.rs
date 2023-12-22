use std::collections::HashMap;
use std::collections::HashSet;

fn input() -> String {
    String::from(
        r#"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9        
    "#,
    )
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let mut bricks = vec![];
    let mut coords: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut safe = HashSet::new();
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let nums = trimmed
            .split(|c| c == '~' || c == ',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        bricks.push(((nums[0], nums[1], nums[2]), (nums[3], nums[4], nums[5])));
    }
    bricks.sort_by_cached_key(|x| x.0 .2);
    for (brick_id, &((x1, y1, z1), (x2, y2, z2))) in bricks.iter().enumerate() {
        let mut curr_z = z1.min(z2);
        let height = z1.max(z2) - curr_z + 1;
        let mut supported = HashSet::new();
        let brick: Vec<(usize, usize)> = if x1 == x2 {
            (y1..=y2).map(|y| (x1, y)).collect()
        } else {
            (x1..=x2).map(|x| (x, y1)).collect()
        };
        loop {
            let mut stop = false;
            for &(x, y) in brick.iter() {
                if let Some(&b) = coords.get(&(x, y, curr_z - 1)) {
                    stop = true;
                    supported.insert(b);
                }
            }
            if stop || curr_z == 1 {
                for &(x, y) in brick.iter() {
                    for dz in 0..height {
                        coords.insert((x, y, curr_z + dz), brick_id);
                    }
                }
                safe.insert(brick_id);
                if supported.len() == 1 {
                    safe.remove(supported.iter().next().unwrap());
                }
                break;
            }
            curr_z -= 1;
        }
    }
    println!("{}", safe.len())
}

pub fn part2() {
    let input = input();
    let input_str = input.as_str();
    let mut bricks = vec![];
    let mut coords: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut safe = HashSet::new();
    let mut supported_by: HashMap<usize, Vec<i64>> = HashMap::new();
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let nums = trimmed
            .split(|c| c == '~' || c == ',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        bricks.push(((nums[0], nums[1], nums[2]), (nums[3], nums[4], nums[5])));
    }
    bricks.sort_by_cached_key(|x| x.0 .2);
    for (brick_id, &((x1, y1, z1), (x2, y2, z2))) in bricks.iter().enumerate() {
        let mut curr_z = z1.min(z2);
        let height = z1.max(z2) - curr_z + 1;
        let mut supported = HashSet::new();
        let brick: Vec<(usize, usize)> = if x1 == x2 {
            (y1..=y2).map(|y| (x1, y)).collect()
        } else {
            (x1..=x2).map(|x| (x, y1)).collect()
        };
        loop {
            let mut stop = false;
            for &(x, y) in brick.iter() {
                if let Some(&b) = coords.get(&(x, y, curr_z - 1)) {
                    stop = true;
                    supported.insert(b);
                }
            }
            if stop || curr_z == 1 {
                for &(x, y) in brick.iter() {
                    for dz in 0..height {
                        coords.insert((x, y, curr_z + dz), brick_id);
                    }
                }
                safe.insert(brick_id);
                if supported.len() == 1 {
                    safe.remove(supported.iter().next().unwrap());
                }
                if curr_z == 1 {
                    supported_by.insert(brick_id, vec![-1]);
                } else {
                    supported_by.insert(
                        brick_id,
                        supported.iter().map(|&x| x as i64).collect::<Vec<_>>(),
                    );
                }
                break;
            }
            curr_z -= 1;
        }
    }
    let mut total = 0;
    for brick_id in 0..bricks.len() {
        if safe.contains(&brick_id) {
            continue;
        }
        let mut removed = HashSet::new();
        removed.insert(brick_id as i64);
        for brick2 in 0..bricks.len() {
            let supporters = supported_by.get(&brick2).unwrap();
            if supporters.iter().all(|x| removed.contains(x)) {
                removed.insert(brick2 as i64);
                total += 1;
            }
        }
    }
    println!("{}", total)
}

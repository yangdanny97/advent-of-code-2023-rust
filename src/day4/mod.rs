use std::collections::HashSet;

fn input() -> String {
    String::from(
        r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#,
    )
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let mut total: i64 = 0;
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts = trimmed.split(|c| c == '|' || c == ':').collect::<Vec<_>>();
        let winning: HashSet<i64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let have: HashSet<i64> = parts[2]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let matches = winning
            .intersection(&have.clone())
            .collect::<Vec<_>>()
            .len();
        if matches > 0 {
            total += 2_i64.pow((matches - 1) as u32);
        }
    }
    println!("{}", total)
}

pub fn part2() {
    let input = input();
    let input_str = input.as_str();
    let mut n_matches: Vec<usize> = vec![];
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts = trimmed.split(|c| c == '|' || c == ':').collect::<Vec<_>>();
        let winning: HashSet<i64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let have: HashSet<i64> = parts[2]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let matches = winning
            .intersection(&have.clone())
            .collect::<Vec<_>>()
            .len();
        n_matches.push(matches);
    }
    let mut instances: Vec<i64> = n_matches.iter().map(|_| 1).collect();
    for i in 0..instances.len() {
        let m = n_matches[i];
        let next = i + 1;
        let this_card_count = instances[i];
        for j in next..next+m {
            if j >= n_matches.len() {
                break;
            }
            instances[j] += this_card_count;
        }
    }
    let total: i64 = instances.iter().sum();
    println!("{}", total)
}

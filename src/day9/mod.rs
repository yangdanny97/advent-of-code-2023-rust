fn input() -> String {
    String::from(
        r#"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "#,
    )
}

fn find_next(seq: Vec<i64>) -> i64 {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    }
    let mut diffs = vec![];
    let mut curr = seq[0];
    for i in 1..seq.len() {
        diffs.push(seq[i] - curr);
        curr = seq[i];
    }
    seq[seq.len() - 1] + find_next(diffs)
}

fn find_prev(seq: Vec<i64>) -> i64 {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    }
    let mut diffs = vec![];
    let mut curr = seq[0];
    for i in 1..seq.len() {
        diffs.push(seq[i] - curr);
        curr = seq[i];
    }
    seq[0] - find_prev(diffs)
}

pub fn part1() {
    let mut total: i64 = 0;
    let input = input();
    let input_str = input.as_str();
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let nums: Vec<i64> = trimmed.split_whitespace().map(|x| x.parse().unwrap()).collect();
        total += find_next(nums);
    }
    println!("{}", total)
}

pub fn part2() {
    let mut total: i64 = 0;
    let input = input();
    let input_str = input.as_str();
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let nums: Vec<i64> = trimmed.split_whitespace().map(|x| x.parse().unwrap()).collect();
        total += find_prev(nums);
    }
    println!("{}", total)
}

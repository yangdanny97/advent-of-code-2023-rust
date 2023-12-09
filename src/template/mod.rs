fn input() -> String {
    String::from(
        r#"

    "#,
    )
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
        
    }
    println!("{}", total)
}

fn input1() -> String {
    String::from(
        r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "#,
    )
}

pub fn part1() {
    let input = input1();
    let input_str = input.as_str();
    let mut sum = 0;
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let numbers: Vec<_> = trimmed.chars().filter(|&c| c.is_numeric()).collect();
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let combined = format!("{}{}", first, last);
        let num: i64 = combined.parse().unwrap();
        sum += num;
    }
    println!("{}", sum)
}

fn input2() -> String {
    String::from(
        r#"
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    "#,
    )
}

pub fn part2() {
    let input = input2();
    let input_str = input.as_str();
    let mut sum = 0;
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let replaced = trimmed
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let numbers: Vec<_> = replaced.chars().filter(|&c| c.is_numeric()).collect();
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let combined = format!("{}{}", first, last);
        let num: i64 = combined.parse().unwrap();
        sum += num;
    }
    println!("{}", sum)
}

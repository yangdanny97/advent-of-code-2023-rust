fn input() -> String {
    String::from(
        r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#,
    )
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let mut sum = 0;
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (first_part, second_part) = trimmed.split_once(':').unwrap();
        let game_number: i64 = first_part.split(' ').collect::<Vec<_>>()[1]
            .parse()
            .unwrap();
        let mut all_valid = true;
        let contents = second_part.split(|c| c == ',' || c == ';').collect::<Vec<_>>();
        for cubes in contents {
            let (count_str, color) = cubes.trim().split_once(' ').unwrap();
            let count: i64 = count_str.parse().unwrap();
            let valid = match color {
                "red" => count <= 12,
                "green" => count <= 13,
                "blue" => count <= 14,
                _ => false,
            };
            if !valid {
                all_valid = false;
            }
        }
        if all_valid {
            sum += game_number;
        }
    }
    println!("{}", sum)
}

pub fn part2() {
    let input = input();
    let input_str = input.as_str();
    let mut total: i64 = 0;
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (_, second_part) = trimmed.split_once(':').unwrap();
        let contents = second_part.split(|c| c == ',' || c == ';').collect::<Vec<_>>();
        let mut min_red: i64 = 0;
        let mut min_blue: i64 = 0;
        let mut min_green: i64 = 0;
        for cubes in contents {
            let (count_str, color) = cubes.trim().split_once(' ').unwrap();
            let count: i64 = count_str.parse().unwrap();
            match color {
                "red" => {
                    min_red = min_red.max(count);
                }
                "green" => {
                    min_green = min_green.max(count);
                }
                "blue" => {
                    min_blue = min_blue.max(count);
                }
                _ => (),
            };
        }
        total += min_red * min_blue * min_green;
    }
    println!("{}", total)
}

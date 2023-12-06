fn input() -> (Vec<i64>, Vec<i64>) {
    let times = vec![7, 15, 30];
    let distances = vec![9, 40, 200];
    (times, distances)
}

fn input2() -> (i64, i64) {
    (71530, 940200)
}

pub fn part1() {
    let (times, distances) = input();
    let mut total = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let mut wins = 0;
        for t in 1..time {
            if (time - t) * t > distance {
                wins += 1;
            }
        }
        total *= wins;
    }
    println!("{}", total)
}

pub fn part2() {
    let (time, distance) = input2();
    let mut wins = 0;
    for t in 1..time {
        if (time - t) * t > distance {
            wins += 1;
        }
    }
    println!("{}", wins)
}

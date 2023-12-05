fn input() -> (Vec<i64>, Vec<String>) {
    // did some slight manual preprocessing
    let strings = [r#"
        50 98 2
        52 50 48
        "#,
        r#"
        0 15 37
        37 52 2
        39 0 15
        "#,
        r#"
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        "#,
        r#"
        88 18 7
        18 25 70
        "#,
        r#"
        45 77 23
        81 45 19
        68 64 13
        "#,
        r#"
        0 69 1
        1 0 69
        "#,
        r#"
        60 56 37
        56 93 4
        "#];
    let seeds: Vec<i64> = vec![79, 14, 55, 13];
    (seeds, strings.iter().map(|s| String::from(*s)).collect())
}

pub fn part1() {
    let (seeds, input) = input();
    let mut mappings: Vec<Vec<(i64, i64, i64)>> = vec![];
    for group in input {
        let mapping_str = group.as_str();
        let mut mapping: Vec<(i64, i64, i64)> = vec![];
        for line in mapping_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let row: Vec<i64> = trimmed
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            mapping.push((row[0], row[1], row[2]));
        }
        mappings.push(mapping);
    }
    let mut min_location = std::i64::MAX;
    for seed in seeds {
        let mut curr = seed;
        for mapping in &mappings {
            for &(dest, src, len) in mapping {
                if curr >= src && curr < src + len {
                    curr += dest - src;
                    break;
                }
            }
        }
        min_location = min_location.min(curr);
    }
    println!("{}", min_location)
}

pub fn part2() {
    let (seed_ranges, input) = input();
    let mut mappings: Vec<Vec<(i64, i64, i64)>> = vec![];
    for group in input {
        let mapping_str = group.as_str();
        let mut mapping: Vec<(i64, i64, i64)> = vec![];
        for line in mapping_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let row: Vec<i64> = trimmed
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            mapping.push((row[0], row[1], row[2]));
        }
        // sort mappings by src
        mapping.sort_by(|a, b| a.1.cmp(&b.1));
        mappings.push(mapping);
    }
    // model ranges as inclusive (min, max) pairs
    let mut ranges: Vec<_> = seed_ranges
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .map(|(start, len)| (start, start + len - 1))
        .collect();
    // store set of ranges at each step instead of single values
    for mapping in &mappings {
        ranges = ranges
            .iter()
            .flat_map(|&(rmin, rmax)| {
                let mut new_ranges: Vec<(i64, i64)> = vec![];
                let mut curr = rmin;
                for &(dest, src, len) in mapping {
                    let smax = src + len - 1;
                    if smax < curr {
                        continue;
                    }
                    if src > rmax || curr > rmax {
                        break;
                    }
                    // if beginning of current interval falls outside of any defined mapping
                    if src > curr {
                        new_ranges.push((curr, src - 1));
                    }
                    let delta = dest - src;
                    let new_rmin = curr.max(src) + delta;
                    let new_rmax = rmax.min(smax) + delta;
                    // add overlapping range
                    new_ranges.push((new_rmin, new_rmax));
                    curr = smax.min(rmax) + 1;
                }
                // if end of current interval falls outside of any defined mapping
                if curr < rmax {
                    new_ranges.push((curr, rmax));
                }
                new_ranges
            })
            .collect();
    }
    let min_location = ranges.iter().map(|&(rmin, _)| rmin).min().unwrap();
    println!("{}", min_location)
}

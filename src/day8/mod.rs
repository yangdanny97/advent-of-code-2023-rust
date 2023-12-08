use num::Integer;
use regex::Regex;
use std::collections::HashMap;

fn input() -> (Vec<char>, String) {
    let mapping = String::from(
        r#"
    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    "#,
    );
    let chars: Vec<_> = "LLR".chars().collect();
    (chars, mapping)
}

fn input2() -> (Vec<char>, String) {
    let mapping = String::from(
        r#"
    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    "#,
    );
    let chars: Vec<_> = "LR".chars().collect();
    (chars, mapping)
}

pub fn part1() {
    let (instrs, input_str) = input();
    let mut steps = 0;
    let mut curr = "AAA";
    let mut mapping = HashMap::new();
    let re = Regex::new(r"[A-Z]+").unwrap();
    // Use find_iter to iterate over matches
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts: Vec<_> = re.find_iter(trimmed).map(|m| m.as_str()).collect();
        mapping.insert(parts[0], (parts[1], parts[2]));
    }
    while curr != "ZZZ" {
        let instruction = instrs[steps % instrs.len()];
        let &(l, r) = mapping.get(curr).unwrap();
        match instruction {
            'L' => {
                curr = l;
            }
            'R' => {
                curr = r;
            }
            _ => {}
        }
        steps += 1;
    }
    println!("{}", steps)
}

// return the cycle length between visits to a Z-node
// assumes that only one Z-node is visited per path, and the node is only visited once per cycle
fn check_start(start: &str, instrs: Vec<char>, mapping: HashMap<&str, (&str, &str)>) -> i64 {
    let mut first_visited = 0;
    let mut curr = start;
    let mut steps = 0;
    loop {
        let instr_idx = steps % instrs.len();
        let instruction = instrs[instr_idx];
        let &(l, r) = mapping.get(curr).unwrap();
        match instruction {
            'L' => {
                curr = l;
            }
            'R' => {
                curr = r;
            }
            _ => {}
        }
        if curr.ends_with('Z') {
            if first_visited != 0 {
                return steps as i64 - first_visited;
            }
            first_visited = steps as i64;
        }
        steps += 1;
    }
}

pub fn part2() {
    let (instrs, input_str) = input2();
    let mut mapping = HashMap::new();
    let re = Regex::new(r"[0-9A-Z]+").unwrap();
    // Use find_iter to iterate over matches
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts: Vec<_> = re.find_iter(trimmed).map(|m| m.as_str()).collect();
        mapping.insert(parts[0], (parts[1], parts[2]));
    }
    let starts: Vec<_> = mapping
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|&k| k)
        .collect();
    let results: Vec<_> = starts
        .iter()
        .map(|s| check_start(s, instrs.clone(), mapping.clone()))
        .collect();
    let mut lcm = 1;
    for r in results {
        lcm = lcm.lcm(&r);
    }
    println!("{}", lcm)
}

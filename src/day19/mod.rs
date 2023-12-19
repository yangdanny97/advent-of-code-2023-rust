use regex::Regex;
use std::collections::HashMap;

fn input() -> (String, String) {
    (
        String::from(
            r#"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
    "#,
        ),
        String::from(
            r#"
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "#,
        ),
    )
}

#[derive(Debug)]
struct Workflow {
    name: String,
    steps: Vec<Step>,
    otherwise: String,
}

#[derive(Debug)]
struct Step {
    rating: char,
    op: char,
    value: i64,
    next: String,
}

fn parse_part(input: &str) -> (i64, i64, i64, i64) {
    let pattern = Regex::new(r"\d+").unwrap();
    let mut matches = pattern.find_iter(input);
    (
        matches.next().unwrap().as_str().parse().unwrap(),
        matches.next().unwrap().as_str().parse().unwrap(),
        matches.next().unwrap().as_str().parse().unwrap(),
        matches.next().unwrap().as_str().parse().unwrap(),
    )
}

fn parse_workflow(input: &str) -> Workflow {
    let parts = input[..input.len() - 1]
        .split(|c| c == '{' || c == ',')
        .collect::<Vec<_>>();
    let name = parts[0];
    let otherwise = parts[parts.len() - 1];
    let mut steps = Vec::new();
    for i in 1..parts.len() - 1 {
        let step = parts[i];
        let mut chars = step.chars();
        let rating = chars.next().unwrap();
        let op = chars.next().unwrap();
        let step_parts = step[2..].split(':').collect::<Vec<_>>();
        let value = step_parts[0].parse::<i64>().unwrap();
        steps.push(Step {
            op: op,
            rating: rating,
            value: value,
            next: String::from(step_parts[1]),
        })
    }
    Workflow {
        steps: steps,
        name: String::from(name),
        otherwise: String::from(otherwise),
    }
}

pub fn part1() {
    let mut total: i64 = 0;
    let (workflows_input, parts_input) = input();
    let mut workflows = HashMap::new();
    let workflows_str = workflows_input.as_str();
    for line in workflows_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let workflow = parse_workflow(trimmed);
        workflows.insert(workflow.name.clone(), workflow);
    }
    let parts_str = parts_input.as_str();
    let mut parts = Vec::new();
    for line in parts_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        parts.push(parse_part(trimmed));
    }
    for p in parts {
        let mut current = "in";
        loop {
            if current == "A" {
                total += p.0 + p.1 + p.2 + p.3;
                break;
            }
            if current == "R" {
                break;
            }
            let workflow = workflows.get(current).unwrap();
            let mut matched = false;
            for step in workflow.steps.iter() {
                let rating = match step.rating {
                    'x' => p.0,
                    'm' => p.1,
                    'a' => p.2,
                    's' => p.3,
                    _ => panic!("unexpected rating"),
                };
                match step.op {
                    '>' if rating > step.value => {
                        matched = true;
                        current = step.next.as_str();
                    }
                    '<' if rating < step.value => {
                        matched = true;
                        current = step.next.as_str();
                    }
                    _ => {}
                }
                if matched {
                    break;
                }
            }
            if !matched {
                current = workflow.otherwise.as_str();
            }
        }
    }
    println!("{}", total)
}

#[derive(Debug, Clone)]
struct Range {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

fn split_range(range: Range, rating: char, op: char, value: i64) -> (Option<Range>, Option<Range>) {
    let mut result1 = range.clone();
    let mut result2 = range.clone();
    match (rating, op) {
        ('x', '<') => {
            let (low, high) = range.x;
            if high < value {
                return (Some(range), None);
            }
            if low >= value {
                return (None, Some(range));
            }
            result1.x = (low, value - 1);
            result2.x = (value, high);
            return (Some(result1), Some(result2));
        }
        ('x', '>') => {
            let (low, high) = range.x;
            if low > value {
                return (Some(range), None);
            }
            if high <= value {
                return (None, Some(range));
            }
            result1.x = (value + 1, high);
            result2.x = (low, value);
            return (Some(result1), Some(result2));
        }
        ('m', '<') => {
            let (low, high) = range.m;
            if high < value {
                return (Some(range), None);
            }
            if low >= value {
                return (None, Some(range));
            }
            result1.m = (low, value - 1);
            result2.m = (value, high);
            return (Some(result1), Some(result2));
        }
        ('m', '>') => {
            let (low, high) = range.m;
            if low > value {
                return (Some(range), None);
            }
            if high <= value {
                return (None, Some(range));
            }
            result1.m = (value + 1, high);
            result2.m = (low, value);
            return (Some(result1), Some(result2));
        }
        ('a', '<') => {
            let (low, high) = range.a;
            if high < value {
                return (Some(range), None);
            }
            if low >= value {
                return (None, Some(range));
            }
            result1.a = (low, value - 1);
            result2.a = (value, high);
            return (Some(result1), Some(result2));
        }
        ('a', '>') => {
            let (low, high) = range.a;
            if low > value {
                return (Some(range), None);
            }
            if high <= value {
                return (None, Some(range));
            }
            result1.a = (value + 1, high);
            result2.a = (low, value);
            return (Some(result1), Some(result2));
        }
        ('s', '<') => {
            let (low, high) = range.s;
            if high < value {
                return (Some(range), None);
            }
            if low >= value {
                return (None, Some(range));
            }
            result1.s = (low, value - 1);
            result2.s = (value, high);
            return (Some(result1), Some(result2));
        }
        ('s', '>') => {
            let (low, high) = range.s;
            if low > value {
                return (Some(range), None);
            }
            if high <= value {
                return (None, Some(range));
            }
            result1.s = (value + 1, high);
            result2.s = (low, value);
            return (Some(result1), Some(result2));
        }
        _ => return (None, None),
    }
}

fn find_range(range: Range, current: &str, workflows: &HashMap<String, Workflow>) -> i64 {
    if current == "R" {
        return 0;
    }
    if current == "A" {
        return (range.x.1 - range.x.0 + 1)
            * (range.m.1 - range.m.0 + 1)
            * (range.a.1 - range.a.0 + 1)
            * (range.s.1 - range.s.0 + 1);
    }
    let workflow = workflows.get(current).unwrap();
    let mut curr_range = Some(range);
    let mut total = 0;
    for step in workflow.steps.iter() {
        if let Some(r) = curr_range {
            let (matching, not_matching) = split_range(r, step.rating, step.op, step.value);
            if let Some(m) = matching {
                total += find_range(m, step.next.as_str(), workflows);
            }
            curr_range = not_matching;
        }
    }
    if let Some(r) = curr_range {
        total += find_range(r, workflow.otherwise.as_str(), workflows);
    }
    return total;
}

pub fn part2() {
    let (workflows_input, _) = input();
    let mut workflows = HashMap::new();
    let workflows_str = workflows_input.as_str();
    for line in workflows_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let workflow = parse_workflow(trimmed);
        workflows.insert(workflow.name.clone(), workflow);
    }
    let total = find_range(
        Range {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        "in",
        &workflows,
    );
    println!("{}", total)
}

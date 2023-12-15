fn input() -> String {
    String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
}

fn hash(input: &str) -> i64 {
    let mut current = 0;
    let chars = input.chars().collect::<Vec<_>>();
    for char in chars {
        current += char as u8 as i64;
        current *= 17;
        current %= 256;
    }
    current
}

pub fn part1() {
    let mut total: i64 = 0;
    let input = input();
    for step in input.trim().split(',') {
        total += hash(step);
    }
    println!("{}", total)
}

pub fn part2() {
    let mut total: i64 = 0;
    let input = input();
    let mut boxes: Vec<Vec<(String, i32)>> = vec![vec![]; 256];
    for step in input.trim().split(',') {
        if step.contains('-') {
            let label = step.replace('-', "");
            let n = hash(label.as_str()) as usize;
            let new_box = boxes[n]
                .iter()
                .filter(|(l, _)| l.as_str() != label)
                .cloned()
                .collect::<Vec<_>>();
            boxes[n] = new_box;
        } else {
            let parts = step.split('=').collect::<Vec<_>>();
            let label = parts[0];
            let n = hash(label) as usize;
            let focal_length = parts[1].parse::<i32>().unwrap();
            let existing = boxes[n].iter().any(|(l, _)| l.as_str() == label);
            if existing {
                let new_box = boxes[n]
                    .iter()
                    .map(|pair| {
                        if pair.0.as_str() == label {
                            (String::from(label), focal_length)
                        } else {
                            pair.clone()
                        }
                    })
                    .collect::<Vec<_>>();
                boxes[n] = new_box;
            } else {
                boxes[n].push((String::from(label), focal_length));
            }
        }
    }
    for (box_idx, lens_box) in boxes.iter().enumerate() {
        for (slot, &(_, focal_length)) in lens_box.iter().enumerate() {
            let power = (box_idx as i64 + 1) * (slot + 1) as i64 * focal_length as i64;
            total += power;
        }
    }
    println!("{}", total)
}

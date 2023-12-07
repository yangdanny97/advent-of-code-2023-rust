use std::cmp::Ordering;
use std::collections::HashMap;

fn input() -> String {
    String::from(
        r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "#,
    )
}

fn score(card: char, joker: bool) -> i64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if joker { 1 } else { 11 },
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn cmp_hand(h1: &[char], h2: &[char], joker: bool) -> Ordering {
    for i in 0..5 {
        let diff = score(h1[i], joker) - score(h2[i], joker);
        if diff != 0 {
            return if diff > 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }
    }
    Ordering::Equal
}

fn hand_type(hand: &HashMap<char, i64>) -> i64 {
    let mut values: Vec<_> = hand.values().collect();
    values.sort_by(|a, b| b.cmp(a));
    if values.len() == 1 {
        return 7;
    }
    let &v = values[0];
    if values.len() == 2 {
        if v == 4 {
            return 6;
        }
        if v == 3 {
            return 5;
        }
    }
    if values.len() == 3 {
        if v == 3 {
            return 4;
        }
        if v == 2 {
            return 3;
        }
    }
    if values.len() == 4 {
        return 2;
    }
    1
}

pub fn part1() {
    let mut total: i64 = 0;
    let input = input();
    let input_str = input.as_str();
    let mut hands: Vec<(HashMap<char, i64>, Vec<char>, i64)> = vec![];
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts: Vec<_> = trimmed.split_whitespace().collect();
        let hand_chars: Vec<_> = parts[0].chars().collect();
        let hand_bid = parts[1].parse::<i64>().unwrap();
        // count number of cards of each type
        let mut hand_map: HashMap<char, i64> = HashMap::new();
        for c in hand_chars.clone() {
            let count = hand_map.entry(c).or_insert(0);
            *count += 1;
        }
        let hand = (hand_map, hand_chars, hand_bid);
        hands.push(hand);
    }
    hands.sort_by(|(m1, h1, _), (m2, h2, _)| {
        hand_type(m1)
            .cmp(&hand_type(m2))
            .then_with(|| cmp_hand(h1, h2, false))
    });
    for (i, (_, _, bid)) in hands.iter().enumerate() {
        total += (i as i64 + 1) * bid;
    }
    println!("{}", total)
}

fn hand_type2(hand: &HashMap<char, i64>) -> i64 {
    let j = hand.get(&'J').map(|&x| x).unwrap_or(0);
    // handle case where all cards are J
    if j == 5 {
        return 7;
    }
    // remove J from mapping
    let mut entries: Vec<_> = hand.iter().filter(|(&c, _)| c != 'J').collect();
    entries.sort_by(|(_, &a), (_, &b)| b.cmp(&a));
    // add J count to strongest card
    let mut modified_hand = hand.clone();
    modified_hand.remove(&'J');
    let val = modified_hand.get_mut(entries[0].0).unwrap();
    *val += j;

    return hand_type(&modified_hand);
}

pub fn part2() {
    let mut total: i64 = 0;
    let input = input();
    let input_str = input.as_str();
    let mut hands: Vec<(HashMap<char, i64>, Vec<char>, i64)> = vec![];
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts: Vec<_> = trimmed.split_whitespace().collect();
        let hand_chars: Vec<_> = parts[0].chars().collect();
        let hand_bid = parts[1].parse::<i64>().unwrap();
        // count number of cards of each type
        let mut hand_map: HashMap<char, i64> = HashMap::new();
        for c in hand_chars.clone() {
            let count = hand_map.entry(c).or_insert(0);
            *count += 1;
        }
        let hand = (hand_map, hand_chars, hand_bid);
        hands.push(hand);
    }
    hands.sort_by(|(m1, h1, _), (m2, h2, _)| {
        hand_type2(m1)
            .cmp(&hand_type2(m2))
            .then_with(|| cmp_hand(h1, h2, true))
    });
    for (i, (_, _, bid)) in hands.iter().enumerate() {
        total += (i as i64 + 1) * bid;
    }
    println!("{}", total)
}

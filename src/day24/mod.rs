use regex::Regex;

fn input() -> (String, f64, f64) {
    (
        String::from(
            r#"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "#,
        ),
        7.0,
        27.0,
    )
}

fn intersection(x1: f64, y1: f64, dx1: f64, dy1: f64, x2: f64, y2: f64, dx2: f64, dy2: f64) -> Option<(f64, f64)> {
    // parallel lines
    if (dx1 == 0.0 && dx2 == 0.0) || (dy1 == 0.0 && dy2 == 0.0) {
        return None
    }
    if (dx2 > dx1 && dx2 / dx1 == dy2 / dy1) || (dx1 > dx2 && dx1 / dx2 == dy1 / dy2) {
        return None
    }
    // vertical lines
    if dx1 == 0.0 {
        let dt = (x1 - x2) / dx2;
        if dt < 0.0 {
            return None
        }
        return Some((x1, y2 + dt * dy2))
    }
    if dx2 == 0.0 {
        let dt= (x2 - x1) / dx1;
        if dt < 0.0 {
            return None
        }
        return Some((x2, y1 + dt * dy1))
    }
    // regular lines
    let mut slope1 = dy1/dx1;
    let mut slope2 = dy2/dx2;
    let mut intercept1 = (-x1) / dx1 * dy1 + y1;
    let mut intercept2 = (-x2) / dx2 * dy2 + y2;
    if intercept1 == intercept2 {
        return Some((x1, intercept1))
    }
    if intercept1 < intercept2 {
        (intercept2, intercept1) = (intercept1, intercept2);
        (slope2, slope1) = (slope1, slope2);
    }
    if slope1 > 0.0 && slope2 < 0.0 {
        return None
    }
    let delta = intercept1 - intercept2;
    let ix = delta / (-slope1 + slope2);
    let iy = intercept2 + delta * (slope2 / (-slope1 + slope2));
    if ix > x1 && dx1 < 0.0 || ix < x1 && dx1 > 0.0 {
        return None
    }
    if ix > x2 && dx2 < 0.0 || ix < x2 && dx2 > 0.0 {
        return None
    }
    return Some((ix, iy))
}

pub fn part1() {
    let mut total: i64 = 0;
    let (input, min_bound, max_bound) = input();
    let input_str = input.as_str();
    let mut stones: Vec<(f64, f64, f64, f64, f64, f64)> = vec![];
    let pattern = Regex::new(r"[-\d]+").unwrap();
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let matches = pattern
            .find_iter(trimmed)
            .map(|m| m.as_str().parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        stones.push((
            matches[0], matches[1], matches[2], matches[3], matches[4], matches[5],
        ))
    }
    for (i, &(x1, y1, _, dx1, dy1, _)) in stones.iter().enumerate() {
        for &(x2, y2, _, dx2, dy2, _) in stones.iter().skip(i + 1) {
            if let Some((x, y)) = intersection(x1, y1, dx1, dy1, x2, y2, dx2, dy2) {
                if x >= min_bound && x <= max_bound && y >= min_bound && y <= max_bound {
                    total += 1;
                }
            }
        }
    }
    println!("{}", total)
}

// this prints the constraints for a SMT solver (like Z3) in SMT-LIB2 format
// run the solver, and sum the resulting values for ix, iy, and iz to get the answer
pub fn part2() {
    let (input, _, _) = input();
    let input_str = input.as_str();
    let mut stones: Vec<(i64, i64, i64, i64, i64, i64)> = vec![];
    let pattern = Regex::new(r"[-\d]+").unwrap();
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let matches = pattern
            .find_iter(trimmed)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        stones.push((
            matches[0], matches[1], matches[2], matches[3], matches[4], matches[5],
        ))
    }
    println!(r#"
(declare-const ix Int)
(declare-const iy Int)
(declare-const iz Int)
(declare-const dx Int)
(declare-const dy Int)
(declare-const dz Int)
(declare-const t1 Int)
(declare-const t2 Int)
(declare-const t3 Int)
(declare-const t4 Int)
(declare-const t5 Int)
    "#);
    for (i, &(x_n, y_n, z_n, dx_n, dy_n, dz_n)) in stones.iter().take(5).enumerate() {
        println!("(assert (>= t{} 0))", i + 1);
        println!("(assert (= (+ (* t{} dx) ix) (+ (* {} t{}) {})))", i + 1, dx_n, i + 1, x_n);
        println!("(assert (= (+ (* t{} dy) iy) (+ (* {} t{}) {})))", i + 1, dy_n, i + 1, y_n);
        println!("(assert (= (+ (* t{} dz) iz) (+ (* {} t{}) {})))", i + 1, dz_n, i + 1, z_n);
    };
    println!(r#"
(check-sat)
(get-model)
    "#)
}

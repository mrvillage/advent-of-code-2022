fn calc(x: &str, y: &str) -> i32 {
    let mut score = 0;
    if x == y {
        score += 3;
    } else if (x == "A" && y == "B") || (x == "B" && y == "C") || (x == "C" && y == "A") {
        score += 6;
    }
    if y == "A" {
        score += 1;
    } else if y == "B" {
        score += 2;
    } else {
        score += 3;
    }
    score
}

fn main() {
    let input = include_str!("input.txt");
    let mut score = 0;
    for line in input.lines() {
        let (x, y) = line.split_at(1);
        let mut y = y.strip_prefix(' ').unwrap();
        if y == "X" {
            y = "A";
        } else if y == "Y" {
            y = "B";
        } else {
            y = "C";
        }
        score += calc(x, y);
    }

    println!("Part One: {}", score);

    let mut score = 0;
    for line in input.lines() {
        let (x, y) = line.split_at(1);
        let y = y.strip_prefix(' ').unwrap();
        if y == "X" {
            if x == "A" {
                score += calc(x, "C");
            } else if x == "B" {
                score += calc(x, "A");
            } else {
                score += calc(x, "B");
            }
        } else if y == "Y" {
            score += calc(x, x);
        } else if x == "A" {
            score += calc(x, "B");
        } else if x == "B" {
            score += calc(x, "C");
        } else {
            score += calc(x, "A");
        }
    }

    println!("Part Two: {}", score);
}

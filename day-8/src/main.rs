fn main() {
    let input = include_str!("input.txt");

    let trees = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = trees.len();
    let width = trees[0].len();

    let mut count = (height + width - 2) * 2;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let h = trees[y][x];
            let mut not_visible = 0;
            // check left
            for i in 0..x {
                if trees[y][i] >= h {
                    not_visible += 1;
                    break;
                }
            }
            // check right
            for i in x + 1..width {
                if trees[y][i] >= h {
                    not_visible += 1;
                    break;
                }
            }
            // check top
            for i in trees.iter().take(y) {
                if i[x] >= h {
                    not_visible += 1;
                    break;
                }
            }
            // check bottom
            for i in trees.iter().take(height).skip(y + 1) {
                if i[x] >= h {
                    not_visible += 1;
                    break;
                }
            }
            if not_visible < 4 {
                count += 1;
            }
        }
    }

    println!("Part One: {}", count);

    let mut max_score = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let h = trees[y][x];
            let mut score = 1;
            // check left
            let mut m = 1;
            for i in (0..x).rev() {
                m = x - i;
                if trees[y][i] >= h {
                    break;
                }
            }
            score *= m;
            // check right
            let mut m = 1;
            for i in x + 1..width {
                m = i - x;
                if trees[y][i] >= h {
                    break;
                }
            }
            score *= m;
            // check top
            let mut m = 1;
            for (i, t) in trees.iter().enumerate().take(y).rev() {
                m = y - i;
                if t[x] >= h {
                    break;
                }
            }
            score *= m;
            // check bottom
            let mut m = 1;
            for (i, t) in trees.iter().enumerate().take(height).skip(y + 1) {
                m = i - y;
                if t[x] >= h {
                    break;
                }
            }
            score *= m;
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part Two: {}", max_score);
}

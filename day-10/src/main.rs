use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Op {
    No,
    Add(i32),
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Op::No)
        } else if s.starts_with("addx") {
            let num = s[5..].parse::<i32>().unwrap();
            Ok(Op::Add(num))
        } else {
            Ok(Op::No)
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut queue = VecDeque::from_iter(input.lines().map(|l| l.parse::<Op>().unwrap()));

    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut op = None;
    let mut strength = 0;
    let mut crt: Vec<Vec<char>> = vec![vec![], vec![], vec![], vec![], vec![], vec![]];
    loop {
        if cycle == 239 {
            break;
        }
        let line_index = (cycle as f64 / 40.0).floor() as usize;
        let pixel_index = cycle % 40;
        if pixel_index == x - 1 || pixel_index == x || pixel_index == x + 1 {
            crt[line_index].push('#');
        } else {
            crt[line_index].push('.');
        }
        cycle += 1;
        if cycle == 20 || (cycle - 20 > 0 && (cycle - 20) % 40 == 0) {
            strength += x * cycle;
        }
        if let Some(o) = op {
            match o {
                Op::No => {
                    op = None;
                },
                Op::Add(num) => {
                    x += num;
                    op = None;
                },
            }
        } else {
            op = queue.pop_front();
            match op {
                Some(Op::No) => {
                    op = None;
                },
                Some(Op::Add(_)) => {
                    continue;
                },
                None => {
                    break;
                },
            }
        }
    }
    println!("Part One: {}", strength);

    println!("Part Two:");
    for line in crt {
        println!(
            "{}",
            line.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        );
    }
}

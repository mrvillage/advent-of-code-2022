use std::collections::HashSet;

fn move_knot(head: (i32, i32), tail: &mut (i32, i32)) {
    if head.0 - tail.0 >= 2 && head.1 == tail.1 {
        *tail = (head.0 - 1, head.1);
    } else if tail.0 - head.0 >= 2 && head.1 == tail.1 {
        *tail = (head.0 + 1, head.1);
    } else if head.1 - tail.1 >= 2 && head.0 == tail.0 {
        *tail = (head.0, head.1 - 1);
    } else if tail.1 - head.1 >= 2 && head.0 == tail.0 {
        *tail = (head.0, head.1 + 1);
    } else if (head.0 - tail.0).abs() + (head.1 - tail.1).abs() == 3 {
        // move the distance 1 side
        if (head.0 - tail.0).abs() == 1 {
            *tail = (head.0, tail.1);
        } else {
            *tail = (tail.0, head.1);
        }
        // move tail diagonally in the direction of head
        if head.0 - tail.0 >= 2 {
            *tail = (head.0 - 1, tail.1);
        } else if tail.0 - head.0 >= 2 {
            *tail = (head.0 + 1, tail.1);
        } else if head.1 - tail.1 >= 2 {
            *tail = (tail.0, head.1 - 1);
        } else if tail.1 - head.1 >= 2 {
            *tail = (tail.0, head.1 + 1);
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(head);

    for i in input.lines() {
        let dir = i.chars().next().unwrap();
        let num = i[2..].parse::<i32>().unwrap();
        for _ in 0..num {
            match dir {
                'U' => {
                    head.1 += 1;
                    move_knot(head, &mut tail);
                },
                'D' => {
                    head.1 -= 1;
                    move_knot(head, &mut tail);
                },
                'L' => {
                    head.0 -= 1;
                    move_knot(head, &mut tail);
                },
                'R' => {
                    head.0 += 1;
                    move_knot(head, &mut tail);
                },
                _ => panic!("Invalid direction"),
            }
            visited.insert(tail);
        }
    }

    println!("Part One: {}", visited.len());

    let mut knots = vec![];
    for _ in 0..10 {
        knots.push((0, 0));
    }
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(knots[9]);

    for i in input.lines() {
        let dir = i.chars().next().unwrap();
        let num = i[2..].parse::<i32>().unwrap();
        for _ in 0..num {
            match dir {
                'U' => {
                    knots[0].1 += 1;
                    for i in 1..10 {
                        move_knot(knots[i - 1], &mut knots[i]);
                    }
                },
                'D' => {
                    knots[0].1 -= 1;
                    for i in 1..10 {
                        move_knot(knots[i - 1], &mut knots[i]);
                    }
                },
                'L' => {
                    knots[0].0 -= 1;
                    for i in 1..10 {
                        move_knot(knots[i - 1], &mut knots[i]);
                    }
                },
                'R' => {
                    knots[0].0 += 1;
                    for i in 1..10 {
                        move_knot(knots[i - 1], &mut knots[i]);
                    }
                },
                _ => panic!("Invalid direction"),
            }
            // println!("{:?}", i);
            // println!("{:?}", knots);
            visited.insert(knots[9]);
        }
    }

    println!("Part Two: {}", visited.len());
}

use std::collections::HashSet;

fn move_knot(head: (i32, i32), tail_ref: &mut (i32, i32)) {
    let mut tail = *tail_ref;
    // only off by one in a single directory
    if head.0 == tail.0 {
        if head.1 - tail.1 == 2 {
            tail.1 = head.1 - 1;
        } else if head.1 - tail.1 == -2 {
            tail.1 = head.1 + 1;
        }
    } else if head.1 == tail.1 {
        if head.0 - tail.0 == 2 {
            tail.0 = head.0 - 1;
        } else if head.0 - tail.0 == -2 {
            tail.0 = head.0 + 1;
        }
    // up two, right 1
    } else if head.0 - tail.0 == 1 && head.1 - tail.1 == 2 {
        tail.0 += 1;
        tail.1 += 1;
    // up two, left 1
    } else if head.0 - tail.0 == -1 && head.1 - tail.1 == 2 {
        tail.0 -= 1;
        tail.1 += 1;
    // down two, right 1
    } else if head.0 - tail.0 == 1 && head.1 - tail.1 == -2 {
        tail.0 += 1;
        tail.1 -= 1;
    // down two, left 1
    } else if head.0 - tail.0 == -1 && head.1 - tail.1 == -2 {
        tail.0 -= 1;
        tail.1 -= 1;
    // right 2, up 1
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == 1 {
        tail.0 += 1;
        tail.1 += 1;
    // right 2, down 1
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == -1 {
        tail.0 += 1;
        tail.1 -= 1;
    // left 2, up 1
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == 1 {
        tail.0 -= 1;
        tail.1 += 1;
    // left 2, down 1
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == -1 {
        tail.0 -= 1;
        tail.1 -= 1;
    // up 2, right 2
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == 2 {
        tail.0 += 1;
        tail.1 += 1;
    // up 2, left 2
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == 2 {
        tail.0 -= 1;
        tail.1 += 1;
    // down 2, right 2
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == -2 {
        tail.0 += 1;
        tail.1 -= 1;
    // down 2, left 2
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == -2 {
        tail.0 -= 1;
        tail.1 -= 1;
    }
    // println!(
    //     "head: {:?}, before tail: {:?}, after tail: {:?}",
    //     head, tail_ref, tail
    // );
    *tail_ref = tail;
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
    // return;

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

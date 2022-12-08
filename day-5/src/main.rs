use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");

    let mut crates_input = vec![];
    let mut actions = vec![];
    let mut doing_actions = false;
    for i in input.lines() {
        if i.is_empty() {
            doing_actions = true;
            continue;
        }
        if doing_actions {
            actions.push(i);
        } else {
            crates_input.push(i);
        }
    }
    let actions = actions
        .iter()
        .map(|s| {
            let split = s.split(' ').collect::<Vec<&str>>();
            (
                split[1].parse::<usize>().unwrap(),
                split[3].parse::<usize>().unwrap() - 1,
                split[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect::<Vec<_>>();
    let mut stacks = (1..=crates_input.last().unwrap().replace(' ', "").len())
        .into_iter()
        .map(|_| VecDeque::new())
        .collect::<Vec<VecDeque<String>>>();
    for i in crates_input {
        if !i.contains('[') {
            continue;
        }
        for j in (0..i.len()).step_by(4) {
            let value = i[j..j + 3]
                .replace(' ', "")
                .replace('[', "")
                .replace(']', "");
            if value.is_empty() {
                continue;
            }
            stacks[j / 4].push_back(value);
        }
    }

    let mut stacks_part_one = stacks.clone();
    for (count, from, to) in actions.clone() {
        for _ in 0..count {
            let value = stacks_part_one[from].pop_front().unwrap();
            stacks_part_one[to].push_front(value);
        }
    }

    println!(
        "Part One: {}",
        stacks_part_one
            .iter()
            .map(|i| i.iter().next().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
    );

    let mut stacks_part_two = stacks.clone();
    for (count, from, to) in actions {
        let mut to_move: VecDeque<String> = VecDeque::new();
        for _ in 0..count {
            let value = stacks_part_two[from].pop_front().unwrap();
            to_move.push_back(value);
        }
        for _ in 0..count {
            let value = to_move.pop_back().unwrap();
            stacks_part_two[to].push_front(value);
        }
    }

    println!(
        "Part Two: {}",
        stacks_part_two
            .iter()
            .map(|i| i.iter().next().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

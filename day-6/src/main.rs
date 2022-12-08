use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    for i in 3..input.len() {
        let set: HashSet<char> = HashSet::from_iter(input[i - 3..=i].chars());
        if set.len() == 4 {
            println!("Part One: {}", i + 1);
            break;
        }
    }

    for i in 13..input.len() {
        let set: HashSet<char> = HashSet::from_iter(input[i - 13..=i].chars());
        if set.len() == 14 {
            println!("Part Two: {}", i + 1);
            break;
        }
    }
}

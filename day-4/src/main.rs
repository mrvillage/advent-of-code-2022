use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let pairs = input
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            let left = split
                .next()
                .unwrap()
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let right = split
                .next()
                .unwrap()
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (left, right)
        })
        .collect::<Vec<_>>();
    let count: u32 = pairs
        .iter()
        .map(|(left, right)| {
            if left[0] <= right[0] && left[1] >= right[1] {
                return 1;
            };
            if right[0] <= left[0] && right[1] >= left[1] {
                return 1;
            };
            0
        })
        .sum();
    println!("Part One: {}", count);

    let overlap: u32 = pairs
        .iter()
        .map(|(left, right)| {
            let left: HashSet<u32> = HashSet::from_iter(left[0]..=left[1]);
            let right: HashSet<u32> = HashSet::from_iter(right[0]..=right[1]);
            let intersection = left.intersection(&right).count();
            if intersection > 0 {
                return 1;
            };
            0
        })
        .sum();
    println!("Part Two: {}", overlap);
}

use std::collections::{hash_set::Intersection, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let v = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut priority = 0;
    for line in input.lines() {
        let (l, r) = line.split_at(line.len() / 2);
        let l: HashSet<char> = HashSet::from_iter(l.chars());
        let r: HashSet<char> = HashSet::from_iter(r.chars());
        let int: Intersection<'_, char, _> = l.intersection(&r);
        let int: HashSet<&char> = HashSet::from_iter::<
            std::collections::hash_set::Intersection<'_, char, _>,
        >(<std::collections::hash_set::Intersection<
            '_,
            char,
            _,
        > as std::iter::IntoIterator>::into_iter(
            int
        ));
        priority += v
            .iter()
            .position(|&x| x == **int.iter().next().unwrap())
            .unwrap()
            + 1;
    }

    println!("Part One: {}", priority);

    let mut priority = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    for i in (0..lines.len()).step_by(3) {
        let one: HashSet<char> = HashSet::from_iter(lines[i].chars());
        let two: HashSet<char> = HashSet::from_iter(lines[i + 1].chars());
        let three: HashSet<char> = HashSet::from_iter(lines[i + 2].chars());
        let int: Intersection<'_, char, _> = one.intersection(&two);
        let int: HashSet<&char> = HashSet::from_iter::<
            std::collections::hash_set::Intersection<'_, char, _>,
        >(<std::collections::hash_set::Intersection<
            '_,
            char,
            _,
        > as std::iter::IntoIterator>::into_iter(
            int
        ));
        let int: HashSet<char> = int.iter().map(|e| **e).collect::<HashSet<char>>();
        let int = int.intersection(&three);
        let int: HashSet<&char> = HashSet::from_iter::<
            std::collections::hash_set::Intersection<'_, char, _>,
        >(<std::collections::hash_set::Intersection<
            '_,
            char,
            _,
        > as std::iter::IntoIterator>::into_iter(
            int
        ));
        priority += v
            .iter()
            .position(|&x| x == **int.iter().next().unwrap())
            .unwrap()
            + 1;
    }

    println!("Part Two: {}", priority);
}

fn main() {
    let input = include_str!("input.txt");
    let mut v: Vec<i32> = vec![];
    let mut next = 0;
    for line in input.lines() {
        if line.is_empty() {
            v.push(next);
            next = 0;
        } else {
            next += line.parse::<i32>().unwrap();
        }
    }
    v.sort();
    v.reverse();

    println!("Part One: {}", v[0]);

    println!("Part Two: {}", v[0] + v[1] + v[2])
}

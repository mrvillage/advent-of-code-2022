use std::str::{Chars, FromStr};

#[derive(Debug, Eq)]
enum Value {
    Num(u8),
    Arr(Vec<Value>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        matches!(in_right_order(self, other), Order::Equal)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match in_right_order(self, other) {
            Order::Lower => Some(std::cmp::Ordering::Less),
            Order::Higher => Some(std::cmp::Ordering::Greater),
            Order::Equal => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match in_right_order(self, other) {
            Order::Lower => std::cmp::Ordering::Less,
            Order::Higher => std::cmp::Ordering::Greater,
            Order::Equal => std::cmp::Ordering::Equal,
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Num(n) => n.to_string(),
            Value::Arr(v) => format!(
                "[{}]",
                v.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

impl FromStr for Value {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();
        let mut chars = s.chars();
        chars.next();
        while let Some(c) = chars.next() {
            if c == '[' {
                values.push(Value::parse_array(s, &mut chars));
            } else if c == ']' {
                return Ok(Value::Arr(values));
            } else if c == ',' {
                continue;
            } else {
                let (c, value) = Value::parse_num(c, &mut chars);
                values.push(value);
                if c == ']' {
                    return Ok(Value::Arr(values));
                }
            }
        }
        Ok(Value::Arr(values))
    }
}

impl Value {
    fn parse_array(s: &str, chars: &mut Chars<'_>) -> Value {
        let mut values = Vec::new();
        while let Some(c) = chars.next() {
            if c == '[' {
                values.push(Value::parse_array(s, chars));
            } else if c == ']' {
                return Value::Arr(values);
            } else if c == ',' {
                continue;
            } else {
                let (c, value) = Value::parse_num(c, chars);
                values.push(value);
                if c == ']' {
                    return Value::Arr(values);
                }
            }
        }
        panic!("Unexpected end of input");
    }

    fn parse_num(c: char, chars: &mut Chars) -> (char, Value) {
        let mut num = String::new();
        num.push(c);
        for c in chars.by_ref() {
            if c == ',' || c == ']' {
                if num.is_empty() && c == ']' {
                    return (c, Value::Arr(vec![]));
                }
                return (c, Value::Num(num.parse().unwrap()));
            } else {
                num.push(c);
            }
        }
        panic!("Unexpected end of input");
    }
}

#[derive(Debug)]
enum Order {
    Lower,
    Higher,
    Equal,
}

fn in_right_order(a: &Value, b: &Value) -> Order {
    match (a, b) {
        (Value::Num(a), Value::Num(b)) => match a.cmp(b) {
            std::cmp::Ordering::Less => Order::Lower,
            std::cmp::Ordering::Equal => Order::Equal,
            std::cmp::Ordering::Greater => Order::Higher,
        },
        (Value::Arr(a), Value::Arr(b)) => {
            for (a, b) in a.iter().zip(b.iter()) {
                match in_right_order(a, b) {
                    Order::Lower => return Order::Lower,
                    Order::Higher => return Order::Higher,
                    Order::Equal => continue,
                }
            }
            match a.len().cmp(&b.len()) {
                std::cmp::Ordering::Less => Order::Lower,
                std::cmp::Ordering::Equal => Order::Equal,
                std::cmp::Ordering::Greater => Order::Higher,
            }
        },
        (a, Value::Num(b)) => in_right_order(a, &Value::Arr(vec![Value::Num(*b)])),
        (Value::Num(a), b) => in_right_order(&Value::Arr(vec![Value::Num(*a)]), b),
    }
}

fn main() {
    let input = include_str!("input.txt");
    let packets = input
        .split("\r\n\r\n")
        .map(|s| {
            let mut lines = s.lines();
            let a = lines.next().unwrap().parse().unwrap();
            let b = lines.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();
    let part_one: usize = packets
        .iter()
        .enumerate()
        .map(|(index, (a, b))| match in_right_order(a, b) {
            Order::Lower => index + 1,
            Order::Higher => 0,
            Order::Equal => panic!("Shouldn't happen"),
        })
        .sum();
    println!("Part One: {}", part_one);

    let mut packets = packets
        .into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();
    packets.push(Value::Arr(vec![Value::Arr(vec![Value::Num(2)])]));
    packets.push(Value::Arr(vec![Value::Arr(vec![Value::Num(6)])]));
    packets.sort();

    let part_two_2 = packets
        .iter()
        .enumerate()
        .find(|(_, p)| p.to_string() == "[[2]]")
        .unwrap()
        .0;
    let part_two_6 = packets
        .iter()
        .enumerate()
        .find(|(_, p)| p.to_string() == "[[6]]")
        .unwrap()
        .0;

    println!("Part Two: {}", (part_two_2 + 1) * (part_two_6 + 1));
}

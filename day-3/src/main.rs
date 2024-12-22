#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part1(file.to_string());
    println!("{}", result);
}

fn part1(mut s: String) -> usize {
    let mut total = 0;
    while let Some(res) = parse_next(s.clone()) {
        if let (Some(number), _) = res {
            total += number;
        }
        s = res.1;
    }
    total
}

type ParseString<R> = (Option<R>, String);

fn next_char_is(s: &str, c: char) -> bool {
    match s.chars().next() {
        Some(other_c) => other_c == c,
        _ => false,
    }
}

fn parse_next(mut s: String) -> Option<ParseString<usize>> {
    let i = s.find("mul(");
    if let Some(i) = i {
        s = remove_up_to(&s, i + 4);
    } else {
        return None;
    }

    let parse = parse_number(s);
    s = parse.1;
    if let None = parse.0 {
        return Some((None, s));
    }
    let first = parse.0.unwrap();


    if !next_char_is(&s, ',') {
        return Some((None, s))
    }
    s = remove_up_to(&s, 1);

    let parse = parse_number(s);
    s = parse.1;
    if let None = parse.0 {
        return Some((None, s));
    }
    let second = parse.0.unwrap();

    if !next_char_is(&s, ')') {
        return Some((None, s))
    }
    s = remove_up_to(&s, 1);

    Some((Some(first * second), s))
}


fn parse_number(mut s: String) -> ParseString<usize> {
    let numbers: String = s.chars()
        .take_while(|c| c.to_string().parse::<usize>().is_ok())
        .collect();
    if numbers.chars().next().is_none() {
        return (None, s)
    }
    s = remove_up_to(&s, numbers.len());

    let number = numbers.parse::<usize>().expect("The individual numbers parsed so this should too");

    (Some(number), s)
}

fn remove_up_to(s: &String, i: usize) -> String {
    s.chars().skip(i).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part1(input.to_string());
        assert_eq!(161, result);
    }
}

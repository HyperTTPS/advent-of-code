#![allow(dead_code)]

fn main() {
    let file = include_str!("../data/part1-input.txt");
    let result = part1(file);
    println!("{}", result);
}

fn part1(reports: &str) -> i32 {
    reports.lines()
        .map(line_is_valid)
        .map(|b| if b {1} else {0})
        .sum()
}

fn line_is_valid(line: &str) -> bool {
    let mut line = line.split_whitespace().map(|s| s.parse::<i32>().expect("Assume all inputs are numbers"));

    let first = line.next().expect("Assume all lines have at least one number");
    match line.next() {
        None => true,
        Some(second) => {
            if !differs(first, second) {
                return false;
            }
            let is_incr = is_increasing(first, second);
            let mut prev = second;
            for term in line {
                if !differs(prev, term) {
                    return false;
                }
                if is_increasing(prev, term) != is_incr {
                    return false;
                }
                prev = term;
            }
            true
        }
    }
}

fn differs(first: i32, second: i32) -> bool {
    let diff = (first - second).abs();
    diff >= 1 && diff <= 3
}

fn is_increasing(first: i32, second: i32) -> bool {
    first - second >= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = part1(input);
        assert_eq!(2, result);
        return Ok(())
    }
}

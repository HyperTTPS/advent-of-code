use std::collections::{HashMap, HashSet};

fn main() {
    let file = include_str!("../data/input.txt");
    let part1_result = part1(&file.to_string());
    println!("Part1: {}", part1_result);
}

fn part1(s: &str) -> usize {
    let (rules, updates) = get_rules_and_updates(s);
    todo!()
}

fn get_rules_and_updates(s: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut split = s.split("\n\n");
    let rules = get_rules(split.next().unwrap());
    let updates = get_updates(split.next().unwrap());
    (rules, updates)
}

fn get_rules(s: &str) -> HashMap<usize, Vec<usize>> {
    let individual_rules = s.split("\n").map(|line| line.split("|")).map(|mut split| {
        (
            split.next().unwrap().parse::<usize>().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
        )
    });
    let mut rules = HashMap::new();
    for (key, value) in individual_rules {
        rules.entry(key).or_insert(Vec::new()).push(value);
    }
    rules
}

fn get_updates(s: &str) -> Vec<Vec<usize>> {
    s.split("\n")
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        include_str!("../data/test-input.txt").to_string()
    }

    #[test]
    fn test_part1() {
        let input = get_test_input();
        let result = part1(&input);
        assert_eq!(143, result);
    }
}

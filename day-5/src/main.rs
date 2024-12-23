use std::collections::HashMap;

fn main() {
    let file = include_str!("../data/input.txt");
    let part1_result = part1(&file.to_string());
    println!("Part1: {}", part1_result);

    let part2_result = part2(&file.to_string());
    println!("Part2: {}", part2_result);
}

fn part1(s: &str) -> usize {
    let (rules, updates) = get_rules_and_updates(s);
    updates
        .into_iter()
        .filter(|update| update_satisfies_rules(&update, &rules))
        .map(|update| get_middle_element(&update))
        .sum()
}

fn part2(s: &str) -> usize {
    let (rules, updates) = get_rules_and_updates(s);
    updates
        .iter()
        .filter(|update| !update_satisfies_rules(&update, &rules))
        .map(|update| order_update(&update, &rules))
        .map(|update| get_middle_element(&update))
        .sum()
}

fn order_update(update: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut old = update.clone();
    let mut new = apply_order_update(old.clone(), rules);
    while old.iter().zip(&new).any(|(x, y)| *x != *y) {
        old = new;
        new = apply_order_update(old.clone(), rules);
    }
    new
}

fn apply_order_update<'a>(
    mut update: Vec<usize>,
    rules: &HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    'outer: for i in 0..update.len() {
        let num = update[i];
        if let Some(num_rules) = rules.get(&num) {
            for j in 0..i {
                let other = update[j];
                if num_rules.contains(&other) {
                    update[i] = other;
                    update[j] = num;
                    continue 'outer;
                }
            }
        }
    }
    update
}

fn get_middle_element(v: &Vec<usize>) -> usize {
    *v.get(v.len() / 2).unwrap()
}

fn update_satisfies_rules(update: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    update.iter().enumerate().all(|(i, num)| {
        rules.get(&num).map_or(true, |num_rules| {
            update
                .iter()
                .take(i)
                .all(|other| !num_rules.contains(&other))
        })
    })
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
        .filter(|line| *line != "")
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

    #[test]
    fn test_part2() {
        let input = get_test_input();
        let result = part2(&input);
        assert_eq!(123, result);
    }
}

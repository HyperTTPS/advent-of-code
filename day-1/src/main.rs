use std::iter::zip;

fn main() {
    let file = include_str!("../data/input.txt");
    let part1_result = part1(&file.to_string());
    println!("Part1: {}", part1_result);

    let part2_result = part2(&file.to_string());
    println!("Part2: {}", part2_result);
}

fn part1(s: &str) -> usize {
    let (list1, list2) = str_to_lists_sorted(&s);
    zip(list1, list2)
        .map(|(x, y)| if x < y { y - x } else { x - y })
        .sum()
}

fn part2(s: &str) -> usize {
    let (list1, list2) = str_to_lists(&s);
    list1
        .into_iter()
        .map(|x| (x, list2.iter().filter(|y| x == **y).count()))
        .map(|(x, count)| x * count)
        .sum()
}

fn str_to_lists_sorted(s: &str) -> (Vec<usize>, Vec<usize>) {
    let (mut list1, mut list2) = str_to_lists(&s);
    list1.sort();
    list2.sort();
    (list1, list2)
}

fn str_to_lists(s: &str) -> (Vec<usize>, Vec<usize>) {
    let lists: (Vec<usize>, Vec<usize>) = s
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| x.split("   "))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .unzip();
    lists
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string()
    }

    #[test]
    fn test_str_to_lists() {
        let (list1, list2) = str_to_lists(&get_test_input());
        assert_eq!(list1, [3, 4, 2, 1, 3, 3]);
        assert_eq!(list2, [4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_part1() {
        let input = get_test_input();
        let result = part1(&input);
        assert_eq!(11, result);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input();
        let result = part2(&input);
        assert_eq!(31, result);
    }
}

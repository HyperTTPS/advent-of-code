use std::iter::zip;

fn main() {
    let file = include_str!("../data/input.txt");
    let part1_result = part1(&file.to_string());
    println!("Part1: {}", part1_result);

    let part2_result = part2(&file.to_string());
    println!("Part2: {}", part2_result);
}

fn part1(s: &str) -> usize {
    let input = str_to_vec(s);
    let size = (input.len(), input.get(0).unwrap().len());
    let mut xmas_sum: usize = 0;
    for i in 0..size.0 {
        for j in 0..size.1 {
            xmas_sum += xmas_count_on_loc(&input, (i, j));
        }
    }
    xmas_sum
}

fn part2(s: &str) -> usize {
    let input = str_to_vec(s);
    let size = (input.len(), input.get(0).unwrap().len());
    (0..size.0)
        .flat_map(|i| (0..size.1).map(move |j| (i, j)))
        .filter(|loc| cross_mas_count_on_loc(&input, *loc))
        .count()
}

fn cross_mas_count_on_loc(input: &Vec<Vec<char>>, loc: (usize, usize)) -> bool {
    let input_size = (input.len(), input.get(0).unwrap().len());
    if loc.0 == 0 || loc.1 == 0 || loc.0 == input_size.0 - 1 || loc.1 == input_size.1 - 1 {
        return false;
    }

    if letter_is_at(input, 'A', loc) {
        if ms_is_at(input, (loc.0 - 1, loc.1 - 1), (loc.0 + 1, loc.1 + 1)) {
            if ms_is_at(input, (loc.0 - 1, loc.1 + 1), (loc.0 + 1, loc.1 - 1)) {
                return true;
            }
        }
    }
    false
}

fn ms_is_at(input: &Vec<Vec<char>>, loc1: (usize, usize), loc2: (usize, usize)) -> bool {
    (letter_is_at(input, 'M', loc1) && letter_is_at(input, 'S', loc2))
        || (letter_is_at(input, 'M', loc2) && letter_is_at(input, 'S', loc1))
}

fn xmas_count_on_loc(input: &Vec<Vec<char>>, loc: (usize, usize)) -> usize {
    if !letter_is_at(input, 'X', loc) {
        return 0;
    }

    const CHECKS: [[(isize, isize); 3]; 8] = [
        [(1, 0), (2, 0), (3, 0)],
        [(-1, 0), (-2, 0), (-3, 0)],
        [(0, 1), (0, 2), (0, 3)],
        [(0, -1), (0, -2), (0, -3)],
        [(1, 1), (2, 2), (3, 3)],
        [(-1, 1), (-2, 2), (-3, 3)],
        [(-1, -1), (-2, -2), (-3, -3)],
        [(1, -1), (2, -2), (3, -3)],
    ];

    CHECKS
        .iter()
        .filter(|check| make_check_on_loc(&input, loc, **check))
        .count()
}

fn make_check_on_loc(
    input: &Vec<Vec<char>>,
    loc: (usize, usize),
    check: [(isize, isize); 3],
) -> bool {
    let (i, j): (isize, isize) = (loc.0.try_into().unwrap(), loc.1.try_into().unwrap());
    let char_locs: Vec<(usize, usize)> = check
        .iter()
        .map(|(x, y)| (i + *x, j + *y))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x.try_into().unwrap(), y.try_into().unwrap()))
        .collect();

    if char_locs.len() != check.len() {
        return false;
    }

    zip(['M', 'A', 'S'], char_locs).all(|(letter, loc)| letter_is_at(input, letter, loc))
}

fn letter_is_at(input: &Vec<Vec<char>>, letter: char, loc: (usize, usize)) -> bool {
    if let Some(line) = input.get(loc.0) {
        if let Some(c) = line.get(loc.1) {
            return letter == *c;
        }
    }
    false
}

fn str_to_vec(s: &str) -> Vec<Vec<char>> {
    s.split("\n").map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string()
    }

    #[test]
    fn test_part1() {
        let input = get_test_input();
        let result = part1(&input);
        assert_eq!(18, result);
    }

    #[test]
    fn test_part1_simple() {
        let input = "XMAS";
        let result = part1(&input);
        assert_eq!(1, result);
    }

    #[test]
    fn test_part1_simple_backwards() {
        let input = "SAMX";
        let result = part1(&input);
        assert_eq!(1, result);
    }

    #[test]
    fn test_part1_simple_2d() {
        let input = "XMAS
XMAS
XMAS
XMAS";
        let result = part1(&input);
        assert_eq!(6, result);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input();
        let result = part2(&input);
        assert_eq!(9, result);
    }
}

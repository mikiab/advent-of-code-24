use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn locations(input: &str) -> impl Iterator<Item = (u32, u32)> + use<'_> {
    input
        .lines()
        .filter_map(|line| line.split_once(char::is_whitespace))
        .map(|(l, r)| {
            (
                l.trim().parse::<u32>().unwrap(),
                r.trim().parse::<u32>().unwrap(),
            )
        })
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left_locations = Vec::with_capacity(1024);
    let mut right_locations = Vec::with_capacity(1024);

    for (left, right) in locations(input) {
        left_locations.push(left);
        right_locations.push(right);
    }

    left_locations.sort_unstable();
    right_locations.sort_unstable();

    left_locations
        .iter()
        .zip(right_locations.iter())
        .fold(0, |distance, (&left, &right)| {
            distance + left.abs_diff(right)
        })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut left_locations = Vec::with_capacity(1024);
    let mut right_locations = HashMap::new();

    for (left, right) in locations(input) {
        left_locations.push(left);
        right_locations.entry(right)
                .and_modify(|value| *value += 1)
                .or_insert(1);
    }

    left_locations
        .iter()
        .map(|l| *right_locations.get(l).unwrap_or(&0) * l)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        assert_eq!(part2(input), 31);
    }
}

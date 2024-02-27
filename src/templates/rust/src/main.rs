fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<&str>) -> u32 {
    let _ = input;
    0
}

fn part2(input: &Vec<&str>) -> u32 {
    let _ = input;
    0
}

#[cfg(test)]
mod tests {
    struct TestCase<T, U> {
        input: T,
        expected: U,
    }

    use super::*;

    #[test]
    fn test_day1() {
        let test_cases = vec![TestCase {
            input: vec![""],
            expected: 0,
        }];

        for test_case in test_cases {
            let result = part1(&test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_day2() {
        let test_cases = vec![TestCase {
            input: vec![""],
            expected: 0,
        }];

        for test_case in test_cases {
            let result = part2(&test_case.input);
            assert_eq!(result, test_case.expected);
        }
    }
}

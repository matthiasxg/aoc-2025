advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i64> {
    let mut sum: i64 = 0;
    let ranges = input.split(",");
    for range in ranges {
        if let Some((start_string, end_string)) = range.split_once('-') {
            let start: i64 = start_string.trim().parse().unwrap();
            let end: i64 = end_string.trim().parse().unwrap();

            for number in start..=end {
                let number_string = number.to_string();
                if number_string.len() % 2 == 0 {
                    let half = number_string.len() / 2;
                    let (a, b) = number_string.split_at(half);
                    if a == b {
                        sum += number;
                    }
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut sum: i64 = 0;

    for range in input.split(',') {
        let (start_str, end_str) = range.split_once('-')?;
        let start: i64 = start_str.trim().parse().unwrap();
        let end: i64 = end_str.trim().parse().unwrap();

        for number in start..=end {
            let number_string = number.to_string();
            let number_string_length = number_string.len();

            for pattern_length in 1..=number_string_length / 2 {
                if number_string_length % pattern_length != 0 {
                    continue;
                }

                let pattern = &number_string[..pattern_length];
                let repeats = number_string_length / pattern_length;

                if number_string == pattern.repeat(repeats) {
                    sum += number;
                    break;
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

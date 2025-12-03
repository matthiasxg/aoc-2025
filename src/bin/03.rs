advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_joltage: u64 = 0;

    for line in input.lines() {
        let mut first_digit: Option<u64> = None;
        let mut first_digit_index: usize = 0;
        let mut second_digit: Option<u64> = None;

        for number in (1..10).rev() {
            if first_digit != None {
                break;
            }
            for line_index in 0..line.len() - 1 {
                if &line[line_index..line_index + 1] == number.to_string() {
                    first_digit = Some(number);
                    first_digit_index = line_index + 1;
                    break;
                }
            }
        }

        for number in (1..10).rev() {
            if second_digit != None {
                break;
            }
            for line_index in first_digit_index..line.len() {
                if &line[line_index..line_index + 1] == number.to_string() {
                    second_digit = Some(number);
                    break;
                }
            }
        }

        total_joltage += first_digit.unwrap() * 10 + second_digit.unwrap();
    }

    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_joltage: u64 = 0;

    for line in input.lines() {
        let mut digit: Option<u64> = None;
        let mut digit_index: usize = 0;
        const ACTIVE_BATTERIES: u32 = 12;
        let mut remaining_batteries: usize = ACTIVE_BATTERIES as usize;

        for battery_index in 0..ACTIVE_BATTERIES {
            for number in (1..10).rev() {
                if digit != None {
                    break;
                }
                for line_index in digit_index..=line.len() - remaining_batteries {
                    if &line[line_index..line_index + 1] == number.to_string() {
                        digit = Some(number);
                        digit_index = line_index + 1;
                        break;
                    }
                }
            }

            let multiplier = 10_u64.pow(ACTIVE_BATTERIES - 1 - battery_index);
            total_joltage += digit.unwrap() * multiplier;
            remaining_batteries -= 1;
            digit = None;
        }
    }

    Some(total_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

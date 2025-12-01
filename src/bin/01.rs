advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    const MODULO: i32 = 100;
    const START: i32 = 50;

    let mut current: i32 = START;
    let mut pin: i32 = 0;

    for line in input.lines() {
        let left_or_right = &line[0..1];
        let amount_string = &line[1..];
        let amount: i32 = amount_string.parse().expect("Invalid number");

        match left_or_right {
            "L" => current -= amount,
            "R" => current += amount,
            _ => panic!("Invalid direction {}", left_or_right),
        }

        current = current.rem_euclid(MODULO);

        if current == 0 {
            pin += 1;
        }
    }
    Some(pin)
}

pub fn part_two(input: &str) -> Option<i32> {
    const MODULO: i32 = 100;
    const START: i32 = 50;

    let mut current: i32 = START;
    let mut pin: i32 = 0;

    for line in input.lines() {
        let left_or_right = &line[0..1];
        let amount_string = &line[1..];
        let mut amount: i32 = amount_string.parse().expect("Invalid number");

        while amount > MODULO {
            pin += 1;
            amount -= MODULO;
        }

        match left_or_right {
            "L" => {
                if current > 0 && (current - amount) <= 0 {
                    pin += 1;
                }
                current = (current - amount).rem_euclid(MODULO);
            }
            "R" => {
                if current + amount >= MODULO {
                    pin += 1;
                }
                current = (current + amount).rem_euclid(MODULO);
            }
            _ => panic!("Unexpected direction {}", left_or_right),
        };
    }
    Some(pin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

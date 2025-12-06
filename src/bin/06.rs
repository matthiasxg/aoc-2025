advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let mut matrix: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<u64> = vec![];
        let first_char = line.chars().next().unwrap();
        if first_char == '+' || first_char == '*' {
            for operator in line.split_ascii_whitespace() {
                operators.push(operator.parse().unwrap());
            }
            break;
        }
        for number_string in line.split_ascii_whitespace() {
            row.push(number_string.parse().unwrap());
        }
        matrix.push(row);
    }

    let rows = matrix.len();
    let columns = matrix[0].len();

    for column in 0..columns {
        let operator = operators[column];
        let mut column_result: u64 = matrix[0][column];
        for row in 1..rows {
            if operator == '+' {
                column_result += matrix[row][column];
            } else if operator == '*' {
                column_result *= matrix[row][column];
            }
        }
        result += column_result;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    let mut matrix: Vec<Vec<String>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    let mut paddings: Vec<u64> = Vec::new();

    let symbols = input.lines().last();
    let mut counter = 0;
    for char in symbols.unwrap().chars() {
        if char != ' ' {
            operators.push(char);

            if counter != 0 {
                paddings.push(counter);
                counter = 0;
            }
        }
        counter += 1;
    }
    paddings.push(counter + 1);

    for line in input.lines() {
        let mut row: Vec<String> = vec![];
        let first_char = line.chars().next().unwrap();
        if first_char == '+' || first_char == '*' {
            break;
        }
        let mut index = 0;
        for &padding in &paddings {
            let padding = padding as usize;
            row.push(line[index..index + padding - 1].to_string());
            index += padding;
        }
        matrix.push(row);
    }

    let rows = matrix.len();
    let columns = matrix[0].len();

    for column in 0..columns {
        let operator = operators[column];
        let padding = (paddings[column] - 1) as usize;

        let mut numbers: Vec<String> = vec![String::new(); padding];

        for row in 0..rows {
            let string = &matrix[row][column];
            let mut counter = 0;
            for char in string.chars() {
                counter += 1;
                if char == ' ' {
                    continue;
                }
                numbers[counter - 1].push(char);
            }
        }

        let mut column_result: u64 = numbers[0].parse().unwrap();
        for index in 1..numbers.len() {
            if operator == '+' {
                column_result += numbers[index].parse::<u64>().unwrap();
            } else if operator == '*' {
                column_result *= numbers[index].parse::<u64>().unwrap();
            }
        }
        result += column_result;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}

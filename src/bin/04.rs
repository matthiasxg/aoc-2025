use std::vec;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut forklift: Vec<Vec<bool>> = vec![];

    for line in input.lines() {
        let mut row: Vec<bool> = vec![];
        for char in line.chars() {
            if char == '.' {
                row.push(false);
            } else {
                row.push(true);
            }
        }
        forklift.push(row);
    }

    let mut accessable: u64 = 0;
    let width: isize = forklift.len() as isize;
    let height: isize = forklift[0].len() as isize;

    for x in 0..width {
        for y in 0..height {
            let mut adjacent = 0;

            if !forklift[x as usize][y as usize] {
                continue;
            };

            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i == x && j == y {
                        continue;
                    }
                    if i >= 0 && i < width && j >= 0 && j < height {
                        if forklift[i as usize][j as usize] {
                            adjacent += 1;
                        }
                    }
                }
            }

            if adjacent < 4 {
                accessable += 1;
            }
        }
    }
    Some(accessable)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut forklift: Vec<Vec<bool>> = vec![];

    for line in input.lines() {
        let mut row: Vec<bool> = vec![];
        for char in line.chars() {
            if char == '.' {
                row.push(false);
            } else {
                row.push(true);
            }
        }
        forklift.push(row);
    }

    let mut sum_accessable: u64 = 0;
    let width: isize = forklift.len() as isize;
    let height: isize = forklift[0].len() as isize;

    loop {
        let mut accessable: u64 = 0;
        let mut to_remove: Vec<(usize, usize)> = vec![];

        for x in 0..width {
            for y in 0..height {
                let mut adjacent = 0;

                if !forklift[x as usize][y as usize] {
                    continue;
                };

                for i in x - 1..=x + 1 {
                    for j in y - 1..=y + 1 {
                        if i == x && j == y {
                            continue;
                        }
                        if i >= 0 && i < width && j >= 0 && j < height {
                            if forklift[i as usize][j as usize] {
                                adjacent += 1;
                            }
                        }
                    }
                }

                if adjacent < 4 {
                    accessable += 1;
                    to_remove.push((x as usize, y as usize));
                }
            }
        }
        if accessable == 0 {
            break;
        }
        sum_accessable += accessable;

        for remove in to_remove {
            forklift[remove.0][remove.1] = false;
        }
    }
    Some(sum_accessable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

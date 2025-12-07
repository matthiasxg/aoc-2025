use std::{collections::HashSet, vec};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let columns = grid[0].len();
    let mut total_splits = 0;

    let mut beams: HashSet<usize> = HashSet::new();
    let start_col = grid[0].iter().position(|&c| c == 'S');
    beams.insert(start_col.unwrap());

    for row in 1..rows {
        let mut next_beams: HashSet<usize> = HashSet::new();
        for &beam in &beams {
            let char = grid[row][beam];
            match char {
                '^' => {
                    total_splits += 1;
                    if beam > 0 {
                        next_beams.insert(beam - 1);
                    }
                    if beam + 1 < columns {
                        next_beams.insert(beam + 1);
                    }
                }
                _ => {
                    next_beams.insert(beam);
                }
            }
        }
        beams = next_beams;
    }

    Some(total_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let columns = grid[0].len();

    let mut beams_count: Vec<u64> = vec![0; columns];
    let start_col = grid[0].iter().position(|&c| c == 'S');
    beams_count[start_col.unwrap()] += 1;

    for row in 1..rows {
        let mut next_beams_count: Vec<u64> = vec![0; columns];

        for column in 0..columns {
            let count = beams_count[column];
            if count == 0 {
                continue;
            }

            let char = grid[row][column];
            match char {
                '^' => {
                    if column > 0 {
                        next_beams_count[column - 1] += count;
                    }
                    if column + 1 < columns {
                        next_beams_count[column + 1] += count;
                    }
                }
                _ => {
                    next_beams_count[column] += count;
                }
            }
        }
        beams_count = next_beams_count;
    }

    return Some(beams_count.iter().sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

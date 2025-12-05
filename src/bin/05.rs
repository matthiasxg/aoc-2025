advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut fresh_ranges: Vec<(usize, usize)> = Vec::new();
    let mut fresh_count = 0;
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let start_index: usize = start.parse().unwrap();
        let end_index: usize = end.parse().unwrap();
        fresh_ranges.push((start_index, end_index));
    }

    for line in lines {
        let ingredient_id: usize = line.parse().unwrap();
        for range in &fresh_ranges {
            if ingredient_id >= range.0 && ingredient_id <= range.1 {
                fresh_count += 1;
                break;
            }
        }
    }

    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut fresh_ranges: Vec<(usize, usize)> = Vec::new();
    let mut fresh_count: u64 = 0;
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let start_index: usize = start.parse().unwrap();
        let end_index: usize = end.parse().unwrap();
        fresh_ranges.push((start_index, end_index));
    }

    fresh_ranges.sort();

    let mut unique_fresh_ranges: Vec<(usize, usize)> = Vec::new();

    for range in fresh_ranges {
        let start = range.0;
        let end = range.1;
        let mut found = false;

        for unique_range in unique_fresh_ranges.iter_mut() {
            let unqiue_start = unique_range.0;
            let unqiue_end = unique_range.1;

            if start >= unqiue_start && start <= unqiue_end {
                let new_end = unqiue_end.max(end);
                unique_range.1 = new_end;
                found = true;
            }
        }

        if !found {
            unique_fresh_ranges.push(range);
        }
    }

    for unique_range in unique_fresh_ranges {
        let unqiue_start = unique_range.0;
        let unqiue_end = unique_range.1;

        fresh_count += (unqiue_end - unqiue_start + 1) as u64
    }

    Some(fresh_count)
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
        assert_eq!(result, Some(14));
    }
}

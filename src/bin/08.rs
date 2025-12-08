use std::collections::HashMap;

advent_of_code::solution!(8);

pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    const ITERATIONS: usize = 10;

    let boxes: Vec<Point3D> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<f64>().unwrap();
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            let z = parts.next().unwrap().parse::<f64>().unwrap();
            Point3D::new(x, y, z)
        })
        .collect();

    let mut distances: Vec<(usize, usize, f64)> = Vec::new();
    for (i, source_box) in boxes.iter().enumerate() {
        for (j, target_box) in boxes.iter().enumerate().skip(i + 1) {
            distances.push((i, j, source_box.distance(target_box)));
        }
    }

    // sort by distance
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<usize> = (0..boxes.len()).collect();
    for index in 0..ITERATIONS {
        let source = distances[index].0;
        let target = distances[index].1;

        let source_circuit = circuits[source];
        let target_circuit = circuits[target];

        if source_circuit != target_circuit {
            for circuit in circuits.iter_mut() {
                if *circuit == target_circuit {
                    *circuit = source_circuit;
                }
            }
        }
    }

    let mut counts = HashMap::new();
    for circuit in circuits {
        *counts.entry(circuit).or_insert(0) += 1;
    }

    let mut counts: Vec<u64> = counts.values().cloned().collect();
    counts.sort_by(|a, b| b.cmp(a));

    Some(counts.iter().take(3).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes: Vec<Point3D> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<f64>().unwrap();
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            let z = parts.next().unwrap().parse::<f64>().unwrap();
            Point3D::new(x, y, z)
        })
        .collect();

    let mut distances: Vec<(usize, usize, f64)> = Vec::new();
    for (i, source_box) in boxes.iter().enumerate() {
        for (j, target_box) in boxes.iter().enumerate().skip(i + 1) {
            distances.push((i, j, source_box.distance(target_box)));
        }
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<usize> = (0..boxes.len()).collect();
    let mut remaining_circuits = circuits.len();

    for index in 0..distances.len() {
        let source = distances[index].0;
        let target = distances[index].1;

        let source_circuit = circuits[source];
        let target_circuit = circuits[target];

        if source_circuit != target_circuit {
            for circuit in circuits.iter_mut() {
                if *circuit == target_circuit {
                    *circuit = source_circuit;
                }
            }

            remaining_circuits -= 1;

            if remaining_circuits == 1 {
                let result = boxes[source].x * boxes[target].x;
                return Some(result as u64);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

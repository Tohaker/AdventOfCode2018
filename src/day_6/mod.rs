use crate::common;
use std::cmp::Ordering;
use std::collections::HashMap;

fn calculate_distance(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn calculate_map_bounds(map: &[(i32, i32)]) -> (i32, i32) {
    let x_max = *map
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(x, _)| x)
        .unwrap();
    let y_max = *map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, y)| y)
        .unwrap();

    (x_max, y_max)
}

fn determine_closest_points(input: &[(i32, i32)]) -> HashMap<usize, Vec<(i32, i32)>> {
    let (x_max, y_max) = calculate_map_bounds(input);

    let mut result: HashMap<usize, Vec<(i32, i32)>> = HashMap::new();

    for x in 0..=x_max {
        for y in 0..=y_max {
            let mut lowest_distance = 1000;
            let mut input_index = 0;
            let mut all_equal = false;

            for (i, c) in input.iter().enumerate() {
                let current_distance = calculate_distance(c, &(x, y));
                match current_distance.cmp(&lowest_distance) {
                    Ordering::Equal => all_equal = true,
                    Ordering::Greater => continue,
                    Ordering::Less => {
                        all_equal = false;
                        input_index = i;
                        lowest_distance = current_distance;
                    }
                }
            }

            if all_equal {
                continue;
            } else {
                result
                    .entry(input_index)
                    .or_insert_with(Vec::new)
                    .push((x, y))
            }
        }
    }

    result
}

fn remove_edge_locations(
    map: &[(i32, i32)],
    input: HashMap<usize, Vec<(i32, i32)>>,
) -> HashMap<usize, Vec<(i32, i32)>> {
    let (x_max, y_max) = calculate_map_bounds(map);
    let mut result = input.clone();

    for (coord, point_list) in input.iter() {
        for point in point_list.iter() {
            if point.0 == 0 || point.1 == 0 || point.0 == x_max || point.1 == y_max {
                result.remove_entry(coord);
                break;
            }
        }
    }

    result
}

fn parse_point(point: &str) -> (i32, i32) {
    let vals: Vec<&str> = point.split(", ").collect();
    let x: i32 = vals[0].parse().unwrap();
    let y: i32 = vals[1].parse().unwrap();

    (x, y)
}

fn sum_distances(point: &(i32, i32), map: &[(i32, i32)]) -> i32 {
    map.iter()
        .fold(0, |acc, c| acc + calculate_distance(c, point))
}

fn determine_region(map: &[(i32, i32)], max_distance: i32) -> Vec<(i32, i32)> {
    let (x_max, y_max) = calculate_map_bounds(map);

    let mut result: Vec<(i32, i32)> = Vec::new();

    for x in 0..=x_max {
        for y in 0..=y_max {
            let dist = sum_distances(&(x, y), map);

            if dist < max_distance {
                result.push((x, y));
            }
        }
    }

    result
}

pub fn part_1() {
    let filename = "./inputs/day_6/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let points: Vec<(i32, i32)> = lines
            .map(|l| l.expect("Could not parse line"))
            .map(|l| parse_point(&l))
            .collect();

        let closest_points = determine_closest_points(&points);
        let internal_areas = remove_edge_locations(&points, closest_points);
        let largest_area = internal_areas
            .iter()
            .max_by(|a, b| a.1.len().cmp(&b.1.len()))
            .map(|(_, v)| v.len())
            .unwrap();

        println!("Day 6 - Part 1: {}", largest_area);
    }
}

pub fn part_2() {
    let filename = "./inputs/day_6/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let points: Vec<(i32, i32)> = lines
            .map(|l| l.expect("Could not parse line"))
            .map(|l| parse_point(&l))
            .collect();

        let region_size = determine_region(&points, 10000).len();

        println!("Day 6 - Part 2: {}", region_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_manhatten_distance() {
        let point_1 = (1, 1);
        let point_2 = (5, 0);

        assert_eq!(5, calculate_distance(&point_1, &point_2));

        let point_1 = (5, 5);

        assert_eq!(5, calculate_distance(&point_1, &point_2));
    }

    #[test]
    fn should_determine_closest_points() {
        let input = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];

        let result = determine_closest_points(&input);

        assert_eq!(9, result.get(&3).unwrap().len());
        assert_eq!(17, result.get(&4).unwrap().len());
    }

    #[test]
    fn should_remove_edge_locations() {
        let input = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
        let points = determine_closest_points(&input);

        let result = remove_edge_locations(&input, points);

        assert!(!result.contains_key(&0));
        assert!(!result.contains_key(&1));
        assert!(!result.contains_key(&2));
        assert!(result.contains_key(&3));
        assert!(result.contains_key(&4));
        assert!(!result.contains_key(&5));
    }

    #[test]
    fn should_read_line_to_point() {
        let input = "46, 246";

        assert_eq!((46, 246), parse_point(input));
    }

    #[test]
    fn should_sum_distances() {
        let point = (4, 3);
        let map = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];

        assert_eq!(30, sum_distances(&point, &map));
    }

    #[test]
    fn should_find_points_within_region() {
        let map = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
        let maximum = 32;

        assert_eq!(16, determine_region(&map, maximum).len());
    }
}

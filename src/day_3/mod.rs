use crate::common;
use rectangle::Rectangle;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
mod rectangle;

fn map_rectangles(input: Vec<String>) -> (HashMap<(u32, u32), u32>, u32) {
    let mut result = HashMap::new();
    let mut all_ids = HashSet::new();
    let mut claimed_points = HashMap::new();
    let mut intersected_ids = HashSet::new();

    let rectangles: Vec<Rectangle> = input.iter().map(|s| Rectangle::new(s)).collect();

    // Each rectangle can cover a series of points, we just collect these points and count
    // how many times they're in our map.
    for r in rectangles.iter() {
        // Collect all the IDs
        all_ids.insert(r.id);

        for x in r.x1..r.x1 + r.width {
            for y in r.y1..r.y1 + r.height {
                *result.entry((x, y)).or_insert(0) += 1;
                // Check if this point has been claimed, and if it has add both ids to our list of intersections.
                match claimed_points.entry((x, y)) {
                    Vacant(entry) => {
                        entry.insert(r.id);
                    }
                    Occupied(entry) => {
                        intersected_ids.insert(*entry.get());
                        intersected_ids.insert(r.id);
                    }
                }
            }
        }
    }

    let lone_rect = all_ids.difference(&intersected_ids).next().unwrap();

    (result, *lone_rect)
}

fn count_overlap(input: HashMap<(u32, u32), u32>) -> usize {
    input.values().filter(|v| **v > 1).count()
}

pub fn part_1() {
    let filename = "./inputs/day_3/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let rectangles = lines.map(|l| l.expect("Could not parse line")).collect();
        let (result, _) = map_rectangles(rectangles);
        println!("Day 3 - Part 1: {}", count_overlap(result));
    }
}

pub fn part_2() {
    let filename = "./inputs/day_3/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let rectangles = lines.map(|l| l.expect("Could not parse line")).collect();
        let (_, difference) = map_rectangles(rectangles);
        println!("Day 3 - Part 2: #{}", difference);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_covered_points_to_map() {
        let recs = vec![
            "#1 @ 1,3: 4x4".to_string(),
            "#2 @ 3,1: 4x4".to_string(),
            "#3 @ 5,5: 2x2".to_string(),
        ];
        let (result, _) = map_rectangles(recs);

        assert_eq!(32, result.len());
        assert_eq!(&2, result.get(&(3, 3)).unwrap());
        assert_eq!(&2, result.get(&(3, 4)).unwrap());
        assert_eq!(&2, result.get(&(4, 3)).unwrap());
        assert_eq!(&2, result.get(&(4, 4)).unwrap());
    }

    #[test]
    fn should_return_list_of_lone_rectangles() {
        let recs = vec![
            "#1 @ 1,3: 4x4".to_string(),
            "#2 @ 3,1: 4x4".to_string(),
            "#3 @ 5,5: 2x2".to_string(),
        ];
        let (_, difference) = map_rectangles(recs);

        assert_eq!(3, difference);
    }

    #[test]
    fn should_count_overlapping_points() {
        let recs = vec![
            "#1 @ 1,3: 4x4".to_string(),
            "#2 @ 3,1: 4x4".to_string(),
            "#3 @ 5,5: 2x2".to_string(),
        ];
        let (input, _) = map_rectangles(recs);

        assert_eq!(4, count_overlap(input));
    }
}

use crate::common;
use rectangle::Rectangle;
use std::collections::HashMap;
mod rectangle;

fn map_rectangles(input: Vec<String>) -> HashMap<(u32, u32), u32> {
    let mut result = HashMap::new();
    let rectangles: Vec<Rectangle> = input.iter().map(|s| Rectangle::new(s)).collect();

    // Each rectangle can cover a series of points, we just collect these points and count
    // how many times they're in our map.
    for r in rectangles.iter() {
        for x in r.x1..r.x1 + r.width {
            for y in r.y1..r.y1 + r.height {
                *result.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    result
}

fn count_overlap(input: HashMap<(u32, u32), u32>) -> u32 {
    let mut result = 0;

    for value in input.values() {
        if *value > 1 {
            result += 1;
        }
    }

    result
}

pub fn part_1() {
    let filename = "./inputs/day_3/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let rectangles = lines.map(|l| l.expect("Could not parse line")).collect();
        println!(
            "Day 3 - Part 1: {}",
            count_overlap(map_rectangles(rectangles))
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_covered_points_to_map() {
        let input1 = "#1 @ 1,3: 4x4".to_string();
        let input2 = "#2 @ 3,1: 4x4".to_string();
        let input3 = "#3 @ 5,5: 2x2".to_string();

        let input = vec![input1, input2, input3];
        let result = map_rectangles(input);

        assert_eq!(32, result.len());
        assert_eq!(&2, result.get(&(3, 3)).unwrap());
        assert_eq!(&2, result.get(&(3, 4)).unwrap());
        assert_eq!(&2, result.get(&(4, 3)).unwrap());
        assert_eq!(&2, result.get(&(4, 4)).unwrap());
    }

    #[test]
    fn should_count_overlapping_points() {
        let recs = vec![
            "#1 @ 1,3: 4x4".to_string(),
            "#2 @ 3,1: 4x4".to_string(),
            "#3 @ 5,5: 2x2".to_string(),
        ];
        let input = map_rectangles(recs);

        assert_eq!(4, count_overlap(input));
    }
}

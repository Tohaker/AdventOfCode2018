#[derive(PartialEq, Debug)]
pub struct Box {
    pub has_two: bool,
    pub has_three: bool,
}

impl Box {
    pub fn count(&self) -> (i32, i32) {
        let mut twos = 0;
        let mut threes = 0;

        if self.has_two {
            twos = 1;
        }

        if self.has_three {
            threes = 1;
        }

        (twos, threes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_values() {
        let box1 = Box {
            has_two: false,
            has_three: false,
        };
        let result1 = (0, 0);
        assert_eq!(result1, box1.count());

        let box2 = Box {
            has_two: true,
            has_three: false,
        };
        let result2 = (1, 0);
        assert_eq!(result2, box2.count());

        let box3 = Box {
            has_two: false,
            has_three: true,
        };
        let result3 = (0, 1);
        assert_eq!(result3, box3.count());
        let box4 = Box {
            has_two: true,
            has_three: true,
        };
        let result4 = (1, 1);
        assert_eq!(result4, box4.count());
    }
}

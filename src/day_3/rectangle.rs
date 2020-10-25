pub struct Rectangle {
    pub id: u32,
    pub x1: u32,
    pub y1: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(input: &str) -> Rectangle {
        let r = input
            .split(|c| c == ' ' || c == '#' || c == '@' || c == ',' || c == ':' || c == 'x')
            .filter_map(|c| c.parse::<u32>().ok())
            .collect::<Vec<_>>();
        Rectangle {
            id: r[0],
            x1: r[1],
            y1: r[2],
            width: r[3],
            height: r[4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_rectangle() {
        let input = "#123 @ 3,2: 5x4";
        let result = Rectangle::new(input);

        assert_eq!(123, result.id);
        assert_eq!(3, result.x1);
        assert_eq!(2, result.y1);
        assert_eq!(5, result.width);
        assert_eq!(4, result.height);
    }
}

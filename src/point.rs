#[derive(Debug,PartialEq,Eq)]
pub struct Point {
    a: i32,
    b: i32,
    x: i32,
    y: i32
}

impl Point {
    pub fn new(a: i32, b: i32, x: i32, y: i32) -> Point {
        Point {
            a,
            b,
            x,
            y,
        }
    }
    pub fn validate(self) -> bool {
        return self.y.pow(2) == self.x.pow(3) + self.a * self.x + self.b;
    }
}

#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn valid_test() {
        let p = Point::new(5, 7, 5,7);
        assert_eq!(p.validate(), false);
    }
}
use std::fmt::Debug;

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i8,
    y: i8
}

#[derive(Debug)]
#[allow(dead_code)]
struct ThreeDimPoint {
    x: i8,
    y: i8,
    z: i8
}

fn main() {
    let point = Point { x: 1, y: 3};
    let three_d_point = ThreeDimPoint { x: 3, y: 5, z: 9 };

    let mut x: &dyn Debug = &point as &dyn Debug;
    println!("1: {:?}", x);

    x = &three_d_point;
    println!("2: {:?}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_format_via_debug() {
        let point = Point { x: 1, y: 3 };
        assert_eq!(format!("{:?}", point), "Point { x: 1, y: 3 }");
        let three_d = ThreeDimPoint { x: 3, y: 5, z: 9 };
        assert_eq!(format!("{:?}", three_d), "ThreeDimPoint { x: 3, y: 5, z: 9 }");
    }
}

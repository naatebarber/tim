pub struct Point {
    x: u32,
    y: u32,
}

pub struct Geometry {
    points: Vec<Vec<Point>>,
    dim: u32,
}

impl Geometry {
    fn new(dim: u32) -> Geometry {
        assert!(
            dim % 4 == 0,
            "Ensure spiral::Geometry dim attr is divisible evenly by 4"
        );

        Geometry {
            dim,
            points: vec![],
        }
    }
}

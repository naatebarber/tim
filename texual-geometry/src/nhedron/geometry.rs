use std::f32::consts::PI;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Geometry {
    radius: f32,
    points: Vec<Point>,
}

impl Geometry {
    pub fn new(radius: f32) -> Geometry {
        Geometry {
            radius,
            points: vec![],
        }
    }

    pub fn translate(&mut self, sequence: String) {
        let mut circular_paths: Vec<Vec<Point>> = vec![];
        let chars: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        for _ in 0..=chars.len() {
            circular_paths.push(vec![]);
        }

        let char_ix = |x: &char| chars.iter().position(|&y| y == *x);
        let segment_size = (2.0 * PI) / sequence.len() as f32;

        for (i, c) in sequence.chars().enumerate() {
            let c_ix = char_ix(&c).unwrap();
            let path = circular_paths.get_mut(c_ix).unwrap();
            let radial_offset = segment_size * (i as f32);

            let x: f32 = f32::cos(radial_offset) * self.radius;
            let y: f32 = f32::sin(radial_offset) * self.radius;
            let z: f32 = 0.;

            let point = Point { x, y, z };

            path.push(point);
        }

        // We now have a bunch of circles on the (x, y) plane representing hex characters.
        // Time to rotate around the Y axis

        let z_segment_size = (PI / 2.) / (chars.len() as f32);

        let geometry: Vec<Vec<Point>> = circular_paths
            .iter()
            .enumerate()
            .map(move |(i, path): (usize, &Vec<Point>)| {
                let z_radial_offset = z_segment_size * (i as f32);
                path.iter()
                    .map(|point| {
                        // Rotate perception
                        // Now apply the same transformation to each point.
                        // They all start with ratio x:1 z:0
                        // y remains unchanged.

                        // reduces a neg to -1 and a pos to 1
                        let _sign = point.x.powi(0);

                        let mut sign = 1.;
                        if point.x < 0. {
                            sign = -1.;
                        }

                        let point_radius = f32::abs(point.x);

                        let x = f32::cos(z_radial_offset) * point_radius * sign;
                        let z = f32::sin(z_radial_offset) * point_radius;

                        Point { x, y: point.y, z }
                    })
                    .collect()
            })
            .collect();

        self.points = geometry.into_iter().flatten().collect();
    }

    pub fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}

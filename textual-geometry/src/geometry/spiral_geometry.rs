use super::{Geometry, Point};

pub struct SpiralGeometry {
    points: Vec<Point>,
    dim: u32,
}

impl SpiralGeometry {
    pub fn new(dim: u32) -> Self {
        assert!(
            dim % 4 == 0,
            "Ensure spiral::Geometry dim attr is divisible evenly by 4"
        );

        SpiralGeometry {
            dim,
            points: vec![],
        }
    }

    fn fold_inward(
        offset_x: u32,
        offset_y: u32,
        dim: u32,
        sequence: &mut Vec<u8>,
    ) -> Vec<Point> {
        let mut x_p = 0;
        let mut y_p = 0;
        let mut c_dim = dim;
        let mut c_offset = 0;

        let mut points: Vec<Point> = vec![];

        let mut sequence_iter = sequence.into_iter().map(|x| x.clone());

        while c_dim > 0 {
            // top to right
            while (x_p - c_offset) < c_dim {
                x_p += 1;
                let lum: u8 = sequence_iter.next().unwrap_or_else(|| 0);
                if lum > 0 {
                    points.push(Point {
                        x: x_p.clone(),
                        y: y_p.clone(),
                        z: None
                    })
                }
            }

            // right to bottom
            while (y_p - c_offset) < c_dim {
                y_p += 1;
                let lum: u8 = sequence_iter.next().unwrap_or_else(|| 0);
                if lum > 0 {
                    points.push(Point {
                        x: x_p.clone(),
                        y: y_p.clone(),
                        z: None
                    })
                }
            }

            // bottom to left
            while x_p > c_offset {
                x_p -= 1;
                let lum: u8 = sequence_iter.next().unwrap_or_else(|| 0);
                if lum > 0 {
                    points.push(Point {
                        x: x_p.clone(),
                        y: y_p.clone(),
                        z: None
                    })
                }
            }

            // left to top
            while y_p > c_offset + 1 {
                y_p -= 1;
                let lum: u8 = sequence_iter.next().unwrap_or_else(|| 0);
                if lum > 0 {
                    points.push(Point {
                        x: x_p.clone(),
                        y: y_p.clone(),
                        z: None
                    })
                }
            }

            // cut c_dim
            c_dim -= 2;
            c_offset += 1;
            x_p = c_offset;
            y_p = c_offset;
        }

        // handle odd dim matrix with remainder c_dim of 1

        // apply offset_x and offset_y to every point in the vec
        points
            .iter()
            .map(|p| Point {
                x: p.x + offset_x,
                y: p.y + offset_y,
                z: None
            })
            .collect::<Vec<Point>>()
    }
}

impl Geometry<Point> for SpiralGeometry {
    fn set_dim(&mut self, dim: u32) {
        self.dim = dim;
    }

    fn translate(&mut self, sequence: String) {
        let mut spiral_paths: Vec<Vec<u8>> = vec![];
        let chars: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let char_ix = |x: &char| chars.iter().position(|&y| y == *x);

        for _ in 0..=chars.len() {
            spiral_paths.push(vec![])
        }

        for c in sequence.chars() {
            let c_ix = char_ix(&c).unwrap();
            for (ix, path) in spiral_paths.iter_mut().enumerate() {
                if c_ix == ix {
                    path.push(255);
                    continue;
                }
                path.push(0);
            }
        }

        let offset_step = self.dim / 4;

        let point_paths = spiral_paths
            .iter_mut()
            .enumerate()
            .map(|(i, p)| {
                let x_offset = (i % 4) as u32 * offset_step;
                let y_offset = (i / 4) as u32 * offset_step;

                SpiralGeometry::fold_inward(x_offset, y_offset, offset_step, p)
            })
            .collect::<Vec<Vec<Point>>>();

        self.points = point_paths.into_iter().flatten().collect();
    }

    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}

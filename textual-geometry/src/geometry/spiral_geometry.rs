use std::borrow::Borrow;

use super::{Geometry, Point, PreGeometry, ReversibleGeometry};

pub struct SpiralGeometry {
    points: Vec<Point>,
    dim: u32,
}

#[allow(dead_code)]
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

    fn fold_inward(offset_x: u32, offset_y: u32, dim: u32, sequence: &mut Vec<u8>) -> Vec<Point> {
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
                        z: None,
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
                        z: None,
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
                        z: None,
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
                        z: None,
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
                z: None,
            })
            .collect::<Vec<Point>>()
    }
}

impl Geometry<Point> for SpiralGeometry {
    fn set_dim(&mut self, dim: u32) {
        assert!(
            dim % 4 == 0,
            "Ensure spiral::Geometry dim attr is divisible evenly by 4"
        );
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

impl ReversibleGeometry for SpiralGeometry {
    fn reverse(&mut self, pregeometry: PreGeometry) -> Option<String> {
        let ((w, _), points) = pregeometry;
        let chars: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let dim = w;
        let outer_offset_step = dim / 4;

        let mut cursors: Vec<Point> = Vec::with_capacity(16);
        for i in 0..=15 {
            let x_offset = i % 4;
            let y_offset = i / 4;

            // Initialize each cursor at the first ix of it's respective character
            cursors.push(Point {
                x: x_offset * outer_offset_step,
                y: y_offset * outer_offset_step,
                z: None,
            })
        }

        let mut inner_offset = 0;
        let mut x = 0;
        let mut y = 0;

        let points_grid: Vec<&[Point]> = points.chunks(dim as usize).collect();

        let next_char = |x: usize, y: usize| {
            println!("x {} y {}", x, y);
            let next_chars: Vec<char> = cursors
                .iter()
                .enumerate()
                .filter_map(|(cursor_ix, cursor)| {
                    let c_x = cursor.x as usize + x;
                    let c_y = cursor.y as usize + y;

                    let row = points_grid.get(c_y).unwrap();
                    let v = &row[c_x];
                    if v.z.unwrap_or(0) > 0 {
                        let char_at_ix = chars[cursor_ix];
                        return Some(char_at_ix);
                    }

                    return None;
                })
                .collect();

            println!("{:?}", next_chars);

            next_chars
        };

        let mut reconstructed = String::default();

        while inner_offset < outer_offset_step / 2 {
            // top to right
            while x < (outer_offset_step - (2 * inner_offset)) {
                x += 1;
                let next = next_char(x as usize, y as usize);
                if let Some(c) = next.get(0) {
                    reconstructed.push(*c);
                }
            }
            // right to bottom
            while y < (outer_offset_step - (2 * inner_offset)) {
                y += 1;
                let next = next_char(x as usize, y as usize);
                if let Some(c) = next.get(0) {
                    reconstructed.push(*c);
                }
            }

            // bottom to left
            while x > inner_offset {
                x -= 1;
                let next = next_char(x as usize, y as usize);
                if let Some(c) = next.get(0) {
                    reconstructed.push(*c);
                }
            }

            // left to top
            while y > inner_offset + 1 {
                y -= 1;
                let next = next_char(x as usize, y as usize);
                if let Some(c) = next.get(0) {
                    reconstructed.push(*c);
                }
            }

            // increase inner offset
            inner_offset += 1;
            x = inner_offset;
            y = inner_offset;
        }

        return Some(reconstructed);
    }
}

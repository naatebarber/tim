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

    /**
     * Form a dim/4 spiral grid and perform an action cb() at each x/y
     */
    fn fold(dim: u32, mut cb: impl FnMut(u32, u32) -> ()) {
        assert!(dim % 4 == 0);

        let outer_offset_step = dim / 4;

        let mut inner_offset = 0;
        let mut x = 0;
        let mut y = 0;

        while inner_offset < outer_offset_step / 2 {
            // top to right
            while x < (outer_offset_step - inner_offset - 1) {
                cb(x, y);
                x += 1;
            }
            // right to bottom
            while y < (outer_offset_step - inner_offset - 1) {
                cb(x, y);
                y += 1;
            }

            // bottom to left
            while x > inner_offset {
                cb(x, y);
                x -= 1;
            }

            // left to top
            while y > inner_offset {
                cb(x, y);
                y -= 1;
            }

            // increase inner offset
            inner_offset += 1;
            x = inner_offset - 1;
            y = inner_offset;
        }
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
        let chars: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let char_ix = |x: &char| chars.iter().position(|&y| y == *x);
        let mut char_iter = sequence.chars();

        let outer_offset_step = self.dim / 4;
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

        let mut points: Vec<Point> = vec![];

        let spiralize = |x: u32, y: u32| {
            let next_char = char_iter.next();
            if let Some(next_char) = next_char {
                let next_char_ix = char_ix(&next_char).unwrap();

                for (i, c) in cursors.iter().enumerate() {
                    if i == next_char_ix {
                        points.push(Point {
                            x: x + c.x,
                            y: y + c.y,
                            z: Some(255),
                        })
                    }
                }
            }
        };

        SpiralGeometry::fold(self.dim, spiralize);

        self.points = points;
    }

    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}

impl ReversibleGeometry for SpiralGeometry {
    fn reverse(&mut self, pregeometry: PreGeometry) -> Option<String> {
        let (_, points) = pregeometry;
        let chars: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let outer_offset_step = self.dim / 4;

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

        let points_grid: Vec<&[Point]> = points.chunks(self.dim as usize).collect();

        let mut reconstructed = String::default();

        let next_char = |x: u32, y: u32| {
            let next_chars: Vec<char> = cursors
                .iter()
                .enumerate()
                .filter_map(|(cursor_ix, cursor)| {
                    let c_x = cursor.x + x;
                    let c_y = cursor.y + y;

                    let row = points_grid.get(c_y as usize).unwrap();
                    let v = &row[c_x as usize];

                    if v.z.unwrap_or(0) > 0 {
                        let char_at_ix = chars[cursor_ix];
                        return Some(char_at_ix);
                    }

                    return None;
                })
                .collect();

            if let Some(c) = next_chars.get(0) {
                reconstructed.push(*c);
            }
        };

        SpiralGeometry::fold(self.dim, next_char);

        return Some(reconstructed);
    }
}

#![allow(unused)]

use super::{Geometry, Point, PreGeometry, ReversibleGeometry};
use itertools::Itertools;

pub struct DBLGeometry {
    dim: u32,
    points: Vec<Point>,
}

impl DBLGeometry {
    pub fn new(dim: u32) -> Self {
        Self {
            dim,
            points: vec![],
        }
    }

    fn hex_to_bits(&self, n: u32) -> [u32; 4] {
        let mut bits = [0; 4];
        for i in 0..4 {
            // Shift the number i bits to the right and check the least significant bit
            bits[i] = (n & (1 << i) != 0) as u32;
        }
        bits
    }

    fn bits_to_hex(&self, bits: &[bool]) -> usize {
        let mut n = 0;
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                n |= 1 << i;
            }
        }
        n
    }
}

impl Geometry<Point> for DBLGeometry {
    fn set_dim(&mut self, dim: u32) {
        assert!(
            dim % 2 == 0,
            "Ensure DBLGeometry dim attr is divisible evenly by 2"
        );
        self.dim = dim;
    }

    fn translate(&mut self, sequence: String) {
        let chars: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let char_ix = |x: &char| chars.iter().position(|&y| y == *x);

        let order = f32::log2(self.dim as f32) as u32; // What power of 2 the dimension is
        let num_blocks = (self.dim / 2).pow(2) as usize;

        let chunks = sequence.chars().chunks(num_blocks);

        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_blocks);

        for _ in 0..num_blocks {
            stacks.push(vec![]);
        }

        for chunk in &chunks {
            for (i, c) in chunk.enumerate() {
                stacks[i].push(c);
            }
        }

        for (i, stack) in stacks.into_iter().enumerate() {
            let mut points: Vec<u32> = vec![0, 0, 0, 0];
            for (j, c) in stack.into_iter().enumerate() {
                let ix = char_ix(&c).unwrap() as u32;
                let mut bits = self.hex_to_bits(ix);
                bits.iter_mut()
                    .for_each(|b| *b *= 2u32.pow(order - 1 - j as u32));
                points = points.into_iter().zip(bits).map(|(a, b)| a + b).collect();
            }

            let x = ((i % (self.dim / 2) as usize) * 2) as u32;
            let y = (((i / (self.dim / 2) as usize) * 2) as f32).floor() as u32;

            self.points.push(Point {
                x,
                y,
                z: Some(points[0]),
            });

            self.points.push(Point {
                x: x + 1,
                y,
                z: Some(points[1]),
            });

            self.points.push(Point {
                x,
                y: y + 1,
                z: Some(points[2]),
            });

            self.points.push(Point {
                x: x + 1,
                y: y + 1,
                z: Some(points[3]),
            });
        }

        let num_layers = sequence.len().div_ceil(num_blocks) as u32;
        let layer_count_bits = self.hex_to_bits(num_layers);
        let first_block = &mut self.points[0..4];
        for (i, point) in first_block.into_iter().enumerate() {
            if let Some(lum) = point.z {
                point.z = Some(lum + layer_count_bits[i]);
            }
        }

        let null_ix = sequence.len() % num_blocks;
        let null_point = &mut self.points[null_ix];
        null_point.z = Some(null_point.z.unwrap() + 1);
        // let null_block = &mut self.points[null_ix..null_ix+4];
        // for point in null_block {
        //     if let Some(lum) = point.z {
        //         point.z = Some(lum + 1)
        //     }
        // }
    }

    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}

impl ReversibleGeometry for DBLGeometry {
    fn reverse(&mut self, pregeometry: PreGeometry) -> Option<String> {
        let ((dim, _), mut raw_points) = pregeometry;
        let chars: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let dim = dim as usize;
        let mut raw_points = raw_points.into_iter().map(Some).collect_vec();
        let mut points: Vec<Point> = vec![];

        for i in (0..dim).step_by(2) {
            for j in (0..dim).step_by(2) {
                let loc = dim * i + j;
                if let Some(point) = raw_points[loc].take() {
                    points.push(point);
                }
                if let Some(point) = raw_points[loc + 1].take() {
                    points.push(point);
                }
                if let Some(point) = raw_points[loc + dim].take() {
                    points.push(point);
                }
                if let Some(point) = raw_points[loc + dim + 1].take() {
                    points.push(point);
                }
            }
        }

        let first_block = &mut points[0..4];
        let layer_count_bits = first_block
            .into_iter()
            .map(|p| {
                let lum = p.z.unwrap();
                (lum & 1) == 1
            })
            .collect_vec();
        let num_layers = self.bits_to_hex(&layer_count_bits);

        let mut out = String::default();

        println!("{num_layers}");

        'outer: for i in (1..8).rev() {
            for (j, block) in points.chunks(4).enumerate() {
                if 7 - num_layers == i && ((block[0].z.unwrap() & 1) == 1) && (j != 0) {
                    println!("j {j} i {i}");
                    break 'outer;
                }
                let bits = block
                    .into_iter()
                    .map(|p| {
                        let lum = p.z.unwrap();
                        (lum & (1 << i)) != 0
                    })
                    .collect_vec();
                let ix = self.bits_to_hex(&bits);
                let c = chars[ix];
                out.push(c)
            }
        }
        Some(out)
    }
}

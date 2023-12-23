use crate::geometry::{Geometry, LossyPoint, Point};
use crate::rendering::{Bitmap, Svg};
use hex;

pub struct Encoder<'a> {
    geometry: &'a mut dyn Geometry<Point>,
    dim: u32,
}

impl<'a> Encoder<'a> {
    pub fn from_sequence(
        dim: u32,
        input_sequence: String,
        geometry: &'a mut dyn Geometry<Point>,
    ) -> Self {
        let hex_repr = hex::encode(input_sequence);

        geometry.set_dim(dim);
        geometry.translate(hex_repr);

        Encoder { geometry, dim }
    }

    pub fn to(&self, path: &str) {
        let mut bitmap = Bitmap::new(self.dim);
        bitmap.from_geometry(self.geometry);
        bitmap.save(path);
    }
}

pub struct LossyEncoder<'a> {
    geometry: &'a mut dyn Geometry<LossyPoint>,
    dim: u32,
    pad: u32,
}

impl<'a> LossyEncoder<'a> {
    pub fn from_sequence(
        dim: u32,
        pad: u32,
        input_sequence: String,
        geometry: &'a mut dyn Geometry<LossyPoint>,
    ) -> Self {
        let hex_repr = hex::encode(input_sequence);

        geometry.set_dim(dim);
        geometry.translate(hex_repr);

        LossyEncoder { geometry, dim, pad }
    }

    pub fn to(&self, path: &str) {
        let mut svg = Svg::new(self.dim, self.pad);
        svg.from_geometry(self.geometry);
        svg.export(path);
    }
}

use super::artist::Artist;
use super::bitmap::Bitmap;
use super::geometry::Geometry;
use crate::Encoder;

pub struct SpiralEncoder {
    geometry: Geometry,
    dim: u32,
    pad: u32,
}

impl Encoder for SpiralEncoder {
    fn from_sequence(input_sequence: String) -> SpiralEncoder {
        let dim = 512;
        let pad = 5;

        let mut geometry = Geometry::new(dim);
        geometry.translate(input_sequence);

        SpiralEncoder { geometry, dim, pad }
    }

    fn to(&self, path: &str) {
        let mut bitmap = Bitmap::new(self.dim, self.pad);
        bitmap.from_geometry(&self.geometry);
        bitmap.save(path);
    }
}

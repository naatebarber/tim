use super::{artist::Artist, geometry::Geometry};
use texual_geometry::Encoder;

pub struct NHedronEncoder {
    geometry: Geometry,
    radius: f32,
    pad: f32,
}

impl Encoder<NHedronEncoder> for NHedronEncoder {
    fn from_sequence(input_sequence: String) -> NHedronEncoder {
        // HARDCODE RADIUS AND PAD FOR NOW

        let radius: f32 = 256.;
        let pad: f32 = 5.;

        let mut geometry = Geometry::new(radius);
        geometry.translate(input_sequence);

        NHedronEncoder {
            geometry,
            radius,
            pad,
        }
    }

    fn to(&self, path: &str) {
        let mut artist = Artist::new(self.radius, self.pad);
        artist.from_geometry(&self.geometry);
        artist.export(path);
    }
}

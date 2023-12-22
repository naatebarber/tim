use super::{geometry::Geometry, artist::Artist};
use texual_geometry::Encoder;

pub struct NHedronEncoder {
    geometry: Geometry,
    radius: f32
}

impl Encoder<NHedronEncoder> for NHedronEncoder {
    fn from_sequence(input_sequence: String) -> NHedronEncoder {
        // HARDCODE RADIUS FOR NOW

        let radius: f32 = 256.;

        let mut geometry = Geometry::new(radius);
        geometry.translate(input_sequence);
        
        NHedronEncoder {
            geometry,
            radius
        }
    }

    fn to(&self, path: &str) {
        let mut artist = Artist::new(self.radius);
        artist.from_geometry(&self.geometry);
        artist.export(path);
    }
}
use crate::Encoder;

pub struct SpiralEncoder;

impl Encoder for SpiralEncoder {
    fn from_sequence(input_sequence: String) -> SpiralEncoder {
        SpiralEncoder {}
    }

    fn to(&self, path: &str) {}
}

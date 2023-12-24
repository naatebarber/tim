pub mod encoder;
pub mod geometry;
pub mod rendering;

pub trait Encoder {
    fn from_sequence(input_sequence: String) -> Self;

    fn to(&self, path: &str);
}

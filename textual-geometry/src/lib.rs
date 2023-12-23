
pub mod geometry;
pub mod rendering;
pub mod encoder;

pub trait Encoder {
    fn from_sequence(input_sequence: String) -> Self;

    fn to(&self, path: &str);
}
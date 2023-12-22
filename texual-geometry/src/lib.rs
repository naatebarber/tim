mod nhedron;
mod spiral;

pub trait Encoder {
    fn from_sequence(input_sequence: String) -> Self;

    fn to(&self, path: &str);
}

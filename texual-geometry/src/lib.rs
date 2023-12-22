pub trait Encoder<T> {
    fn from_sequence(input_sequence: String) -> T;

    fn to(&self, path: &str);
}

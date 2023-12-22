mod nhedron;

use std::env;
use hex;
use texual_geometry::Encoder;

use nhedron::encoder::NHedronEncoder;

fn main() {
    let input_txt = env::args().nth(1).unwrap_or_else(|| String::from("Generic"));
    let transform_hex = hex::encode(input_txt);
    let encoder = NHedronEncoder::from_sequence(transform_hex);

    let cwd = env::current_dir().unwrap();
    let cwd = cwd.to_str().unwrap();
    let outfile = format!("{}/{}", cwd, "test.svg");

    encoder.to(&outfile);
}

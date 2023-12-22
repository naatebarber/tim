mod nhedron;

use hex;
use std::env;
use texual_geometry::Encoder;

use nhedron::encoder::NHedronEncoder;

fn main() {
    let mut input_txt: String = String::default();
    let mut arg_iter = env::args().skip(1);

    while let Some(next_str) = arg_iter.next() {
        input_txt.push_str(&format!("{} ", next_str));
    }

    let transform_hex = hex::encode(input_txt);
    let encoder = NHedronEncoder::from_sequence(transform_hex);

    let cwd = env::current_dir().unwrap();
    let cwd = cwd.to_str().unwrap();
    let outfile = format!("{}/{}", cwd, "test.svg");

    encoder.to(&outfile);
}

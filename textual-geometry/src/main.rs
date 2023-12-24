mod geometry;

use std::env;
use textual_geometry::encoder::Encoder;
use textual_geometry::encoder::LossyEncoder;
use textual_geometry::geometry::NHedronGeometry;
use textual_geometry::geometry::SpiralGeometry;
use textual_geometry::geometry::{Geometry, ReversibleGeometry};
use textual_geometry::rendering::bitmap::Bitmap;

fn main() {
    let mut input_txt: String = String::default();
    let mut arg_iter = env::args().skip(1);
    while let Some(next_str) = arg_iter.next() {
        input_txt.push_str(&format!("{} ", next_str));
    }

    let cwd = env::current_dir().unwrap();
    let cwd = cwd.to_str().unwrap();

    encode_decode(input_txt, cwd);
}

fn encode_decode(input_txt: String, cwd: &str) {
    // let hex_sample = "0123456789abcdef";
    // let hex_sample = "";
    // let mut hex_str = String::default();
    // for _ in 0..=270 {
    //     hex_str.push_str(hex_sample);
    // }

    let hex_str = hex::encode(input_txt);
    println!("Original: {}", hex_str);

    let dim = 256;
    let mut spiral_geo = SpiralGeometry::new(dim);
    spiral_geo.translate(hex_str);
    let mut bitmap = Bitmap::new(dim);
    bitmap.from_geometry(&spiral_geo);
    let spiral_outfile = format!("{}/output_geometry/{}", cwd, "spiral.png");
    bitmap.save(&spiral_outfile);

    let pregeometry =
        Bitmap::to_points(&spiral_outfile).expect("Failed to load pregeometry from src.");

    let mut geometry = SpiralGeometry::new(pregeometry.0 .0 - 1);
    // let mut geometry = SpiralGeometry::new(pregeometry.0.0);
    let reconstructed = geometry.reverse(pregeometry).unwrap();
    println!("Reconstructed: {}", reconstructed);

    let bytes = hex::decode(reconstructed).unwrap();
    // let s = std::str::from_utf8(&bytes).unwrap();
    let s = String::from_utf8_lossy(&bytes);
    println!("UTF: {}", s);
}

#[allow(dead_code)]
fn encode_all(input_txt: String, cwd: &str) {
    let mut spiral_geo = SpiralGeometry::new(0);
    let spiral_encoder = Encoder::from_sequence(256, input_txt.clone(), &mut spiral_geo);
    let spiral_outfile = format!("{}/output_geometry/{}", cwd, "spiral.png");
    spiral_encoder.to(&spiral_outfile);

    let mut nhedron_geo = NHedronGeometry::new(0.);
    let nhedron_encoder = LossyEncoder::from_sequence(256, 2, input_txt, &mut nhedron_geo);
    let nhedron_outfile = format!("{}/output_geometry/{}", cwd, "nhedron.svg");
    nhedron_encoder.to(&nhedron_outfile);
}

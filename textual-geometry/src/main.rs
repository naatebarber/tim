mod geometry;

use textual_geometry::encoder::LossyEncoder;
use std::env;
use textual_geometry::geometry::NHedronGeometry;
use textual_geometry::geometry::SpiralGeometry;
use textual_geometry::encoder::Encoder;

fn main() {
    let mut input_txt: String = String::default();
    let mut arg_iter = env::args().skip(1);
    while let Some(next_str) = arg_iter.next() {
        input_txt.push_str(&format!("{} ", next_str));
    }

    let cwd = env::current_dir().unwrap();
    let cwd = cwd.to_str().unwrap();

    let mut spiral_geo = SpiralGeometry::new(0);
    let spiral_encoder = Encoder::from_sequence(256, 2, input_txt.clone(), &mut spiral_geo);
    let spiral_outfile = format!("{}/output_geometry/{}", cwd, "spiral.png");
    spiral_encoder.to(&spiral_outfile);

    let mut nhedron_geo = NHedronGeometry::new(0.);
    let nhedron_encoder = LossyEncoder::from_sequence(256, 2, input_txt, &mut nhedron_geo);
    let nhedron_outfile = format!("{}/output_geometry/{}", cwd, "nhedron.svg");
    nhedron_encoder.to(&nhedron_outfile);
}

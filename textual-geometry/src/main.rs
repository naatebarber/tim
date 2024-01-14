mod geometry;

extern crate getopts;

use getopts::Options;
use std::env;
use std::io::{self, BufRead};
use textual_geometry::encoder::Encoder;
use textual_geometry::encoder::LossyEncoder;
use textual_geometry::geometry::NHedronGeometry;
use textual_geometry::geometry::SpiralGeometry;
use textual_geometry::geometry::{Geometry, ReversibleGeometry};
use textual_geometry::rendering::bitmap::Bitmap;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "e",
        "encode",
        "Geometry format with which to encode the input sequence",
        "spiral",
    );
    opts.optopt(
        "d",
        "decode",
        "Geometry format with which to decode the input image",
        "spiral",
    );
    opts.optopt(
        "p",
        "path",
        "Path to geometry file",
        "/path/to/my/geometry.png",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("e") {
        let mut path: String = String::default();
        if let Some(p) = matches.opt_str("p") {
            path = p;
        } else if matches.opt_str("p").is_none() {
            print_usage(&program, opts);
            std::process::exit(1);
        }

        let input_text = read_stdin();
        if input_text.len() < 1 {
            eprintln!("Nothing to encode.");
            print_usage(&program, opts);
        }

        match matches.opt_str("e").as_deref() {
            Some("spiral") => spiral_encode(input_text, &path),
            _ => {
                println!("Defaulting to Spiral");
                spiral_encode(input_text, &path)
            }
        }
    } else if matches.opt_present("d") {
        let mut path: String = String::default();
        if let Some(p) = matches.opt_str("p") {
            path = p;
        } else if matches.opt_str("p").is_none() {
            print_usage(&program, opts);
            std::process::exit(1);
        }

        match matches.opt_str("d").as_deref() {
            Some("spiral") => spiral_decode(&path),
            _ => {
                println!("Defaulting to Spiral");
                spiral_decode(&path)
            }
        }
    } else {
        print_usage(&program, opts);
    }
}

fn read_stdin() -> String {
    let input = io::stdin();
    let mut stdin = input.lock();

    let mut line = String::new();

    while let Ok(n_bytes) = stdin.read_line(&mut line) {
        if n_bytes == 0 {
            break;
        }
    }

    return line;
}

fn spiral_encode(input_text: String, path: &str) {
    let hex_str = hex::encode(input_text);
    let dim = 256;
    let mut spiral_geo = SpiralGeometry::new(dim);
    spiral_geo.translate(hex_str);
    let mut bitmap = Bitmap::new(dim);
    bitmap.from_geometry(&spiral_geo);
    bitmap.save(&path);
}

fn spiral_decode(path: &str) {
    let pregeometry = Bitmap::to_points(path).expect("Failed to load pregeometry from src.");
    let mut geometry = SpiralGeometry::new(pregeometry.0 .0);
    let reconstructed = geometry.reverse(pregeometry).unwrap();
    let bytes = hex::decode(reconstructed).unwrap();
    let s = String::from_utf8_lossy(&bytes);
    println!("{}", s);
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

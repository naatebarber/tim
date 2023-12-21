use crate::geometry;
use crate::geometry::Geometry;
use draw::{
    render::{bitmap::PNGRenderer, Renderer},
    *,
};
use std::env;

pub struct Artist;

impl Artist {
    pub fn from_geometry(geometry: &Geometry) {
        let diameter = 510;
        let radius: f32 = (diameter as f32) / 2.;

        let mut canvas = Canvas::new(diameter, diameter);
        Artist::background(&mut canvas);

        let pts: Vec<Drawing> = geometry
            .get_points()
            .iter()
            .map(|point: &geometry::Point| {
                let luminosity = point.z;
                let x = radius + point.x;
                let y = radius + point.y;

                Artist::point_at(x, y, Some(luminosity as u8))
            })
            .collect();

        for d in pts.into_iter() {
            canvas.display_list.add(d);
        }

        Artist::export(&canvas);
    }

    pub fn point_at(x: f32, y: f32, luminosity: Option<u8>) -> Drawing {
        let mut luminosity = luminosity.unwrap_or_else(|| 255);

        if luminosity < 0 || luminosity > 255 {
            println!("Invalid luminosity");
            luminosity = 255;
        }

        Drawing::new()
            .with_shape(Shape::Rectangle {
                width: 1,
                height: 1,
            })
            .with_position(Point { x, y })
            .with_style(Style::filled(RGB {
                r: luminosity,
                g: luminosity,
                b: luminosity,
            }))
    }

    pub fn background(canvas: &mut Canvas) {
        let bg = Drawing::new()
            .with_shape(Shape::Rectangle {
                width: canvas.width,
                height: canvas.height,
            })
            .with_position(Point { x: 0., y: 0. })
            .with_style(Style::filled(RGB { r: 0, g: 0, b: 0 }));

        canvas.display_list.add(bg);
    }

    pub fn export(canvas: &Canvas) {
        let cwd = env::current_dir().unwrap();
        let cwd = cwd.to_str().unwrap();
        let outfile = format!("{}/{}", cwd, "test.svg");

        println!("{}", outfile);

        render::save(canvas, &outfile, SvgRenderer::new())
            .expect("Failed to save your geometry to disk.")
    }
}

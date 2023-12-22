use super::geometry;
use super::geometry::Geometry;
use draw::*;

pub struct Artist {
    pad: u32,
    canvas: Canvas,
}

impl Artist {
    pub fn new(dim: u32, pad: u32) -> Artist {
        let canvas = Canvas::new(dim, dim);

        Artist { pad, canvas }
    }

    pub fn from_geometry(&mut self, geometry: &Geometry) {
        Artist::background(&mut self.canvas);

        let pts: Vec<Drawing> = geometry
            .get_points()
            .iter()
            .map(|point: &geometry::Point| {
                let luminosity = 255;
                let x = point.x + self.pad;
                let y = point.y + self.pad;

                Artist::point_at(x, y, Some(luminosity as u8))
            })
            .collect();

        for d in pts.into_iter() {
            self.canvas.display_list.add(d);
        }
    }

    pub fn point_at(x: u32, y: u32, luminosity: Option<u8>) -> Drawing {
        let luminosity = luminosity.unwrap_or_else(|| 255);

        Drawing::new()
            .with_shape(Shape::Rectangle {
                width: 1,
                height: 1,
            })
            .with_position(Point {
                x: x as f32,
                y: y as f32,
            })
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

    pub fn export(&self, path: &str) {
        // let cwd = env::current_dir().unwrap();
        // let cwd = cwd.to_str().unwrap();
        // let outfile = format!("{}/{}", cwd, "test.svg");
        render::save(&self.canvas, path, SvgRenderer::new())
            .expect("Failed to save your geometry to disk.")
    }
}

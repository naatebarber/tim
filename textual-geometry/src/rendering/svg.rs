use crate::geometry::{Geometry, LossyPoint};
use draw::*;

pub struct Svg {
    pad: u32,
    canvas: Canvas,
}

impl Svg {
    pub fn new(dim: u32, pad: u32) -> Svg {
        let imsize = pad * 2 + dim;
        let canvas = Canvas::new(imsize, imsize);

        Svg { pad, canvas }
    }

    pub fn from_geometry(&mut self, geometry: &dyn Geometry<LossyPoint>) {
        Svg::background(&mut self.canvas);

        let pts: Vec<Drawing> = geometry
            .get_points()
            .iter()
            .map(|point: &LossyPoint| {
                let luminosity = point.z.unwrap_or_else(|| 0.) as u8;

                let x = self.pad as f32 + point.x;
                let y = self.pad as f32 + point.y;

                Svg::point_at(x, y, luminosity)
            })
            .collect();

        for d in pts.into_iter() {
            self.canvas.display_list.add(d);
        }
    }

    pub fn point_at(x: f32, y: f32, luminosity: u8) -> Drawing {
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

    pub fn export(&self, path: &str) {
        render::save(&self.canvas, path, SvgRenderer::new())
            .expect("Failed to save your geometry to disk.")
    }
}

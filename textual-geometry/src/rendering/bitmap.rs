use image::GrayImage;
use crate::geometry::Geometry;
use crate::geometry::Point;

pub struct Bitmap {
    buf: GrayImage,
    pad: u32,
}

impl Bitmap {
    pub fn new(dim: u32, pad: u32) -> Bitmap {
        let imsize = 2 * pad + dim;
        let image_buffer = GrayImage::from_fn(imsize, imsize, |_, _| image::Luma([0u8]));

        Bitmap {
            buf: image_buffer,
            pad,
        }
    }

    pub fn from_geometry(&mut self, geometry: &dyn Geometry<Point>) {
        let points = geometry.get_points();

        for point in points.iter() {
            let pix = self
                .buf
                .get_pixel_mut(self.pad + point.x, self.pad + point.y);
            *pix = image::Luma([255u8])
        }
    }

    pub fn save(&self, path: &str) {
        self.buf.save(path).unwrap();
    }
}

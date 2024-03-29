use std::io::Cursor;

use crate::geometry::Geometry;
use crate::geometry::Point;
use crate::geometry::PreGeometry;
use image::io::Reader as ImageReader;
use image::GrayImage;
use image::ImageError;

pub struct Bitmap {
    pub buf: GrayImage,
}

impl Bitmap {
    pub fn new(dim: u32) -> Bitmap {
        let imsize = dim;
        let image_buffer = GrayImage::from_fn(imsize, imsize, |_, _| image::Luma([0u8]));

        Bitmap { buf: image_buffer }
    }

    pub fn from_geometry(&mut self, geometry: &dyn Geometry<Point>) {
        let points = geometry.get_points();

        for point in points.iter() {
            let pix = self.buf.get_pixel_mut(point.x, point.y);
            *pix = image::Luma([255u8])
        }
    }

    pub fn to_sparse_points(src: &str) -> Result<PreGeometry, ImageError> {
        let image = ImageReader::open(src)?.decode()?;
        let luma8 = image.into_luma8();
        let (width, height) = luma8.dimensions();

        let points = luma8
            .enumerate_pixels()
            .filter_map(|(x, y, pix)| -> Option<Point> {
                if pix.0[0] > 0 {
                    return Some(Point {
                        x,
                        y,
                        z: Some(pix.0[0] as u32),
                    });
                } else {
                    None
                }
            })
            .collect::<Vec<Point>>();

        Ok(((width, height), points))
    }

    pub fn to_points(src: &str) -> Result<PreGeometry, ImageError> {
        let image = ImageReader::open(src)?.decode()?;
        let luma8 = image.into_luma8();
        let (width, height) = luma8.dimensions();

        let points = luma8
            .enumerate_pixels()
            .map(|(x, y, pix)| -> Point {
                return Point {
                    x,
                    y,
                    z: Some(pix.0[0] as u32),
                };
            })
            .collect::<Vec<Point>>();

        Ok(((width, height), points))
    }

    pub fn save(&self, path: &str) {
        self.buf.save(path).unwrap();
    }

    pub fn write_png_to(&self, buffer: &mut std::io::BufWriter<Cursor<Vec<u8>>>) {
        self.buf.write_to(buffer, image::ImageFormat::Png).unwrap()
    }

    pub fn get_export_type() -> image::ImageOutputFormat {
        image::ImageOutputFormat::Png
    }
}

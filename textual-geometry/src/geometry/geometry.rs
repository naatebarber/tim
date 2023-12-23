pub struct Point {
    pub x: u32,
    pub y: u32,
    pub z: Option<u32>
}

pub struct LossyPoint {
    pub x: f32,
    pub y: f32,
    pub z: Option<f32>
}

pub trait Geometry<PointType> {
    fn set_dim(&mut self, dim: u32);

    fn translate(&mut self, sequence: String);

    fn get_points(&self) -> &Vec<PointType>;
}
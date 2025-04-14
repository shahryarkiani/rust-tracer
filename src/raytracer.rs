use crate::{canvas::{self, Canvas}, ray::{Point3, Ray}};



pub struct RayTracer {
    viewport_height: f64,
    viewport_width: f64,
    focal_len: f64,
    camera_pos: Point3
}

impl RayTracer {
    pub fn new(canvas: &impl Canvas, viewport_height: f64, focal_length: f64) -> RayTracer {
        // Camera is positioned at (0, 0, 0)

        let viewport_width = viewport_height * (canvas.width() as f64 / canvas.height() as f64);

        RayTracer {
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_len: focal_length,
            camera_pos: Point3::new(0.0, 0.0, 0.0)
        }
    }


    pub fn draw(canvas: &mut impl Canvas) {

    }
}
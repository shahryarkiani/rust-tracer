use core::f32;

use crate::ray::{Interval, Point3, Ray};

#[derive(Default, Clone, Copy)]
pub struct Bbox {
    axis_intervals: [Interval; 3],
}

impl Bbox {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Bbox {
        Bbox {
            axis_intervals: [x, y, z],
        }
    }

    pub fn from_points(a: Point3, b: Point3, c: Point3) -> Bbox {
        let mut bbox = Bbox::default();

        bbox.axis_intervals[0] = Interval::new(
            f32::min(a.x, f32::min(b.x, c.x)),
            f32::max(a.x, f32::max(b.x, c.x)),
        );
        bbox.axis_intervals[1] = Interval::new(
            f32::min(a.y, f32::min(b.y, c.y)),
            f32::max(a.y, f32::max(b.y, c.y)),
        );
        bbox.axis_intervals[2] = Interval::new(
            f32::min(a.x, f32::min(b.x, c.x)),
            f32::max(a.x, f32::max(b.x, c.x)),
        );
        bbox
    }

    pub fn axis_interval(&self, i: usize) -> Interval {
        self.axis_intervals[i]
    }

    pub fn intersects(&self, ray: Ray) -> bool {
        let mut tmin: f32 = 0.0;
        let mut tmax = f32::INFINITY;

        for i in 0..=2usize {
            let dir_inv = 1.0 / ray.dir().axis_val(i);

            let sign = f32::is_sign_positive(dir_inv);

            let bmin = self.axis_interval(i).get_val(sign as usize);
            let bmax = self.axis_interval(i).get_val(!sign as usize);

            let dmin = (bmin - ray.origin().axis_val(i)) * dir_inv;
            let dmax = (bmax - ray.origin().axis_val(i)) * dir_inv;

            tmin = f32::max(tmin, dmin);
            tmax = f32::min(tmax, dmax);
        }

        return tmax > tmax;
    }
}

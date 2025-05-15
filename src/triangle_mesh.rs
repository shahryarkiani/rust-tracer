use crate::{
    hittable::{HitInfo, Hittable},
    material::Material,
    ray::{Interval, Point3, Ray},
};

pub struct TriangleMesh {
    indices: Vec<u32>,
    vertices: Vec<Point3>,
    material: Material,
}

impl TriangleMesh {
    pub fn new(material: Material) -> TriangleMesh {
        TriangleMesh {
            indices: Vec::new(),
            vertices: Vec::new(),
            material: material,
        }
    }

    pub fn add_vertex(&mut self, new_vertex: Point3) {
        self.vertices.push(new_vertex);
    }

    pub fn add_triangle(&mut self, vertex_index_1: u32, vertex_index_2: u32, vertex_index_3: u32) {
        self.indices.push(vertex_index_1);
        self.indices.push(vertex_index_2);
        self.indices.push(vertex_index_3);
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool {
        hit_info_out.t = f64::INFINITY;

        let mut i: usize = 0;

        while i < self.indices.len() {
            let a = self.vertices[self.indices[i] as usize];
            let b = self.vertices[self.indices[i + 1] as usize];
            let c = self.vertices[self.indices[i + 2] as usize];
            i += 3;

            triangle_hit(a, b, c, ray, interval, hit_info_out);
        }
        if hit_info_out.t < f64::INFINITY {
            hit_info_out.material = self.material;
            return true;
        }

        false
    }
}

fn triangle_hit(
    a: Point3,
    b: Point3,
    c: Point3,
    ray: Ray,
    interval: Interval,
    hit_info_out: &mut HitInfo,
) -> bool {
    let normal = (b - a).cross(c - a);
    let tri_normal = normal / normal.magnitude();

    let det = tri_normal.dot(a);

    if det <= 0.00001 {
        return false;
    }

    let t = (det - tri_normal.dot(ray.origin())) / (tri_normal.dot(ray.dir()));

    if hit_info_out.t < t {
        return false;
    }

    let q = ray.at(t);

    if ((b - a).cross(q - a)).dot(tri_normal) < 0.0 {
        return false;
    }
    if ((c - b).cross(q - b)).dot(tri_normal) < 0.0 {
        return false;
    }
    if ((a - c).cross(q - c).dot(tri_normal)) < 0.0 {
        return false;
    }

    hit_info_out.normal = tri_normal;
    hit_info_out.t = t;
    hit_info_out.point = q;

    true
}

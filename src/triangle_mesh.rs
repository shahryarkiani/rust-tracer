use core::f32;

use crate::{
    bbox::Bbox,
    hittable::{HitInfo, Hittable},
    material::Material,
    ray::{Interval, Point3, Ray},
    vec3::Vec3,
};

pub struct TriangleMesh {
    indices: Vec<u32>,
    vertices: Vec<Point3>,
    normals: Vec<Vec3>,
    material: Material,
}

#[derive(Default)]
pub struct Scene {
    meshes: Vec<TriangleMesh>,
    nodes: Vec<Bbox>,
}

impl Scene {
    pub fn add_mesh(&mut self, mesh: TriangleMesh) {
        let mut min_x: f32 = f32::INFINITY;
        let mut min_y: f32 = f32::INFINITY;
        let mut min_z: f32 = f32::INFINITY;

        let mut max_x: f32 = f32::NEG_INFINITY;
        let mut max_y: f32 = f32::NEG_INFINITY;
        let mut max_z: f32 = f32::NEG_INFINITY;

        for vertex in &mesh.vertices {
            min_x = f32::min(min_x, vertex.x);
            max_x = f32::max(max_x, vertex.x);

            min_y = f32::min(min_y, vertex.y);
            max_y = f32::max(max_y, vertex.y);

            min_z = f32::min(min_z, vertex.z);
            max_z = f32::max(max_z, vertex.z);
        }

        let epsilon: f32 = 0.0001;

        self.nodes.push(Bbox::new(
            Interval::new(min_x - epsilon, max_x + epsilon),
            Interval::new(min_y - epsilon, max_y + epsilon),
            Interval::new(min_z - epsilon, max_z + epsilon),
        ));
        self.meshes.push(mesh);
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool {
        hit_info_out.t = f32::INFINITY;
        let mut i: usize = 0;

        while i < self.meshes.len() {
            if self.nodes[i].intersects(ray) {
                self.meshes[i].hit(ray, interval, hit_info_out);
            }

            i += 1;
        }

        if hit_info_out.t < f32::INFINITY {
            return true;
        }

        return false;
    }
}

impl TriangleMesh {
    pub fn new(material: Material) -> TriangleMesh {
        TriangleMesh {
            indices: Vec::new(),
            vertices: Vec::new(),
            normals: Vec::new(),
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

        let a = self.vertices[vertex_index_1 as usize];
        let b = self.vertices[vertex_index_2 as usize];
        let c = self.vertices[vertex_index_3 as usize];

        let mut normal = (b - a).cross(c - a);
        normal = normal / normal.magnitude();
        self.normals.push(normal);
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, ray: Ray, interval: Interval, hit_info_out: &mut HitInfo) -> bool {
        let mut i: usize = 0;
        let mut hit = false;

        while i < self.indices.len() {
            let a = self.vertices[self.indices[i] as usize];
            let b = self.vertices[self.indices[i + 1] as usize];
            let c = self.vertices[self.indices[i + 2] as usize];

            let normal = self.normals[i / 3];

            hit |= triangle_hit(a, b, c, normal, ray, interval, hit_info_out);

            i += 3;
        }
        if hit {
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
    normal: Vec3,
    ray: Ray,
    interval: Interval,
    hit_info_out: &mut HitInfo,
) -> bool {
    let d = normal.dot(ray.dir());

    if d == 0.0 {
        return false;
    }

    let t = (normal.dot(a) - normal.dot(ray.origin())) / (d);

    if hit_info_out.t < t || !interval.contains(t) {
        return false;
    }

    let q = ray.at(t);

    if ((b - a).cross(q - a)).dot(normal) < 0.0 {
        return false;
    }
    if ((c - b).cross(q - b)).dot(normal) < 0.0 {
        return false;
    }
    if ((a - c).cross(q - c).dot(normal)) < 0.0 {
        return false;
    }
    if d < 0.0 {
        hit_info_out.normal = normal;
    } else {
        hit_info_out.normal = -normal;
    }

    hit_info_out.t = t;
    hit_info_out.point = q;

    true
}

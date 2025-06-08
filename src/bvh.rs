use std::simd;

struct BvhData {
    x_min: Vec<simd::f32x4>,
    x_max: Vec<simd::f32x4>,
    y_min: Vec<simd::f32x4>,
    y_max: Vec<simd::f32x4>,
    z_min: Vec<simd::f32x4>,
    z_max: Vec<simd::f32x4>,
}

pub struct BvhTree {
    node_data: BvhData,
}

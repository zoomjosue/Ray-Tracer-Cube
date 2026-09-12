use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::{dot, Vec3};

const EPSILON: f32 = 1e-4;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl RayIntersect for Plane {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let denominator = dot(&self.normal, ray_direction);
        if denominator.abs() < EPSILON {
            return None;
        }

        let distance = dot(&(self.point - ray_origin), &self.normal) / denominator;
        if distance <= EPSILON {
            return None;
        }

        Some(Intersect {
            point: ray_origin + ray_direction * distance,
            normal: self.normal,
            distance,
            material: self.material,
        })
    }
}

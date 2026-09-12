use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

const EPSILON: f32 = 1e-4;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: Vec3, material: Material) -> Self {
        let half_size = size * 0.5;
        Cube {
            min: center - half_size,
            max: center + half_size,
            material,
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let origins = [ray_origin.x, ray_origin.y, ray_origin.z];
        let directions = [ray_direction.x, ray_direction.y, ray_direction.z];
        let minimums = [self.min.x, self.min.y, self.min.z];
        let maximums = [self.max.x, self.max.y, self.max.z];

        let mut near_distance = f32::NEG_INFINITY;
        let mut far_distance = f32::INFINITY;
        let mut near_normal = Vec3::zeros();
        let mut far_normal = Vec3::zeros();

        for axis in 0..3 {
            if directions[axis].abs() < EPSILON {
                if origins[axis] < minimums[axis] || origins[axis] > maximums[axis] {
                    return None;
                }
                continue;
            }

            let mut axis_near = (minimums[axis] - origins[axis]) / directions[axis];
            let mut axis_far = (maximums[axis] - origins[axis]) / directions[axis];
            let mut axis_near_normal = axis_normal(axis, -1.0);
            let mut axis_far_normal = axis_normal(axis, 1.0);

            if axis_near > axis_far {
                std::mem::swap(&mut axis_near, &mut axis_far);
                std::mem::swap(&mut axis_near_normal, &mut axis_far_normal);
            }

            if axis_near > near_distance {
                near_distance = axis_near;
                near_normal = axis_near_normal;
            }
            if axis_far < far_distance {
                far_distance = axis_far;
                far_normal = axis_far_normal;
            }

            if near_distance > far_distance {
                return None;
            }
        }

        let (distance, normal) = if near_distance > EPSILON {
            (near_distance, near_normal)
        } else if far_distance > EPSILON {
            (far_distance, far_normal)
        } else {
            return None;
        };

        Some(Intersect {
            point: ray_origin + ray_direction * distance,
            normal,
            distance,
            material: self.material,
        })
    }
}

fn axis_normal(axis: usize, sign: f32) -> Vec3 {
    match axis {
        0 => Vec3::new(sign, 0.0, 0.0),
        1 => Vec3::new(0.0, sign, 0.0),
        _ => Vec3::new(0.0, 0.0, sign),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    fn cube() -> Cube {
        Cube::new(
            Vec3::zeros(),
            Vec3::new(2.0, 2.0, 2.0),
            Material::new(Color::new(18, 76, 35), 1.0),
        )
    }

    #[test]
    fn intersects_front_face() {
        let hit = cube()
            .ray_intersect(&Vec3::new(0.0, 0.0, 5.0), &Vec3::new(0.0, 0.0, -1.0))
            .expect("the ray should hit the cube");

        assert!((hit.distance - 4.0).abs() < EPSILON);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn exits_when_ray_starts_inside() {
        let hit = cube()
            .ray_intersect(&Vec3::zeros(), &Vec3::new(1.0, 0.0, 0.0))
            .expect("the ray should exit the cube");

        assert!((hit.distance - 1.0).abs() < EPSILON);
        assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn misses_when_parallel_to_a_face() {
        assert!(cube()
            .ray_intersect(&Vec3::new(2.0, 0.0, 5.0), &Vec3::new(0.0, 0.0, -1.0))
            .is_none());
    }
}

// mod Hit;

use crate::hit::*;
use crate::ray::Ray;

use crate::vec3::*;

pub struct Sphere
{
    radius: f32,
    center: point3,
}

impl Sphere
{
    pub fn new(cen: point3, rad: f32) -> Self
    {
        Sphere { radius: rad, center: cen }
    }
}

impl Hit for Sphere
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>
    {
        let oc: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c: f32 = oc.length_squared() - self.radius * self.radius;

        let discriminant: f32 = half_b * half_b - a * c;
        if discriminant < 0.0 { return None; }
        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root
        {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root
            { return None; }
        }

        let p = ray.at(root);
        let mut rec = HitRecord{
            t: root,
            p: p,
            normal: Vec3::new(),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}
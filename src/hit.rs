// pub mod ray;

use crate::ray::*;
use crate::vec3::*;
pub struct HitRecord
{
    pub p: point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord
{
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) 
    {
        self.front_face = 
        if Vec3::dot(ray.direction(), *outward_normal) < 0.0 
        { true } else { false };
        
        self.normal = 
        if self.front_face 
        { *outward_normal } else { -*outward_normal };

    }
}
pub trait Hit
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct World {
    list: Vec<Box<dyn Hit>>
}

impl World {

    // pub fn new(list: Vec<Box<World>>) -> World {
    //     World { list }
    // }

    pub fn push(&mut self, hitable: impl Hit + 'static) {
        self.list.push(Box::new(hitable))
    }
}

impl Hit for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.list.iter() {
            if let Some(hit) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}
// pub type World = Vec<Box<dyn Hit="">>;
// pub type World = Vec<Box<dyn Hit="">>;

// impl Hit for World
// {
//     fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
//         let hit_anything = false;
//         let closest_so_far = t_max;

//         let mut temp_rec = None;

//         for object in self
//         {
//             if object.hit(r, t_min, t_max, rec)
//             {
//                 hit_anything = false;
//                 closest_so_far = temp_rec.t;
//                 temp_rec = Some(rec);
//             }
//         }
//     }
// }

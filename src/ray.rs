use crate::vec3::Vec3;

use crate::vec3::*;

#[derive(Clone, Copy)]
pub struct Ray
{
    orig: point3,
    dir: Vec3,
}

impl Ray
{
    pub fn new(origin: Vec3, direction: Vec3) -> Self
    {
        Self{
            orig: origin,
            dir: direction
        }
    }

    pub fn origin(&self) -> point3
    {
        self.orig
    } 

    pub fn direction(&self) -> Vec3
    {
        self.dir
    }

    pub fn at(&self,t: f32) -> point3
    {
        self.orig + t * self.dir
    }
}
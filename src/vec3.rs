

use std::{ops::{self, Index, IndexMut}, fmt::Display,};

pub(crate) use Vec3 as point3;

#[derive(Clone, Copy)]
pub struct Vec3
{
    e: [f32; 3], 
}

impl Vec3 
{
    pub fn new() -> Self
    {
        Self{ e: [0.0; 3] }
    }

    pub fn new_with_values(e0: f32, e1: f32, e2: f32) -> Self
    {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }

    pub fn length(&self) -> f32 
    { 
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 
    { 
        self.e[0] * self.e[0] +
        self.e[1] * self.e[1] +
        self.e[2] * self.e[2]
    }

    pub fn dot(v: Vec3, u: Vec3) -> f32
    {
        v.e[0] * u.e[0] +
        v.e[1] * u.e[1] +
        v.e[2] * u.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3
    {
        Vec3 { e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0]
        ] }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 
    { 
        let len = v.length();
        v / len 
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }
}

impl Display for Vec3
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

impl Index<usize> for Vec3
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Neg for Vec3
{
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3{
            e:[
               -self.e[0],
               -self.e[1],
               -self.e[2]
            ]
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Self::Output
    {
        Vec3 { e: [
            self.e[0] - v.e[0],
            self.e[1] - v.e[1],
            self.e[2] - v.e[2]
        ] }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Self::Output
    {
        Vec3 { e: [
            self.e[0] + v.e[0],
            self.e[1] + v.e[1],
            self.e[2] + v.e[2]
        ] }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output
    {
        Vec3 { e: [
            self.e[0] * v.e[0],
            self.e[1] * v.e[1],
            self.e[2] * v.e[2]
        ] }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: f32) -> Self::Output
    {
        Vec3 { 
            e: [
                self.e[0] * v,
                self.e[1] * v,
                self.e[2] * v
            ]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, v: f32) -> Self::Output
    {
        Vec3 { e: [
            self.e[0] / v,
            self.e[1] / v,
            self.e[2] / v
        ] }
    }
}

impl ops::Mul<Vec3> for f32
{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
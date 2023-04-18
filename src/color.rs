// use crate::vec3::color;
use crate::vec3::Vec3;
pub(crate) type Color = Vec3;

pub fn write_color(pixel_color: &Color)
{
    println!(
        "{} {} {}", 
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    );
}

// use crate::vec3::color;
use crate::vec3::Vec3;
pub(crate) type Color = Vec3;


impl Color
{
    pub fn write_color(&self, samples_per_pixel: u64) -> String
    {
        let ir = (256.0 * (self.x() / (samples_per_pixel as f32)).clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self.y() / (samples_per_pixel as f32)).clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self.z() / (samples_per_pixel as f32)).clamp(0.0, 0.999)) as u64;
    
        format!("{} {} {}", ir, ig, ib)
    
        // println!(
        //     "{} {} {}", 
        //     (255.999 * pixel_color.x()) as i32,
        //     (255.999 * pixel_color.y()) as i32,
        //     (255.999 * pixel_color.z()) as i32
        // );
    }
        
}
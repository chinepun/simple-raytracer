pub mod vec3;
pub mod color;

use crate::color::*;

use std::{io::{stdout, Write}, ops::Div};
fn main() 
{
    const IMAGE_HEIGHT: usize = 256;
    const IMAGE_WIDTH: usize = 256;

    let rows= [0; IMAGE_HEIGHT];
    let columns = [0; IMAGE_WIDTH];

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for (j, _) in rows.iter().enumerate()
    {
        if let Err(_) = stdout().flush() {
            eprintln!("\rScanlines remaining: {}", j)
        };
        for (i, _) in columns.iter().enumerate()
        {
            let r = (i as f32)
                .div((IMAGE_WIDTH - 1) as f32);
            let g = (j as f32)
                .div((IMAGE_HEIGHT - 1) as f32);
            let b = 0.25;

            let pixel_color: color::Color = color::Color::new_with_values(
                r,
                g,
                b
            );

            write_color(&pixel_color);
        }
    }
}

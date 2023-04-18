use std::ops::Div;
use std::io::{stdout, Write};
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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            // println!("{} {} {} \n", ir, ig, ib);
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

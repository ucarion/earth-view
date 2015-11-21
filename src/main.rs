extern crate byteorder;
extern crate cgmath;
extern crate image;

use std::fs::File;

mod elevation;
mod color;

fn main() {
    const NUM_COLS: u32 = 10800;
    const NUM_ROWS: u32 = 6000;

    let elev_data_file = File::open("elevation_data/g10g").unwrap();
    let mut img_buf = image::ImageBuffer::new(NUM_COLS as u32, NUM_ROWS as u32);
    let mut elevation_iter = elevation::ElevationIterator(elev_data_file);

    println!("Staring...");

    for y in 0..NUM_ROWS {
        for x in 0..NUM_COLS {
            let elevation = elevation_iter.next().unwrap();
            let (r, g, b) = color::find_color(elevation);

            img_buf.put_pixel(x, y, image::Rgb([r, g, b]));
        }

        println!("{}", y);
    }

    println!("Writing ...");

    let mut img_out = File::create("output.png").unwrap();
    image::ImageRgb8(img_buf).save(&mut img_out, image::PNG).unwrap();
}

extern crate byteorder;
extern crate cgmath;
extern crate image;

use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt};
use cgmath::{EuclideanVector, Vector3};

fn main() {
    const NUM_COLS: u32 = 10800;
    const NUM_ROWS: u32 = 6000;
    const MAX_HEIGHT: f64 = 10000.0;

    let mut elev_data_file = File::open("elevation_data/j10g").unwrap();
    let mut img_buf = image::ImageBuffer::new(NUM_COLS as u32, NUM_ROWS as u32);
    let mut max_elev: f64 = 0.0;

    println!("Staring...");

    for y in 0..NUM_ROWS {
        for x in 0..NUM_COLS {
            let elevation = elev_data_file.read_i16::<LittleEndian>().unwrap() as f64;
            max_elev = if elevation > max_elev { elevation } else { max_elev };

            let (r, g, b) = if elevation > 0.0 {
                find_color(elevation)
            } else {
                (0, 0, 255)
            };

            img_buf.put_pixel(x, y, image::Rgb([r, g, b]));
            // println!("({:?}, {:?}) - {:?} - {:?}", x, y, elevation, color);
        }

        println!("{}", y);
    }

    println!("Writing ...");

    let mut img_out = File::create("output.png").unwrap();
    image::ImageRgb8(img_buf).save(&mut img_out, image::PNG).unwrap();
}

fn find_color(elevation: f64) -> (u8, u8, u8) {
    let color_ramp = [
        (0.0,     Vector3::new(26.0, 150.0, 65.0)),
        (100.0,   Vector3::new(166.0, 217.0, 106.0)),
        (500.0,   Vector3::new(255.0, 255.0, 191.0)),
        (1000.0,  Vector3::new(253.0, 174.0, 97.0)),
        (2000.0,  Vector3::new(215.0, 25.0, 28.0)),
        (5000.0,  Vector3::new(255.0, 255.0, 255.0)),
        (10000.0, Vector3::new(255.0, 255.0, 255.0))
    ];

    for i in 1..color_ramp.len() {
        let (left_height, left_color) = color_ramp[i - 1];
        let (right_height, right_color) = color_ramp[i];

        if left_height < elevation && elevation < right_height {
            let lerp_amount = (elevation - left_height) / (right_height - left_height);
            return vec3_to_rgb(&left_color.lerp(&right_color, lerp_amount));
        }
    }

    let (_, last_color) = color_ramp[color_ramp.len() - 1];
    vec3_to_rgb(&last_color)
}

fn vec3_to_rgb(vec: &Vector3<f64>) -> (u8, u8, u8) {
    let r = (vec.x * 255.0).floor() as u8;
    let g = (vec.y * 255.0).floor() as u8;
    let b = (vec.z * 255.0).floor() as u8;

    (r, g, b)
}

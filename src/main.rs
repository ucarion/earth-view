extern crate byteorder;
extern crate cgmath;
extern crate image;
extern crate itertools;

mod elevation;
mod color;

use std::io::{BufReader};
use std::fs::File;

use byteorder::{LittleEndian, WriteBytesExt};

use elevation::Elevation;

fn main() {
//    let original_width = 10800;
//    let original_height = 5400;
//
//    let new_width = original_width / 2;
//    let new_height = original_height / 2;
//
//    let mut file_out = File::create(format!("elevation_data/full_data-{}x{}", new_width, new_height)).unwrap();
//
//    let mut reader1 = elevation_iter(&format!("elevation_data/full_data-{}x{}", original_width, original_height));
//    let mut reader2 = elevation_iter(&format!("elevation_data/full_data-{}x{}", original_width, original_height));
//
//    for y in 0..new_height {
//        let mut buckets = vec![(Elevation::Sea, Elevation::Sea, Elevation::Sea, Elevation::Sea); new_width];
//
//        let size_i16 = 2;
//        reader1.seek(SeekPosition::Start((y * 2) * original_width * size_i16)).unwrap();
//        reader2.seek(SeekPosition::Start((y * 2 + 1) * original_width * size_i16)).unwrap();
//
//        for x in 0..original_height {
//            buckets[x / 2].0 = Elevation::new(reader1.read_i16::<LittleEndian>().unwrap());
//            buckets[x / 2].1 = Elevation::new(reader1.read_i16::<LittleEndian>().unwrap());
//            buckets[x / 2].2 = Elevation::new(reader2.read_i16::<LittleEndian>().unwrap());
//            buckets[x / 2].3 = Elevation::new(reader2.read_i16::<LittleEndian>().unwrap());
//        }
//
//        for (a, b, c, d) in buckets {
//            let mut sum_land_elevation = 0;
//            let mut num_water = 0;
//
//            for x in [a, b, c, d].iter() {
//                match *x {
//                    Elevation::Sea => num_water += 1,
//                    Elevation::Land { elevation } => sum_land_elevation += elevation as i16
//                }
//            }
//
//            let out = if num_water >= 2 {
//                Elevation::Sea.to_raw()
//            } else {
//                sum_land_elevation / (4 - num_water)
//            };
//
//            file_out.write_i16::<LittleEndian>(out).unwrap();
//        }
//
//        println!("{}", y);
//    }
//

    let path = "elevation_data/derived/transformed/full-10800x5400";
    let width = 10800;
    let height = 5400;
    // let path = "elevation_data/derived/full_data-5400x2700";
    let mut reader = elevation_iter(path);
    let mut img_buf = image::ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let elevation = reader.next().unwrap();
            let (r, g, b) = color::find_color(elevation);
            img_buf.put_pixel(x, y, image::Rgb([r, g, b]));
        }

        println!("{}", y);
    }

    let mut img_out = File::create("output.png").unwrap();
    image::ImageRgb8(img_buf).save(&mut img_out, image::PNG).unwrap();
}

fn elevation_iter(path: &str) -> elevation::ElevationIterator<BufReader<File>> {
    elevation::ElevationIterator(BufReader::new(File::open(path).unwrap()))
}

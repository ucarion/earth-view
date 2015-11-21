extern crate byteorder;
extern crate cgmath;
extern crate image;
extern crate itertools;

mod elevation;
// mod color;

use std::io::{BufReader, Seek, SeekFrom, Write};
use std::fs::File;
use std::mem;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use elevation::Elevation;

fn main() {
    let original_width = 10800;
    let original_height = 5400;

    let new_width = original_width / 2;
    let new_height = original_height / 2;

    let mut file_out = File::create(format!("elevation_data/full_data-{}x{}", new_width, new_height)).unwrap();

    let mut reader1 = elevation_iter(&format!("elevation_data/full_data-{}x{}", original_width, original_height));
    let mut reader2 = elevation_iter(&format!("elevation_data/full_data-{}x{}", original_width, original_height));
    reader2.seek(SeekFrom::Start(original_width)).unwrap();

    let i16_width = mem::size_of::<i16>() as u64;

    for dest_y in 0..original_width / i16_width {
        for _dest_x in 0..original_height / i16_width {
            let a = Elevation::new(reader1.read_i16::<LittleEndian>().unwrap());
            let b = Elevation::new(reader1.read_i16::<LittleEndian>().unwrap());
            let c = Elevation::new(reader2.read_i16::<LittleEndian>().unwrap());
            let d = Elevation::new(reader2.read_i16::<LittleEndian>().unwrap());
            let mut average = 0;
            let mut num_water = 0;

            for elev in [a, b, c, d].iter() {
                match *elev {
                    Elevation::Land { elevation } => average += elevation as i16,
                    Elevation::Sea => num_water += 1
                }
            }

            let out = if num_water >= 2 {
                Elevation::Sea.to_raw()
            } else {
                average / (4 - num_water)
            };

            file_out.write_i16::<LittleEndian>(out).unwrap();
        }

        let offset = new_width * i16_width;
        let i = reader1.seek(SeekFrom::Current(offset as i64)).unwrap();
        let j = reader2.seek(SeekFrom::Current(offset as i64)).unwrap();
        println!("Finished with row: {} -- {} {}", dest_y, i, j);
    }
}

fn elevation_iter(path: &str) -> BufReader<File> {
    BufReader::new(File::open(path).unwrap())
}

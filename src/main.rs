extern crate byteorder;
extern crate cgmath;
extern crate image;

use std::io::{BufReader, Write};
use std::fs::File;

use byteorder::{LittleEndian, WriteBytesExt};

mod elevation;
mod color;

fn main() {
    let a = elevation_iter("elevation_data/a10g");
    let b = elevation_iter("elevation_data/b10g");
    let c = elevation_iter("elevation_data/c10g");
    let d = elevation_iter("elevation_data/d10g");
    let e = elevation_iter("elevation_data/e10g");
    let f = elevation_iter("elevation_data/f10g");
    let g = elevation_iter("elevation_data/g10g");
    let h = elevation_iter("elevation_data/h10g");
    let i = elevation_iter("elevation_data/i10g");
    let j = elevation_iter("elevation_data/j10g");
    let k = elevation_iter("elevation_data/k10g");
    let l = elevation_iter("elevation_data/l10g");
    let m = elevation_iter("elevation_data/m10g");
    let n = elevation_iter("elevation_data/n10g");
    let o = elevation_iter("elevation_data/o10g");
    let p = elevation_iter("elevation_data/p10g");

    // Arrangement from http://www.ngdc.noaa.gov/mgg/topo/gltiles.html
    let elevations = vec![
        ((10800, 4800), vec![a, b, c, d]),
        ((10800, 6000), vec![e, f, g, h]),
        ((10800, 6000), vec![i, j, k, l]),
        ((10800, 4800), vec![m, n, o, p])
    ];

    println!("Starting ...");

    let mut file_out = File::create("elevation_data/full_data").unwrap();

    for ((width, height), mut data_row) in elevations.into_iter() {
        for _ in 0..height {
            for source in &mut data_row {
                for _ in 0..width {
                    let elevation = source.next().unwrap().to_raw();
                    file_out.write_i16::<LittleEndian>(elevation).unwrap();
                }
            }

            println!("Finished a row");
        }
    }
}

fn elevation_iter(path: &str) -> elevation::ElevationIterator<BufReader<File>> {
    elevation::ElevationIterator(BufReader::new(File::open(path).unwrap()))
}

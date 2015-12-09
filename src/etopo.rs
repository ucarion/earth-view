use std::io::BufReader;
use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn process_raw_data() {
    const OFFSET: i16 = 10803;

    let mut input = BufReader::new(File::open("etopo/etopo1_ice_c_i2.bin").unwrap());
    let mut output = File::create("heightmap.bin").unwrap();

    let mut min = i16::max_value();
    let mut max = i16::min_value();

    let mut count = 0;

    while let Ok(elevation) = input.read_i16::<LittleEndian>() {
        let to_write = elevation + OFFSET;
        output.write_u16::<LittleEndian>(to_write as u16).unwrap();

        if elevation < min {
            min = elevation;
        }

        if elevation > max {
            max = elevation;
        }

        count += 1;
    }

    println!("{} {}", min, max);
    println!("{} {}", min + OFFSET, max + OFFSET);
    println!("{}", count);
}

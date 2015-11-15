use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

pub enum Elevation {
    Land { elevation: f64 },
    Sea
}

pub struct ElevationIterator<T>(pub T);

impl<T: Read> Iterator for ElevationIterator<T> {
    type Item = Elevation;

    fn next(&mut self) -> Option<Elevation> {
        self.0.read_i16::<LittleEndian>().ok().map(|elevation| {
            if elevation == -500 {
                Elevation::Sea
            } else {
                Elevation::Land { elevation: elevation as f64 }
            }
        })
    }
}

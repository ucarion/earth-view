use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

pub enum Elevation {
    Land { elevation: f64 },
    Sea
}

impl Elevation {
    pub fn to_raw(&self) -> i16 {
        match *self {
            Elevation::Land { elevation } => elevation.round() as i16,
            Elevation::Sea => -500
        }
    }
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

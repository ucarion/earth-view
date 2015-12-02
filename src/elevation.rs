use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Clone, Copy, Debug)]
pub enum Elevation {
    Land { elevation: f64 },
    Sea
}

impl Elevation {
    pub fn to_raw(&self) -> i16 {
        match *self {
            Elevation::Land { elevation } => elevation as i16,
            Elevation::Sea => -500
        }
    }

    pub fn new(raw: i16) -> Elevation {
        match raw {
            -500 => Elevation::Sea,
            _ => Elevation::Land { elevation: raw as f64 }
        }
    }
}

pub struct ElevationIterator<T>(pub T);

impl<T: Read> Iterator for ElevationIterator<T> {
    type Item = Elevation;

    fn next(&mut self) -> Option<Elevation> {
        self.0.read_i16::<LittleEndian>().ok().map(Elevation::new)
    }
}

#![macro_use]

pub enum ScaleSize {
    Diatonic,
    Blues,
    Pentatonic,
}
pub struct ScaleIntervals {
    pub name: String,
    pub is_minor: bool,
    pub size: ScaleSize,
}
macro_rules! music_intervals {
    ($name:literal, $minor:literal, $size:ident) => {
        ScaleIntervals {
            name: $name.to_string(),
            is_minor: $minor,
            size: ScaleSize::$size,
        }
    };
}
impl ScaleIntervals {
    pub fn intervals(&self) -> Vec<usize>{
        return match self.size { 
            ScaleSize::Diatonic => match self.is_minor {
                false => vec!(2,2,1,2,2,2,1),
                true => vec!(2,1,2,2,1,2,2),
            },
            ScaleSize::Blues => match self.is_minor {
                false => vec!(2,1,1,3,2,3),
                true => vec!(3,2,1,1,3,2),
            },
            ScaleSize::Pentatonic => match self.is_minor {
                false => vec!(2,2,3,2,3),
                true => vec!(3,2,2,3,2),
            },
        }
    }
    pub fn notes(&self) -> Vec<usize> {
        let mut n = 0;
        let mut notes:Vec<usize> = vec!(0);
        for i in self.intervals() {
            n += i;
            notes.push(n);
        }
        notes
    }
}
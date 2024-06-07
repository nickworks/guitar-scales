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
    pub note_letters: Vec<String>,
    pub note_numbers: Vec<String>,
    pub total_tones: usize,
}
macro_rules! music_intervals {
    ($name:literal, $minor:literal, $size:ident) => {
        ScaleIntervals {
            total_tones: 12,
            name: $name.to_string(),
            is_minor: $minor,
            size: ScaleSize::$size,
            note_letters: vec![
                "C".to_string(),
                "C#".to_string(),
                "D".to_string(),
                "D#".to_string(),
                "E".to_string(),
                "F".to_string(),
                "F#".to_string(),
                "G".to_string(),
                "G#".to_string(),
                "A".to_string(),
                "A#".to_string(),
                "B".to_string(),
            ],
            note_numbers: vec![
                "R".to_string(),
                "b2".to_string(),
                "2".to_string(),
                "b3".to_string(),
                "3".to_string(),
                "4".to_string(),
                "b5".to_string(),
                "5".to_string(),
                "b6".to_string(),
                "6".to_string(),
                "b7".to_string(),
                "7".to_string(),
            ]
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
    pub fn is_note_in_scale(&self, mut n:i16) -> bool {
        while n < 0 {
            n += self.total_tones as i16;
        }
        n %= self.total_tones as i16;
        self.notes().iter().any(|note| *note == n as usize)
    }
    pub fn get_note_letter(&self, n:usize) -> String {
        String::from(&self.note_letters[n % self.total_tones])
    }
    pub fn get_note_number(&self, n:usize) -> String {
        String::from(&self.note_numbers[n % self.total_tones])
    }
    pub fn get_total_tones(&self) -> usize {
        self.total_tones
    }
}
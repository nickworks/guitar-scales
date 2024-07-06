#![macro_use]

use egui::{Color32, TextBuffer};

use crate::NoteMarker;

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
    pub blank: String,
}
pub struct Bubble {
    pub color: Color32,
    pub text: String,
}
macro_rules! music_intervals {
    ($name:literal, $minor:literal, $size:ident) => {
        ScaleIntervals {
            total_tones: 12,
            name: $name.to_string(),
            is_minor: $minor,
            size: ScaleSize::$size,
            blank: "".to_string(),
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
        // wrap n, so that 0 <= n < self.total_tones
        while n < 0 {
            n += self.total_tones as i16;
        }
        n %= self.total_tones as i16;
        // returns true if any element matches n
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
    pub fn get_note_color(&self, note_0_to_11:usize) -> Color32 {
        match self.is_minor {
            true => match note_0_to_11 {
                0 => Color32::WHITE,
                6 => Color32::BLUE,
                3|7 => Color32::GOLD,
                2|5|8|10 => Color32::RED,
                _ => Color32::GRAY,
            },
            false => match note_0_to_11 {
                0 => Color32::WHITE,
                3 => Color32::BLUE,
                4|7 => Color32::GOLD,
                2|5|9|11 => Color32::RED,
                _ => Color32::DARK_GRAY,
            }
        }
    }
    pub fn get_bubble(&self, note_as_int:usize, key:usize, note_marker:NoteMarker) -> Bubble {
        let note_0_to_11 = (note_as_int + 12).checked_sub(key).unwrap_or(0)%12;
        let caption_letter = self.get_note_letter(note_as_int);
        let caption_number = self.get_note_number(note_0_to_11);
        let is_note_in_scale = self.is_note_in_scale(note_as_int as i16 - key as i16);
        let blank = "".to_string();
        return Bubble {
            color: match note_marker {
                NoteMarker::Triads => match self.is_minor {
                    true => match note_0_to_11 { 0|3|7 => self.get_note_color(note_0_to_11), _ => Color32::TRANSPARENT, },
                    false => match note_0_to_11 { 0|4|7 => self.get_note_color(note_0_to_11), _ => Color32::TRANSPARENT, },
                },
                NoteMarker::AllNotes => self.get_note_color(note_0_to_11),
                NoteMarker::NotesInKey | NoteMarker::Numbers => match is_note_in_scale {
                    false => Color32::TRANSPARENT,
                    true  => self.get_note_color(note_0_to_11),
                },
            },
            text: match note_marker {
                NoteMarker::Triads => match self.is_minor {
                    true => match note_0_to_11 { 0|3|7 => caption_number, _ => blank, },
                    false => match note_0_to_11 { 0|4|7 => caption_number, _ => blank, }
                },
                NoteMarker::AllNotes => caption_letter,
                NoteMarker::NotesInKey => match is_note_in_scale {
                    true => caption_letter,
                    false => blank,
                },
                NoteMarker::Numbers => match is_note_in_scale {
                    true => caption_number,
                    false => blank,
                },
            },
        };
    }
}
#![macro_use]

use egui::Color32;
use strum_macros::EnumIter;

use crate::NoteMarker;

#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum ScaleSize {
    Diatonic,
    Blues,
    Pentatonic,
}
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum ScaleType {
    Minor,
    Major,
}
pub struct Scale {
    pub typ: ScaleType,
    pub siz: ScaleSize,
    pub key: usize,
}
pub struct Bubble {
    pub color: Color32,
    pub text: String,
}
pub const TOTAL_TONES:usize = 12;
pub const NOTE_LETTERS: [&'static str; TOTAL_TONES] = ["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"];
const NOTE_NUMBERS: [&'static str; TOTAL_TONES] = ["R","b2","2","b3","3","4","b5","5","b6","6","b7","7"];

impl Scale {
    pub fn notes(&self) -> Vec<usize>{
        return match self.typ {
            ScaleType::Minor => match self.siz {
                ScaleSize::Diatonic => vec!(0,2,3,5,7,8,10),
                ScaleSize::Blues => vec!(0,3,5,6,7,10),
                ScaleSize::Pentatonic => vec!(0,3,5,7,10),
            },
            ScaleType::Major => match self.siz {
                ScaleSize::Diatonic => vec!(0,2,4,5,7,9,11),
                ScaleSize::Blues => vec!(0,2,3,4,7,9),
                ScaleSize::Pentatonic => vec!(0,2,4,7,9),
            },
        }
    }
    pub fn is_note_in_scale(&self, mut n:i16) -> bool {
        // wrap n, so that 0 <= n < TOTAL_TONES
        while n < 0 {
            n += TOTAL_TONES as i16;
        }
        n %= TOTAL_TONES as i16;
        // returns true if any element matches n
        self.notes().iter().any(|note| *note == n as usize)
    }
    pub fn get_note_letter(&self, n:usize) -> String {
        String::from(NOTE_LETTERS[n % TOTAL_TONES])
    }
    pub fn get_note_number(&self, n:usize) -> String {
        String::from(NOTE_NUMBERS[n % TOTAL_TONES])
    }
    pub fn get_note_color(&self, note_0_to_11:usize) -> Color32 {
        match self.typ {
            ScaleType::Minor => match note_0_to_11 {
                0 => Color32::WHITE,
                6 => Color32::BLUE,
                3|7 => Color32::GOLD,
                2|5|8|10 => Color32::RED,
                _ => Color32::GRAY,
            },
            ScaleType::Major => match note_0_to_11 {
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
                NoteMarker::Triads => match self.typ {
                    ScaleType::Minor => match note_0_to_11 { 0|3|7 => self.get_note_color(note_0_to_11), _ => Color32::TRANSPARENT, },
                    ScaleType::Major => match note_0_to_11 { 0|4|7 => self.get_note_color(note_0_to_11), _ => Color32::TRANSPARENT, },
                },
                NoteMarker::AllNotes => self.get_note_color(note_0_to_11),
                NoteMarker::NotesInKey | NoteMarker::Numbers => match is_note_in_scale {
                    false => Color32::TRANSPARENT,
                    true  => self.get_note_color(note_0_to_11),
                },
            },
            text: match note_marker {
                NoteMarker::Triads => match self.typ {
                    ScaleType::Minor => match note_0_to_11 { 0|3|7 => caption_number, _ => blank, },
                    ScaleType::Major => match note_0_to_11 { 0|4|7 => caption_number, _ => blank, }
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
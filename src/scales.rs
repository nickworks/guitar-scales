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
    pub text_color: Color32,
}
impl Bubble {
    pub fn blank() -> Self {
        Self {
            color: Color32::TRANSPARENT,
            text_color: Color32::BLACK,
            text: "".to_string(),
        }
    }
    pub fn new(colors:(Color32,Color32), txt:String) -> Bubble{
        Self {
            color: colors.0,
            text_color: colors.1,
            text: txt.to_string(),
        }
    }
}
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum NoteType {
    Root,
    Triad,
    Blue,
    InScale,
    NotInScale,
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
    pub fn color_lookup(dark_mode:bool, typ:NoteType) -> (Color32, Color32) {
        match dark_mode {
            true => match typ {
                NoteType::Root => (Color32::WHITE, Color32::BLACK),
                NoteType::Blue => (Color32::BLUE, Color32::BLACK),
                NoteType::Triad => (Color32::GOLD, Color32::BLACK),
                NoteType::InScale => (Color32::RED, Color32::BLACK),
                _ => (Color32::DARK_GRAY, Color32::BLACK),
            },
            false => match typ {
                NoteType::Root => (Color32::BLACK, Color32::WHITE),
                NoteType::Blue => (Color32::BLUE, Color32::BLACK),
                NoteType::Triad => (Color32::GOLD, Color32::BLACK),
                NoteType::InScale => (Color32::RED, Color32::WHITE),
                _ => (Color32::DARK_GRAY, Color32::BLACK),
            },
        }
    }
    pub fn get_note_type(&self, note_0_to_11:usize) -> NoteType {
        match self.typ {
            ScaleType::Minor => match note_0_to_11 {
                0 => NoteType::Root,
                6 => NoteType::Blue,
                3|7 => NoteType::Triad,
                2|5|8|10 => NoteType::InScale,
                _ => NoteType::NotInScale,
            },
            ScaleType::Major => match note_0_to_11 {
                0 => NoteType::Root,
                3 => NoteType::Blue,
                4|7 => NoteType::Triad,
                2|5|9|11 => NoteType::InScale,
                _ => NoteType::NotInScale,
            }
        }
    }
    pub fn get_bubble(&self, dark_mode:bool, note_as_int:usize, key:usize, note_marker:NoteMarker) -> Bubble {
        let note_0_to_11 = (note_as_int + 12).checked_sub(key).unwrap_or(0)%12;
        let is_note_in_scale = self.is_note_in_scale(note_as_int as i16 - key as i16);
        
        let typ = self.get_note_type(note_0_to_11);
        let colors = Scale::color_lookup(dark_mode, typ);

        let bubble_letter = Bubble::new(colors, self.get_note_letter(note_as_int));
        let bubble_number = Bubble::new(colors, self.get_note_number(note_0_to_11));
        let bubble_debug = Bubble::new(colors, note_as_int.to_string());

        return match is_note_in_scale {
            false => {
                match note_marker {
                    NoteMarker::AllNotes => bubble_letter,
                    NoteMarker::Debug => bubble_debug,
                    _ => Bubble::blank(),
                }
            },
            true => match note_marker {
                NoteMarker::Triads => match typ {
                    NoteType::Root | NoteType::Triad => bubble_number,
                    _ => Bubble::blank(),
                },
                NoteMarker::AllNotes | NoteMarker::Letters => bubble_letter,
                NoteMarker::Numbers => bubble_number,
                NoteMarker::Debug => bubble_debug,
            },
        };
    }
}
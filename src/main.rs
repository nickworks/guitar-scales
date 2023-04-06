#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::*;
use egui::*;
use egui_extras::{StripBuilder, Size};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod guitar;
use guitar::*;

#[cfg(not(target_arch = "wasm32"))]
fn main(){
    let _result = eframe::run_native(
        "guitar-scales",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(FretboardApp::new())),
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();
    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    // start rendering the app in canvas#guitar-scales:
    let _result = eframe::start_web(
        "guitar-scales",
        eframe::WebOptions::default(),
        Box::new(|_cc| Box::new(MyEGuiApp::new())),
    );

    ()
}
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
enum FretMarker {
    None,
    Dots,
    Numbers,
}
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
enum NoteMarker {
    Notes,
    NotesInKey,
    Dots,
}

struct FretboardApp {
    strings: Vec<usize>,
    frets: usize,
    scale_num: usize,
    scale_key: usize,
    scales: Vec<ScaleIntervals>,
    notes: Vec<String>,
    fret_marks: FretMarker,
    note_marks: NoteMarker,
}
impl FretboardApp {
    fn new() -> Self {
        Self::default()
    }
    pub fn lookup_note_num(&self, string:&String) -> usize {
        self.notes.iter().position(|n| n == string).unwrap()
    }
    pub fn lookup_note_str(&self, n:usize) -> String {
        String::from(&self.notes[n % self.notes.len()])
    }
    pub fn lookup_scale_num(&self, string:&String) -> usize {
        self.scales.iter().position(|n| n.name == *string).unwrap()
    }
    pub fn lookup_scale_str(&self, n:usize) -> String {
        match n {
            x if x >= self.scales.len() => {
                String::from("")
            },
            _ => {
                String::from(&self.scales[n].name)
            }
        }
    }
    pub fn is_note_root(&self, mut n:i16) -> bool {
        n -= self.scale_key as i16;
        while n < 0 {
            n += 12;
        }
        n %= 12;
        n == 0
    }
    pub fn is_note_in_scale(&self, mut n:i16) -> bool {
        n -= self.scale_key as i16;
        while n < 0 {
            n += 12;
        }
        n %= 12;
        self.scales[self.scale_num].notes.iter().any(|note| *note == n as usize)
    }
    pub fn fret_marker(&self, fret:usize) -> String {
        let f:i32 = fret.try_into().unwrap_or(0) - 1;
        match f {
            3|5|7|9|12|15|17|19 => {
                match self.fret_marks {
                    FretMarker::Numbers => f.to_string(),
                    FretMarker::Dots => String::from("•"),
                    FretMarker::None => String::from(""),
                }
            },
            _ => String::from("")
        }
    }
}
impl Default for FretboardApp {
    fn default() -> Self {
        Self {
            strings: vec![0,5,10,15,19,24],
            frets: 9,
            scale_key: 8,
            scale_num: 0,
            scales: vec![
                music_scale!("pentatonic minor", 0, 3, 5, 7, 10, 12),
                music_scale!("pentatonic major", 0, 2, 4, 7, 9, 12),
                music_scale!("minor", 0, 2, 3, 5, 7, 8, 10, 12),
                music_scale!("major", 0, 2, 4, 5, 7, 9, 11, 12),
                music_scale!("dorian", 0, 2, 3, 5, 7, 9, 10, 12),
                music_scale!("phrygian", 0, 1, 3, 5, 7, 8, 10, 12),
                music_scale!("lydian", 0, 2, 4, 6, 7, 9, 11, 12),
                music_scale!("blues minor", 0, 3, 5, 6, 7, 10, 12),
            ],
            notes: vec![
                "E ".to_string(),
                "F ".to_string(),
                "F#".to_string(),
                "G ".to_string(),
                "G#".to_string(),
                "A ".to_string(),
                "A#".to_string(),
                "B ".to_string(),
                "C ".to_string(),
                "C#".to_string(),
                "D ".to_string(),
                "D#".to_string(),
            ],
            fret_marks: FretMarker::Dots,
            note_marks: NoteMarker::NotesInKey,
        }
    }
}
impl eframe::App for FretboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let panel = egui::CentralPanel::default();
        panel.show(ctx, |ui|{

            
            ui.heading("Guitar Scales");
            // render toolbar:
            ui.horizontal(|ui|{

                ComboBox::from_id_source("scale")
                    .selected_text(format!("{} scale", &self.lookup_scale_str(self.scale_num)))
                    .width(200.0)
                    .show_ui(ui, |inner_ui| {
                        for i in 0..self.scales.len() {
                            inner_ui.selectable_value(
                                &mut self.scale_num,
                                i,
                                &self.scales[i].name
                            );
                        }
                    });

                ComboBox::from_id_source("key")
                    .selected_text(format!("key of {}", &self.lookup_note_str(self.scale_key)))
                    .width(50.0)
                    .show_ui(ui, |inner_ui| {
                        for i in 0..self.notes.len() {
                            inner_ui.selectable_value(
                                &mut self.scale_key,
                                i,
                                &self.notes[i]
                            );
                        }
                    });
                
                ComboBox::from_id_source("frets")
                    .selected_text(format!("{} frets", self.frets))
                    .width(100.0)
                    .show_ui(ui, |inner_ui| {
                        for i in 4..25 {
                            inner_ui.selectable_value(
                                &mut self.frets,
                                i,
                                format!("{}",i)
                            );
                        }
                    });
                
                ComboBox::from_id_source("fret_marks")
                    .selected_text(format!("{:?}", self.fret_marks))
                    .width(100.0)
                    .show_ui(ui, |inner_ui|{
                        for fm in FretMarker::iter() {
                            inner_ui.selectable_value(
                                &mut self.fret_marks,
                                fm,
                                format!("{:?}", fm),
                            );
                        }
                    });
                ComboBox::from_id_source("note_marks")
                    .selected_text(format!("{:?}", self.note_marks))
                    .width(100.0)
                    .show_ui(ui, |inner_ui|{
                        for nm in NoteMarker::iter() {
                            inner_ui.selectable_value(
                                &mut self.note_marks,
                                nm,
                                format!("{:?}", nm),
                            );
                        }
                    });
                
            });
            ui.add_space(10.0);
            
            egui::Grid::new("some_unique_id")
                //.striped(true)
                .show(ui, |ui| {

                let num_cols = self.frets + 2;
                // start first row

                // "draw" fret markers by inserting Labels into the grid.
                for fret in 0..num_cols {
                    ui.colored_label(Color32::WHITE, self.fret_marker(fret));
                }
                let y_top = ui.available_rect_before_wrap().bottom();
                ui.end_row();


                // "draw" each guitar string as a row in the grid
                for i in (0..*&self.strings.len()).rev() {
                    for fret in 0..(*&self.frets+1) {
                        
                        let note_as_int = &self.strings[i] + fret;
                        let caption = &self.notes[note_as_int%&self.notes.len()];

                        if fret == 0 {
                            ComboBox::from_id_source(i)
                            .selected_text(caption)
                            .show_ui(ui, |inner_ui| {
                                for ii in 0..self.notes.len() {
                                    inner_ui.selectable_value(&mut self.strings[i], ii, &self.notes[ii]);
                                }
                            });
                        }
                        
                        if self.is_note_in_scale(note_as_int as i16){
                            
                            let caption = match self.note_marks {
                                NoteMarker::Notes|NoteMarker::NotesInKey => caption,
                                NoteMarker::Dots => "•",
                                _ => "",
                            };
                            if self.is_note_root(note_as_int as i16) {
                                ui.colored_label(Color32::WHITE, caption);
                            } else {
                                ui.colored_label(Color32::GOLD, caption);
                            }
                        } else {
                            let caption = match self.note_marks {
                                NoteMarker::Notes => caption,
                                _ => "",
                            };
                            ui.small(RichText::new(caption).color(Color32::DARK_GRAY));
                        }

                    }
                    ui.end_row();
                }
                let y_bottom = ui.available_rect_before_wrap().top();
                
                // a collaction of shapes to hold the lines we want to draw:
                let mut shapes = Vec::new();

                for fret in 0..num_cols {
                    // "draw" the fret markers underneath the fret board
                    ui.colored_label(Color32::WHITE, self.fret_marker(fret));

                    if fret == 0 {
                        continue;
                    }
                    // calculate placement of lines, and
                    // store each line in shapes collection
                    let x = ui.available_rect_before_wrap().left() - 20.;
                    shapes.push(Shape::line_segment(
                        [
                            Pos2 { x:x, y:y_top },
                            Pos2 { x:x, y:y_bottom },
                        ],
                        Stroke{
                            width: match fret {
                                1 => 2.,
                                _ => 1.
                            },
                            color: match fret {
                                1 => Color32::WHITE,
                                _ => Color32::DARK_GRAY,
                            }
                        }
                    ));
                }
                ui.end_row();
                
                // draw lines:
                ui.painter().extend(shapes);
            });
        });
    }
}
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::*;
use egui::*;
use egui_extras::{StripBuilder, Size};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

mod guitar;
use guitar::*;

#[cfg(not(target_arch = "wasm32"))]
fn main(){
    let _result = eframe::run_native(
        "guitar-scales",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyEGuiApp::new())),
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

struct MyEGuiApp {
    strings: Vec<usize>,
    frets: usize,
    scale_num: usize,
    scale_key: usize,
    scales: Vec<ScaleIntervals>,
    notes: Vec<String>,
}
impl MyEGuiApp {
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
            x if x >= self.scales.len() || x < 0 => {
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
}
impl Default for MyEGuiApp {
    fn default() -> Self {
        Self {
            strings: vec![0,5,10,15,19,24],
            frets: 9,
            scale_key: 8,
            scale_num: 0,
            scales: vec![
                music_scale!("pentaminor", 0, 3, 5, 7, 10, 12),
                music_scale!("pentamajor", 0, 2, 4, 7, 9, 12),
                music_scale!("minor", 0, 2, 3, 5, 7, 8, 10, 12),
                music_scale!("major", 0, 2, 4, 5, 7, 9, 11, 12),
                music_scale!("dorian", 0, 2, 3, 5, 7, 9, 10, 12),
                music_scale!("phrygian", 0, 1, 3, 5, 7, 8, 10, 12),
                music_scale!("lydian", 0, 2, 4, 6, 7, 9, 11, 12),
                music_scale!("bluesminor", 0, 3, 5, 6, 7, 10, 12),
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
        }
    }
}
impl eframe::App for MyEGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let panel = egui::CentralPanel::default();
        panel.show(ctx, |ui|{
            ui.heading("Guitar Scales");
            // render toolbar:
            ui.horizontal(|ui|{
                
                ComboBox::from_id_source("scale")
                    .selected_text(format!("{} scale", &self.lookup_scale_str(self.scale_num)))
                    .show_ui(ui, |inner_ui| {
                        for i in 0..self.scales.len() {
                            inner_ui.selectable_value(&mut self.scale_num, i, &self.scales[i].name);
                        }
                    });

                ComboBox::from_id_source("key")
                    .selected_text(format!("key of {}", &self.lookup_note_str(self.scale_key)))
                    .width(50.0)
                    .show_ui(ui, |inner_ui| {
                        for i in 0..self.notes.len() {
                            inner_ui.selectable_value(&mut self.scale_key, i, &self.notes[i]);
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
                                format!("{}",i));
                        }
                    });
                
            });
            ui.add_space(10.0);
            
            let line = Shape::LineSegment{
                points:[
                    Pos2 {
                        x:0f32,
                        y:0f32,
                    },
                    Pos2 {
                        x:500f32,
                        y:500f32,
                    },
                ],
                stroke:Stroke{
                    width:1.,
                    color:Color32::WHITE,
                }
            };

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                for fret in 0..(*&self.frets + 1) {
                    ui.colored_label(Color32::WHITE, match fret {
                        3|5|7|9|12|15|17|19 => {
                            "•"
                        },
                        _ => {
                            ""
                        }
                    });
                }
                ui.end_row();
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
                        } else {
                            if self.is_note_in_scale(note_as_int as i16){
                                //ui.button(caption);
                                if self.is_note_root(note_as_int as i16) {
                                    ui.colored_label(Color32::WHITE, caption);
                                } else {
                                    ui.colored_label(Color32::GOLD, caption);
                                }
                            } else {
                                ui.colored_label(Color32::DARK_GRAY, caption);
                            }
                        }
                    }
                    ui.end_row();
                }
                for fret in 0..(*&self.frets + 1) {
                    ui.colored_label(Color32::WHITE, match fret {
                        3|5|7|9|12|15|17|19 => {
                            "•"
                        },
                        _ => {
                            ""
                        }
                    });
                }
                ui.end_row();
            });
        });
    }
}
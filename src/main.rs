#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{borrow::{Borrow, BorrowMut}, ops::Div};

use eframe::*;
use egui::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod scales;
use scales::*;

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
    AllNotes,
    Letters,
    Numbers,
    Triads,
    Debug,
}
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
enum StringStyle {
    String,
    Cells,
}
struct Instrument {
    name: String,
    tuning: String,
    strings: Vec<usize>,
}
struct DrawSettings {
    dark_mode: bool,
    frets: usize,
    fret_marks: FretMarker,
    note_marks: NoteMarker,
    string_style: StringStyle,
    space_string: f32,
    space_fret: f32,
}
struct FretboardApp {
    instruments: Vec<Instrument>,
    current_instrument: usize,
    settings: DrawSettings,
    scale: Scale,
}
impl Default for FretboardApp {
    fn default() -> Self {
        Self {
            current_instrument: 0,
            instruments: vec![
                Instrument {
                    name: "Guitar".to_string(),
                    tuning: "EADGBE".to_string(),
                    strings: vec![4,9,14,19,23,28],
                },
                Instrument {
                    name: "Violin".to_string(),
                    tuning: "GDAE".to_string(),
                    strings: vec![7,14,21,28],
                }
            ],
            settings: DrawSettings {
                dark_mode: false,
                frets: 9,
                fret_marks: FretMarker::Dots,
                note_marks: NoteMarker::Letters,
                string_style: StringStyle::String,
                space_string: 20.0,
                space_fret: 40.0,
            },
            scale: Scale {
                siz: ScaleSize::Pentatonic,
                typ: ScaleType::Minor,
                key: 0,
            }
        }
    }
}
impl eframe::App for FretboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.settings.dark_mode = ctx.style().visuals.dark_mode;
        self.draw_panel_scale(ctx);
        self.draw_panel_settings(ctx);
        self.draw_panel_fretboard(ctx);
    }
}
impl FretboardApp {
    fn new() -> Self {
        Self::default()
    }
    pub fn instrument(&self) -> &Instrument {
        &self.instruments[self.current_instrument]
    }
    pub fn strings(&self) -> &Vec<usize> {
        &self.instrument().strings
    }
    fn draw_fret_marker(&self, fret:usize, painter:Painter, mut pos:Pos2){
        let is_octave = fret % 12 == 0;
        let draw_dot = |p|{
            painter.circle_filled(p, 3f32, match self.settings.dark_mode {
                false => Color32::BLACK,
                true => Color32::WHITE,
            });
        };
        match match fret.try_into().unwrap_or(0) {
            3|5|7|9|12|15|17|19 => self.settings.fret_marks,
            _ => FretMarker::None,
        } {
            FretMarker::None => {},
            FretMarker::Dots => {
                match is_octave {
                    false => draw_dot(pos),
                    true => {
                        pos.x += 4.0;
                        draw_dot(pos);
                        pos.x -= 8.0;
                        draw_dot(pos);
                    }
                }
            },
            FretMarker::Numbers => {
                painter.text(
                    pos,
                    Align2::CENTER_CENTER,
                    (fret).to_string(),
                    FontId {
                        size: 12f32,
                        family: FontFamily::Monospace,
                    },
                    match is_octave {
                        false => Color32::WHITE,
                        true => Color32::GOLD,
                    }
                );
            }
        }
    }
    fn draw_panel_scale(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("the_top_panel").show(ctx, |ui|{
            ui.add_space(3.0);
            ui.heading("Instrument & Scale");
            // render toolbar:
            ui.horizontal(|ui|{
    
                ComboBox::from_id_source("instrument")
                    .selected_text(format!("{} ({})", &self.instrument().name, &self.instrument().tuning))
                    .show_ui(ui, |inner_ui| {
                        for i in 0..self.instruments.len() {
                            inner_ui.selectable_value(&mut self.current_instrument, i, &self.instruments[i].name);
                        }
                    });
                
                ComboBox::from_id_source("scale_type")
                    .selected_text(format!("{:?} scale", self.scale.typ))
                    .show_ui(ui, |inner_ui|{
                        for s in ScaleType::iter() {
                            inner_ui.selectable_value(&mut self.scale.typ, s, format!("{:?}", s));
                        }
                    });
                
                ComboBox::from_id_source("scale_size")
                    .selected_text(format!("{:?} scale", self.scale.siz))
                    .show_ui(ui, |inner_ui|{
                        for s in ScaleSize::iter() {
                            inner_ui.selectable_value(&mut self.scale.siz, s, format!("{:?}", s));
                        }
                    });
                ui.add(egui::Slider::new(&mut self.scale.key, 0..=11).show_value(false));
                ui.label(format!("key of {}", NOTE_LETTERS[self.scale.key]));
            });
            ui.add_space(3.0);
        });
    }
    fn draw_panel_settings(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("View Options")
        .resizable(false)
        .default_width(240.0)
        .show(ctx, |ui|{
            ui.add_space(5.0);
            ui.heading("View Options");
            // render toolbar:
            egui::widgets::global_dark_light_mode_buttons(ui);
            egui::Grid::new("settings")
            .striped(true)
            .show(ui, |ui|{
                ui.label(format!("{} frets", self.settings.frets));
                ui.add(egui::Slider::new(&mut self.settings.frets, 4..= 25).show_value(false));
                ui.end_row();
                ui.label("fret marks");
                ComboBox::from_id_source("fret_marks")
                    .selected_text(format!("{:?}", self.settings.fret_marks))
                    .width(100.0)
                    .show_ui(ui, |inner_ui|{
                        for fm in FretMarker::iter() {
                            inner_ui.selectable_value(&mut self.settings.fret_marks, fm, format!("{:?}", fm));
                        }
                    });
                ui.end_row();
                ui.label("note marks");
                ComboBox::from_id_source("note_marks")
                    .selected_text(format!("{:?}", self.settings.note_marks))
                    .width(100.0)
                    .show_ui(ui, |inner_ui|{
                        for nm in NoteMarker::iter() {
                            inner_ui.selectable_value(&mut self.settings.note_marks, nm, format!("{:?}", nm));
                        }
                    });
                ui.end_row();
                ui.label("string style");
                ComboBox::from_id_source("string_style")
                    .selected_text(format!("{:?}", self.settings.string_style))
                    .width(100.0)
                    .show_ui(ui, |inner_ui|{
                        for s in StringStyle::iter() {
                            inner_ui.selectable_value(&mut self.settings.string_style, s, format!("{:?}", s));
                        }
                    });
                ui.end_row();
                ui.label("string spacing");
                ui.add(egui::Slider::new(&mut self.settings.space_string, 20.0..=60.0).show_value(false));
                ui.end_row();
                ui.label("fret spacing");
                ui.add(egui::Slider::new(&mut self.settings.space_fret, 40.0..=100.0).show_value(false));
            });
            ui.add_space(10.0);

        });
    }
    fn draw_panel_fretboard(&mut self, ctx: &egui::Context){
        let painter = Painter::new(
            ctx.clone(),
            LayerId {
                id: Id::new("shapes_layer"),
                order: Order::Background,
            },
            ctx.available_rect(),
        );
        egui::CentralPanel::default().show(ctx, |ui|{

            let num_cols = self.settings.frets + 1;
            // start first row

            let rect = ui.available_rect_before_wrap();
            let x_left = rect.left();
            let x_right = rect.right();
            let y_top = rect.top() + 30f32;
            let y_bottom = y_top + self.strings().len() as f32 * self.settings.space_string;
            
            // paint frets and fret-markers
            for fret in 0..num_cols {
                let mut fret_x = x_left + fret as f32 * self.settings.space_fret;
                self.draw_fret_marker(fret, painter.to_owned(), Pos2 { x: fret_x, y: y_top - 15f32 });
                self.draw_fret_marker(fret, painter.to_owned(), Pos2 { x: fret_x, y: y_bottom + 15f32 });
                fret_x += self.settings.space_fret.div(2f32);
                painter.line_segment(
                    [
                        Pos2 { x:fret_x, y:y_top },
                        Pos2 { x:fret_x, y:y_bottom },
                    ],
                    Stroke{
                        width: match fret {
                            0 => 3.,
                            _ => 1.
                        },
                        color: match fret {
                            0 => match self.settings.dark_mode {
                                false => Color32::BLACK,
                                true => Color32::WHITE,
                            },
                            _ => match self.settings.dark_mode {
                                false => Color32::LIGHT_GRAY,
                                true => Color32::DARK_GRAY,
                            },
                        }
                    }
                );
            }
            // paint strings and notes
            for i in 0..*&self.strings().len() {
                let string = self.strings()[self.strings().len() - i - 1];
                let yt :f32= y_top + (0f32 + i as f32) * self.settings.space_string;
                let yb :f32= y_top + (1f32 + i as f32) * self.settings.space_string;
                let yc = (yt + yb)/2f32;
                // draw horizontal line (string):
                match self.settings.string_style {
                    StringStyle::String => {
                        painter.line_segment(
                            [
                                Pos2::new(x_left, yc),
                                Pos2::new(x_right, yc),
                            ],
                            Stroke::new(1.0, match self.settings.dark_mode {
                                false => Color32::BLACK,
                                true => Color32::WHITE,
                            }),
                        );
                    },
                    StringStyle::Cells => {
                        painter.line_segment(
                            [
                                Pos2::new(x_left,yt),
                                Pos2::new(x_right, yt),
                            ],
                            Stroke::new(1.0, match self.settings.dark_mode {
                                false => Color32::BLACK,
                                true => Color32::WHITE,
                            }),
                        );
                        painter.line_segment(
                            [
                                Pos2::new(x_left, yb),
                                Pos2::new(x_right, yb),
                            ],
                            Stroke::new(1.0, match self.settings.dark_mode {
                                false => Color32::BLACK,
                                true => Color32::WHITE,
                            }),
                        );
                    },
                };
                // paint notes:
                for fret in 0..(num_cols) {
                    let b = self.scale.get_bubble(self.settings.dark_mode, string + fret, self.scale.key, self.settings.note_marks);
                    let pos = Pos2 {x: x_left + fret as f32 * self.settings.space_fret, y: yc };
                    painter.circle_filled(pos, 12f32, b.color);
                    painter.text(pos, Align2::CENTER_CENTER, b.text, FontId { size: 13f32, family: FontFamily::Monospace}, b.text_color);
                }
            }
        });
    }
}
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

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
    NotesInKey,
    Numbers,
    Triads,
}

struct FretboardApp {
    strings: Vec<usize>,
    frets: usize,
    scale_num: usize,
    scale_key: usize,
    scales: Vec<ScaleIntervals>,
    fret_marks: FretMarker,
    note_marks: NoteMarker,
    space_string: f32,
}
impl FretboardApp {
    fn new() -> Self {
        Self::default()
    }
    pub fn scale(&self) -> &ScaleIntervals {
        &self.scales[self.scale_num]
    }
    pub fn draw_fret_marker(&self, fret:usize, painter:Painter, mut pos:Pos2){
        let is_octave = fret % 12 == 0;
        let draw_dot = |p|{
            painter.circle_filled(p, 3f32, Color32::WHITE);
        };
        match match fret.try_into().unwrap_or(0) {
            3|5|7|9|12|15|17|19 => self.fret_marks,
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
}
impl Default for FretboardApp {
    fn default() -> Self {
        Self {
            // EADGBE:
            strings: vec![4,9,14,19,23,28],
            // GDAE: (violin)
            //strings: vec![7, 14, 21, 28],
            frets: 9,
            scale_key: 0,
            scale_num: 0,
            scales: vec![
                music_intervals!("major", false, Diatonic),
                music_intervals!("minor", true, Diatonic),
                music_intervals!("blues major", false, Blues),
                music_intervals!("blues minor", true, Blues),
                music_intervals!("pentatonic major", false, Pentatonic),
                music_intervals!("pentatonic minor", true, Pentatonic),
            ],
            fret_marks: FretMarker::Dots,
            note_marks: NoteMarker::Triads,
            space_string: 20.0,
        }
    }
}
impl eframe::App for FretboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let painter = Painter::new(
            ctx.clone(),
            LayerId {
                id: Id::new("shapes_layer"),
                order: Order::Background,
            },
            ctx.available_rect(),
        );
        egui::TopBottomPanel::top("the_top_panel").show(ctx, |ui|{
            ui.heading("Current Scale");
            // render toolbar:
            ui.horizontal(|ui|{
    
                ComboBox::from_id_source("scale")
                    .selected_text(format!("{} scale", &self.scale().name))
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
                    .selected_text(format!("key of {}", self.scales[self.scale_num].get_note_letter(self.scale_key)))
                    .width(50.0)
                    .show_ui(ui, |inner_ui| {
                        for i in 0..self.scale().get_total_tones() {
                            let str = self.scale().get_note_letter(i);
                            inner_ui.selectable_value(&mut self.scale_key, i, str);
                        }
                    });
                
            });
            ui.add_space(10.0);
        });
        egui::TopBottomPanel::bottom("the_bottom_panel").show(ctx, |ui|{
            ui.heading("View Options");
            // render toolbar:
            ui.horizontal(|ui|{
                
                ComboBox::from_id_source("frets")
                    .selected_text(format!("{} frets", self.frets))
                    .width(100.0)
                    .show_ui(ui, |inner_ui| {
                        for i in 4..25 {
                            inner_ui.selectable_value(&mut self.frets, i, format!("{}",i));
                        }
                    });
                
                ComboBox::from_id_source("fret_marks")
                    .selected_text(format!("{:?}", self.fret_marks))
                    .width(100.0)
                    .show_ui(ui, |inner_ui|{
                        for fm in FretMarker::iter() {
                            inner_ui.selectable_value(&mut self.fret_marks, fm, format!("{:?}", fm));
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
                ui.add(egui::Slider::new(&mut self.space_string, 20.0..=100.0).text("My value"));
            });
            ui.add_space(10.0);

        });
        egui::CentralPanel::default().show(ctx, |ui|{

            ui.horizontal(|ui|{
                egui::Grid::new("some_unique_id")
                    .min_row_height(self.space_string)
                    .show(ui, |ui| {
                        ui.end_row();
                        for i in (0..*&self.strings.len()).rev() {
                            let caption = self.scale().get_note_letter(self.strings[i]);
                            ComboBox::from_id_source(i)
                                .selected_text(caption)
                                .show_ui(ui, |inner_ui| {
                                    for ii in 0..self.scale().get_total_tones() {
                                        let str = self.scale().get_note_letter(ii);
                                        inner_ui.selectable_value(&mut self.strings[i], ii, str);
                                    }
                                });
                            ui.end_row();
                        }
                    });
                ui.vertical(|ui|{
                    // draw the fretboard below
                    egui::Grid::new("some_unique_id")
                    .min_row_height(self.space_string)
                    .show(ui, |ui| {

                        let num_cols = self.frets + 1;
                        // start first row

                        // paint fret-markers
                        for fret in 0..num_cols {
                            //ui.colored_label(Color32::WHITE, self.fret_marker(fret));
                            ui.centered_and_justified(|ui|{
                                let pos = ui.available_rect_before_wrap().center();
                                self.draw_fret_marker(fret, painter.clone(), pos);
                            });
                        }
                        let y_top = ui.available_rect_before_wrap().bottom();
                        ui.end_row();
                        
                        // "draw" each guitar string as a row in the grid
                        let x1 = ui.max_rect().left();
                        let x2 = ui.max_rect().right();
                        
                        for i in (0..*&self.strings.len()).rev() {
                            let cy = ui.available_rect_before_wrap().center().y;
                            // draw horizontal line (string):
                            painter.line_segment(
                                [
                                    Pos2 {x:x1, y:cy},
                                    Pos2 {x:x2, y:cy},
                                ],
                                Stroke{
                                    width: 1.0,
                                    color: Color32::WHITE,
                                }
                            );
                            for fret in 0..(num_cols) {
                                
                                // every note has a unique number
                                let note_as_int = &self.strings[i] + fret;
                                let b = self.scale().get_bubble(note_as_int, self.scale_key, self.note_marks);
                                
                                // paint the notes onto the fretboard:
                                ui.centered_and_justified(|ui|{
                                    let pos = ui.available_rect_before_wrap().center();
                                    painter.circle_filled(pos, 12f32, b.color);
                                    painter.text(pos, Align2::CENTER_CENTER, b.text, FontId { size: 13f32, family: FontFamily::Monospace}, Color32::BLACK);
                                });
                            }
                            ui.end_row();
                        }
                        
                        // bottom of fretboard
                        let y_bottom = ui.available_rect_before_wrap().top();
                        // draw row of fret-markers:
                        for fret in 0..num_cols {
                            ui.centered_and_justified(|ui|{
                                let pos = ui.available_rect_before_wrap().center();
                                self.draw_fret_marker(fret, painter.clone(), pos);
                            });

                            // calculate placement of lines, and
                            // store each line in shapes collection
                            let x = ui.available_rect_before_wrap().left() - 5f32;
                            painter.line_segment(
                                [
                                    Pos2 { x:x, y:y_top },
                                    Pos2 { x:x, y:y_bottom },
                                ],
                                Stroke{
                                    width: match fret {
                                        0 => 2.,
                                        _ => 1.
                                    },
                                    color: match fret {
                                        0 => Color32::WHITE,
                                        _ => Color32::DARK_GRAY,
                                    }
                                }
                            );
                        }
                        ui.end_row();
                        
                    });
                });
            });
        });
        //painter.extend(shapes);
    }
}
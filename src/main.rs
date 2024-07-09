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
struct Instrument {
    name: String,
    tuning: String,
    strings: Vec<usize>,
}
struct DrawSettings {
    frets: usize,
    fret_marks: FretMarker,
    note_marks: NoteMarker,
    space_string: f32,
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
                frets: 9,
                fret_marks: FretMarker::Dots,
                note_marks: NoteMarker::NotesInKey,
                space_string: 20.0,
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
        self.draw_panel_top(ctx);
        self.draw_panel_bottom(ctx);
        self.draw_panel_center(ctx);
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
            painter.circle_filled(p, 3f32, Color32::WHITE);
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
    fn draw_panel_top(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("the_top_panel").show(ctx, |ui|{
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
                
                ComboBox::from_id_source("key")
                    .selected_text(format!("key of {}", NOTE_LETTERS[self.scale.key]))
                    .width(50.0)
                    .show_ui(ui, |inner_ui| {
                        for i in 0..TOTAL_TONES {
                            let str = self.scale.get_note_letter(i);
                            inner_ui.selectable_value(&mut self.scale.key, i, str);
                        }
                    });
            });
            ui.add_space(10.0);
        });
    }
    fn draw_panel_bottom(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("the_bottom_panel").show(ctx, |ui|{
            ui.heading("View Options");
            // render toolbar:
            ui.horizontal(|ui|{
                
                ComboBox::from_id_source("frets")
                    .selected_text(format!("{} frets", self.settings.frets))
                    .width(100.0)
                    .show_ui(ui, |inner_ui| {
                        for i in 4..25 {
                            inner_ui.selectable_value(&mut self.settings.frets, i, format!("{}",i));
                        }
                    });
                ComboBox::from_id_source("fret_marks")
                    .selected_text(format!("{:?}", self.settings.fret_marks))
                    .width(100.0)
                    .show_ui(ui, |inner_ui|{
                        for fm in FretMarker::iter() {
                            inner_ui.selectable_value(&mut self.settings.fret_marks, fm, format!("{:?}", fm));
                        }
                    });
                ComboBox::from_id_source("note_marks")
                    .selected_text(format!("{:?}", self.settings.note_marks))
                    .width(100.0)
                    .show_ui(ui, |inner_ui|{
                        for nm in NoteMarker::iter() {
                            inner_ui.selectable_value(&mut self.settings.note_marks, nm, format!("{:?}", nm));
                        }
                    });
                ui.add(egui::Slider::new(&mut self.settings.space_string, 20.0..=100.0).text("My value"));
            });
            ui.add_space(10.0);

        });
    }
    fn draw_panel_center(&mut self, ctx: &egui::Context){
        let painter = Painter::new(
            ctx.clone(),
            LayerId {
                id: Id::new("shapes_layer"),
                order: Order::Background,
            },
            ctx.available_rect(),
        );
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.vertical(|ui|{

                // draw the fretboard below
                egui::Grid::new("some_unique_id")
                .min_row_height(self.settings.space_string)
                .show(ui, |ui| {

                    let num_cols = self.settings.frets + 1;
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
                    
                    for i in (0..*&self.strings().len()).rev() {
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
                            let note_as_int = &self.strings()[i] + fret;
                            let b = self.scale.get_bubble(note_as_int, self.scale.key, self.settings.note_marks);
                            
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
    }
}
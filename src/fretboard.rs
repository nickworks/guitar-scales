use egui::*;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::ops::Div;
use crate::scales::{Scale, ScaleSize, ScaleType, NOTE_LETTERS, TOTAL_TONES};
use egui_notify::Toasts;

#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum FretMarker {
    Dots,
    Numbers,
    None,
}
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum NoteMarker {
    AllNotes,
    Letters,
    Numbers,
    Debug,
}
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
enum StringStyle {
    String,
    Cells,
}
pub struct Instrument {
    name: String,
    strings: Vec<usize>,
}
impl Default for Instrument {
    fn default() -> Self {
        Self {
            name: "None".to_string(),
            strings: vec!(),
        }
    }
}
impl Instrument {
    fn guitar() -> Instrument {
        Instrument {
            name: "Guitar (standard)".to_string(),
            strings: vec!(4,9,14,19,23,28),
        }
    }
    fn name(&mut self) -> &mut String {
        &mut self.name
    }
}
struct DrawSettings {
    dark_mode: bool,
    vertical: bool,
    frets: usize,
    fret_marks: FretMarker,
    note_marks: NoteMarker,
    string_style: StringStyle,
    space_string: f32,
    space_fret: f32,
}
pub struct FretboardApp {
    toasts: Toasts,
    instruments: Vec<Instrument>,
    current_instrument: usize,
    show_settings: bool,
    show_instrument: bool,
    settings: DrawSettings,
    scale: Scale,
}
impl Default for FretboardApp {
    fn default() -> Self {
        Self {
            // initialize once
            toasts: Toasts::default(),
            current_instrument: 0,
            instruments: vec![
                Instrument {
                    name: "Guitar".to_string(),
                    strings: vec![4,9,14,19,23,28],
                },
                Instrument {
                    name: "Violin".to_string(),
                    strings: vec![7,14,21,28],
                },
                Instrument {
                    name: "Cello".to_string(),
                    strings: vec![0,7,14,21],
                },
            ],
            show_instrument: false,
            show_settings: false,
            settings: DrawSettings {
                dark_mode: false,
                vertical: true,
                frets: 12,
                fret_marks: FretMarker::Dots,
                note_marks: NoteMarker::Letters,
                string_style: StringStyle::String,
                space_string: 50.0,
                space_fret: 80.0,
            },
            scale: Scale {
                siz: ScaleSize::Pentatonic,
                typ: ScaleType::Minor,
                key: 7,
            }
        }
    }
}
impl eframe::App for FretboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.settings.dark_mode = ctx.style().visuals.dark_mode;
        if self.show_settings {
            self.draw_panel_settings(ctx);
        }
        if self.show_instrument {
            self.draw_panel_instrument(ctx);
        }
        self.draw_top_bar(ctx);
        self.draw_bottom_bar(ctx);
        self.draw_panel_fretboard(ctx);
        self.toasts.show(ctx);
    }
}
impl FretboardApp {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn instrument(&self) -> &Instrument {
        &self.instruments[self.current_instrument]
    }
    pub fn strings(&self) -> &Vec<usize> {
        &self.instrument().strings
    }
    fn draw_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("the_top_panel").show(ctx, |ui|{
            ui.add_space(3.0);
            ui.heading(format!("{:?} {:?} scale in {}", self.scale.siz, self.scale.typ, NOTE_LETTERS[self.scale.key]));
            
            // render toolbar:
            ui.horizontal(|ui|{
    
                ComboBox::from_id_source("instrument")
                    .selected_text(format!("{}", &self.instrument().name))
                    .show_ui(ui, |inner_ui| {
                        for i in 0..self.instruments.len() {
                            inner_ui.selectable_value(&mut self.current_instrument, i, &self.instruments[i].name);
                        }
                    });
                
                ComboBox::from_id_source("scale_size")
                    .selected_text(format!("{:?}", self.scale.siz))
                    .show_ui(ui, |inner_ui|{
                        for s in ScaleSize::iter() {
                            inner_ui.selectable_value(&mut self.scale.siz, s, format!("{:?}", s));
                        }
                    });
                
                ComboBox::from_id_source("scale_type")
                    .selected_text(format!("{:?} scale", self.scale.typ))
                    .show_ui(ui, |inner_ui|{
                        for s in ScaleType::iter() {
                            inner_ui.selectable_value(&mut self.scale.typ, s, format!("{:?}", s));
                        }
                    });
                
                ui.add(egui::Slider::new(&mut self.scale.key, 0..=11).show_value(false));
                ui.label(format!("key of {}", NOTE_LETTERS[self.scale.key]));

            });
            ui.add_space(3.0);
        });
    }
    fn draw_bottom_bar(&mut self, ctx: &egui::Context){
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui|{
            ui.add_space(3f32);
            ui.horizontal(|ui| {
                let id_gap_size = Id::new("gap_size");
                let width_init = ui.max_rect().width();
                let width_bttns = ui.data(|data| data.get_temp(id_gap_size).unwrap_or(width_init));
                let width_gap = width_init - width_bttns;
                ui.toggle_value(&mut self.show_instrument, "🎸 Instrument");
                ui.add_space(width_gap);
                ui.toggle_value(&mut self.show_settings, "⚙ Visual Settings");
                // calculate the gap-size AFTER drawing toggle buttons
                ui.data_mut(|data| data.insert_temp(
                    id_gap_size,
                    ui.min_rect().width() - width_gap,
                ));
            });
            ui.add_space(0f32);
        });
    }
    fn draw_panel_settings(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("View Options")
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
                ui.label("vertical fretboard");
                let label = match self.settings.vertical {
                    true => "On",
                    false => "Off",
                };
                ui.toggle_value(&mut self.settings.vertical, label);
                ui.end_row();
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
    fn draw_panel_instrument(&mut self, ctx: &egui::Context) {
        let red = match self.settings.dark_mode { false => Color32::LIGHT_RED, true => Color32::DARK_RED };
        let green = match self.settings.dark_mode { false => Color32::LIGHT_GREEN, true => Color32::DARK_GREEN };
        egui::SidePanel::left("View instrument")
        .resizable(false)
        .default_width(240.0)
        .show(ctx, |ui|{
            ui.add_space(5.0);
            ui.heading("Instruments");
            ui.add_space(14f32);
            ui.horizontal(|ui|{
                ComboBox::from_id_source("instrument")
                .selected_text(format!("{:.28}", &self.instrument().name))
                .width(100f32)
                .truncate()
                .show_ui(ui, |inner_ui| {
                    for i in 0..self.instruments.len() {
                        inner_ui.selectable_value(&mut self.current_instrument, i, &self.instruments[i].name);
                    }
                });
                if ui.add(Button::new("New").fill(green)).clicked() {
                    self.instruments.push(Instrument::guitar());
                    self.current_instrument = self.instruments.len() - 1;
                };
            });
            ui.add_space(14f32);
            egui::Grid::new("instrument_meta").show(ui, |ui|{
                let name = self.instruments[self.current_instrument].name();
                ui.label("Name");
                ui.add(egui::TextEdit::singleline(name).min_size(Vec2{x:130f32,y:20f32}).char_limit(30));
                ui.end_row();
                ui.label("Strings");
                if ui.add(Button::new("Add string").fill(green)).clicked() {
                    let new_string = self.instruments[self.current_instrument].strings[self.strings().len() - 1] + 5;
                    self.instruments[self.current_instrument].strings.push(new_string);
                }
                ui.end_row();
            });
            ui.add_space(12f32);
            egui::Grid::new("instrument_grid").show(ui, |ui|{
                for i in (0..*&self.strings().len()).rev() {
                    let caption = NOTE_LETTERS[self.strings()[i]%12];
                    ui.label(format!("   #{}", i+1));
                    ComboBox::from_id_source(i)
                        .selected_text(caption)
                        .show_ui(ui, |inner_ui| {
                            for ii in 0..TOTAL_TONES {
                                inner_ui.selectable_value(&mut self.instruments[self.current_instrument].strings[i], ii, self.scale.get_note_letter(ii));
                            }
                        });
                    if ui.add(Button::new(" X ").fill(red)).clicked() {
                        self.instruments[self.current_instrument].strings.remove(i);
                    }
                    ui.end_row();
                }
            });
            ui.add_space(14.0);
            if ui.add(Button::new(" Delete instrument ").fill(red)).clicked() {
                if self.instruments.len() > 1 {
                    self.instruments.remove(self.current_instrument);
                    if self.current_instrument + 1 >= self.instruments.len() {
                        self.current_instrument = match self.instruments.len() { 0 => 0, _ => self.instruments.len() - 1 };
                    }
                } else {
                    self.toasts.error("You may not delete your only instrument.");
                }
            }
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

            let num_frets = self.settings.frets + 1;
            
            // the "width" of the fret board
            let fretboard_size = (self.strings().len() as f32) * self.settings.space_string;
            let half_size = fretboard_size/2f32;

            // rect position
            let rect = ui.available_rect_before_wrap();
            let offset:f32 = 10f32 + match self.settings.vertical {
                true => rect.top(),
                false => rect.left(),
            };
            let center = match self.settings.vertical {
                false => rect.center().y,
                true => rect.center().x,
            };
            // fret marker positions
            let gap_to_fret_markers = 10f32;
            let d_fret_marker1 = center - half_size as f32 - gap_to_fret_markers;
            let d_fret_marker2 = center + half_size as f32 + gap_to_fret_markers;
            
            // paint frets and fret-markers
            for fret in 0..num_frets {
                let mut pos_down_neck = fret as f32 * self.settings.space_fret + offset;
                let pos_fret_marker1 = match self.settings.vertical {
                    true => Pos2 { x: d_fret_marker1, y: pos_down_neck },
                    false => Pos2 { x: pos_down_neck, y: d_fret_marker1 },
                };
                let pos_fret_marker2 = match self.settings.vertical {
                    true => Pos2 { x: d_fret_marker2, y: pos_down_neck },
                    false => Pos2 { x: pos_down_neck, y: d_fret_marker2 },
                };
                self.draw_fret_marker(fret, painter.to_owned(), pos_fret_marker1);
                self.draw_fret_marker(fret, painter.to_owned(), pos_fret_marker2);
                pos_down_neck += self.settings.space_fret.div(2f32);
                self.draw_fret(painter.to_owned(), fret, pos_down_neck, center, half_size);
            }
            // paint strings and notes
            for i in 0..*&self.strings().len() {
                let string = self.strings()[ match self.settings.vertical {
                    true => i,
                    false => self.strings().len() - i - 1,
                }];

                let cell_pree :f32= center - half_size + (i as f32 * self.settings.space_string);
                let cell_post :f32= cell_pree + self.settings.space_string;
                let cell_middle = (cell_pree + cell_post)/2f32;

                // draw horizontal line (string):
                match self.settings.string_style {
                    StringStyle::String => {
                        self.draw_line(painter.to_owned(), rect, cell_middle);
                    },
                    StringStyle::Cells => {
                        self.draw_line(painter.to_owned(), rect, cell_pree);
                        self.draw_line(painter.to_owned(), rect, cell_post);
                    },
                };

                // paint notes:
                for fret in 0..(num_frets) {
                    let b = self.scale.get_bubble(self.settings.dark_mode, string + fret, self.scale.key, self.settings.note_marks);
                    let pos = match self.settings.vertical {
                        false => Pos2 { x: offset + fret as f32 * self.settings.space_fret, y: cell_middle },
                        true => Pos2 { x: cell_middle, y: offset + fret as f32 * self.settings.space_fret },
                    };
                    painter.circle_filled(pos, 12f32, b.color);
                    painter.text(pos, Align2::CENTER_CENTER, b.text, FontId { size: 13f32, family: FontFamily::Monospace}, b.text_color);
                }
            }
        });
    }
    fn draw_line(&self, painter:Painter, rect:Rect, pos:f32){
        painter.line_segment(
            match self.settings.vertical {
                false => [
                    Pos2::new(rect.left(), pos),
                    Pos2::new(rect.right(), pos),
                ],
                true => [
                    Pos2::new(pos, rect.top()),
                    Pos2::new(pos, rect.bottom()),
                ],
            },
            Stroke::new(1.0, match self.settings.dark_mode {
                false => Color32::BLACK,
                true => Color32::WHITE,
            }),
        );
    }
    fn draw_fret(&self, painter:Painter, fret:usize, offset:f32, center:f32, half_width:f32) {
        painter.line_segment(
            match self.settings.vertical {
                true => [
                    Pos2 {
                        x: center - half_width,
                        y: offset,
                    },
                    Pos2 {
                        x: center + half_width,
                        y: offset,
                    },
                ],
                false => [
                    Pos2 {
                        x: offset,
                        y: center - half_width,
                    },
                    Pos2 {
                        x: offset,
                        y: center + half_width,
                    },
                ],
            },
            Stroke {
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
                        false => match self.settings.dark_mode { true => Color32::WHITE, false => Color32::BLACK },
                        true => match self.settings.dark_mode { true => Color32::GOLD, false => Color32::BLUE },
                    }
                );
            }
        }
    }
}
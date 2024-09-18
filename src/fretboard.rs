use egui::*;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::ops::Div;
use crate::scales::{note_letter, prefers_flats, scale_name, NoteType, Scale, ScaleSize, ScaleType, TOTAL_TONES};
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
#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
enum Panel {
    None,
    Instrument,
    ViewSettings,
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
    fn none() -> Instrument {
        Instrument::from("none", vec!())
    }
    fn guitar() -> Instrument {
        Instrument::from("Guitar", vec!(4,9,14,19,23,28))
    }
    fn violin() -> Instrument {
        Instrument::from("Violin", vec![7,14,21,28])
    }
    fn cello() -> Instrument {
        Instrument::from("Cello", vec![0,7,14,21])
    }
    fn ukulele() -> Instrument {
        Instrument::from("Ukulele", vec![7,12,16,21])
    }
    fn banjo() -> Instrument {
        Instrument::from("Banjo", vec![7,14,19,23,26])
    }
    fn from(name: &str, strings: Vec<usize>) -> Instrument {
        Instrument {
            name: name.to_string(),
            strings: strings,
        }
    }
    fn name(&mut self) -> &mut String {
        &mut self.name
    }
}
struct DrawSettings {
    dark_mode: bool,
    vertical: bool,
    show_legend: bool,
    frets: usize,
    fret_marks: FretMarker,
    note_marks: NoteMarker,
    string_style: StringStyle,
    space_string: f32,
    space_fret: f32,
    dot_size:f32,
}
pub struct FretboardApp {
    toasts: Toasts,
    empty_instrument: Instrument,
    instruments: Vec<Instrument>,
    current_instrument: usize,
    open_panel: Panel,
    settings: DrawSettings,
    scale: Scale,
}
impl Default for FretboardApp {
    fn default() -> Self {
        Self {
            // initialize once
            toasts: Toasts::default(),
            open_panel:Panel::None,
            current_instrument: 0,
            empty_instrument: Instrument::none(),
            instruments: vec![
                Instrument::guitar(),
                Instrument::violin(),
                Instrument::cello(),
                Instrument::ukulele(),
                Instrument::banjo(),
            ],
            settings: DrawSettings {
                dark_mode: false,
                vertical: true,
                show_legend: true,
                frets: 12,
                fret_marks: FretMarker::Dots,
                note_marks: NoteMarker::Letters,
                string_style: StringStyle::String,
                space_string: 50.0,
                space_fret: 50.0,
                dot_size:16f32,
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
        self.draw_top_bar(ctx);
        match self.open_panel {
            Panel::Instrument => self.draw_panel_instrument(ctx),
            Panel::ViewSettings => self.draw_panel_settings(ctx),
            _ => {},
        }
        self.draw_panel_fretboard(ctx);
        self.toasts.show(ctx);
    }
}
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            //"../assets/noto/NotoMusic-Regular.ttf",
            "../Lucida.ttf",
            //"../FiraSans-Regular.otf"
            //"../GoogleSans-Regular.ttf"
            //"../SF-Pro.ttf"
        )),
    );
    // Put my font as highest priority for the monospace font definition
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
fn font_glyph() -> FontId {
    font(18f32, FontFamily::Proportional)
}
fn font(size:f32, family:FontFamily) -> FontId {
    FontId { size:size, family:family }
}
impl FretboardApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }
    pub fn instrument(&self) -> &Instrument {
        if self.instruments.len() <= 0 {
            return &self.empty_instrument
        }
        &self.instruments[self.current_instrument]
    }
    pub fn strings(&self) -> &Vec<usize> {
        &self.instrument().strings
    }
    fn draw_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("the_top_panel").show(ctx, |ui|{
            // render toolbar:
            ui.horizontal(|ui|{
                ui.set_height(30f32);
                ComboBox::from_id_source("scale_key")
                    .selected_text(format!("key of {}", self.scale.get_note_letter(self.scale.key)))
                    .show_ui(ui, |inner_ui|{
                        for i in 0..12 {
                            let letter = note_letter(i, prefers_flats(self.scale.typ, i));
                            inner_ui.selectable_value(&mut self.scale.key, i, letter);
                        }
                    });
                ComboBox::from_id_source("scale_type")
                    .selected_text(format!("{:?}", self.scale.typ))
                    .show_ui(ui, |inner_ui|{
                        for s in ScaleType::iter() {
                            inner_ui.selectable_value(&mut self.scale.typ, s, format!("{:?}", s));
                        }
                    });
                ComboBox::from_id_source("scale_size")
                    .selected_text(self.scale.scale_name())
                    .show_ui(ui, |inner_ui|{
                        for s in ScaleSize::iter() {
                            inner_ui.selectable_value(&mut self.scale.siz, s, scale_name(s));
                        }
                    });
            });
            // render toolbar:
            ui.horizontal(|ui|{
                ui.set_height(20f32);
                ComboBox::from_id_source("instrument")
                    .selected_text(format!("{}", &self.instrument().name))
                    .width(100f32)
                    .truncate()
                    .show_ui(ui, |inner_ui| {
                        for i in 0..self.instruments.len() {
                            inner_ui.selectable_value(&mut self.current_instrument, i, &self.instruments[i].name);
                        }
                    });
                let mut show_settings = self.open_panel == Panel::ViewSettings;
                if ui.toggle_value(&mut show_settings, "👁 Settings").clicked(){
                    self.open_panel = match self.open_panel {
                        Panel::ViewSettings => Panel::None,
                        _ => Panel::ViewSettings,
                    };
                }
                let mut show_instrument = self.open_panel == Panel::Instrument;
                if ui.toggle_value(&mut show_instrument, "🎸 Instruments").clicked() {
                    self.open_panel = match self.open_panel {
                        Panel::Instrument => Panel::None,
                        _ => Panel::Instrument,
                    };
                }
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
            ui.add_space(14f32);
            // render toolbar:
            egui::Grid::new("settings")
            .show(ui, |ui|{
                ui.label("theme");
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.end_row();
                ui.label("vertical fretboard");
                let label = match self.settings.vertical {
                    true => "On",
                    false => "Off",
                };
                ui.toggle_value(&mut self.settings.vertical, label);
                ui.end_row();
                ui.label("show legend");
                let label = match self.settings.show_legend {
                    true => "On",
                    false => "Off",
                };
                ui.toggle_value(&mut self.settings.show_legend, label);
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
                ui.add(egui::Slider::new(&mut self.settings.space_string, 40.0..=100.0).show_value(false));
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
                    .selected_text(format!("{}", &self.instrument().name))
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
            if self.instruments.len() > 0 {
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
                ui.add_space(6f32);
                egui::Grid::new("instrument_grid").show(ui, |ui|{
                    for i in (0..*&self.strings().len()).rev() {
                        let caption = note_letter(self.strings()[i]%12, true);
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
                    if self.instruments.len() > 0 {
                        self.instruments.remove(self.current_instrument);
                        if self.current_instrument + 1 >= self.instruments.len() {
                            self.current_instrument = match self.instruments.len() { 0 => 0, _ => self.instruments.len() - 1 };
                        }
                    } else {
                        self.toasts.error("There are no instruments to delete");
                    }
                }
            } else {
                // no instrument to render
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
                    let b = self.scale.get_bubble(self.settings.dark_mode, string + fret, self.settings.note_marks);
                    let pos = match self.settings.vertical {
                        false => Pos2 { x: offset + fret as f32 * self.settings.space_fret, y: cell_middle },
                        true => Pos2 { x: cell_middle, y: offset + fret as f32 * self.settings.space_fret },
                    };
                    painter.circle_filled(pos, self.settings.dot_size, b.color);
                    painter.text(pos, Align2::CENTER_CENTER, b.text, font_glyph(), b.text_color);
                }
            }
        
            // paint legend
            if self.settings.show_legend {
                self.draw_legend(rect, painter);
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
            self.stroke(1f32),
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
            self.stroke(match fret {
                0 => 3f32,
                _ => 1f32,
            }),
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
                    font(12f32, FontFamily::Monospace),
                    match is_octave {
                        false => match self.settings.dark_mode { true => Color32::WHITE, false => Color32::BLACK },
                        true => match self.settings.dark_mode { true => Color32::GOLD, false => Color32::BLUE },
                    }
                );
            }
        }
    }
    fn draw_legend(&self, rect:Rect, painter:Painter){
        let m = 10f32;
        let w = 250f32;
        let h = 165f32;
        let bg = Rect {
            min: Pos2 { x: rect.right() - m - w, y: rect.bottom() - m - h },
            max: Pos2 { x: rect.right() - m, y: rect.bottom() - m },
        };
        painter.rect(bg, Rounding::same(4f32), match self.settings.dark_mode {
            true => Color32::DARK_GRAY,
            false => Color32::LIGHT_GRAY,
        }, self.stroke(1f32));
        let draw_dot = |x:f32, y:f32, n, str|{
            let pos = Pos2 { x: bg.min.x + x, y: bg.min.y + y };
            let b = self.scale.get_bubble_from(self.settings.dark_mode, n, self.settings.note_marks);
            painter.circle_filled(pos, self.settings.dot_size, b.color);
            painter.text(pos, Align2::CENTER_CENTER, b.text, font_glyph(), b.text_color);
            painter.text(
                Pos2 { x: pos.x + 30f32, y: pos.y}, 
                Align2::LEFT_CENTER, 
                str,
                font(14f32, FontFamily::Monospace),
                match self.settings.dark_mode { true => Color32::WHITE, false => Color32::BLACK }
            );
        };
        draw_dot(20f32, 20f32, NoteType::Root, "Root notes");
        draw_dot(20f32, 80f32, NoteType::InPentatonic, "Notes in scale (penta)");
        draw_dot(20f32, 140f32, NoteType::InDiatonic, "Notes in scale (natural)");
        draw_dot(20f32, 50f32, NoteType::Triad, "Triad notes");
        draw_dot(20f32, 110f32, NoteType::Blue, "Blue notes");

    }
    fn stroke(&self, weight:f32) -> Stroke {
        Stroke::new(weight, match self.settings.dark_mode {
            false => Color32::BLACK,
            true => Color32::WHITE,
        })
    }
}
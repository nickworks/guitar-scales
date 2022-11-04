#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::*;

#[cfg(not(target_arch = "wasm32"))]
fn main(){
    
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
    name:String,
    age:u32,
}
impl MyEGuiApp {
    fn new() -> Self {
        Self::default()
    }
}
impl Default for MyEGuiApp {
    fn default() -> Self {
        Self {
            name: "Nick".to_owned(),
            age: 36,
        }
    }
}
impl eframe::App for MyEGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

use eframe::egui::{self, RichText, Color32};
use crate::selection::rand;
use egui_extras::*;

pub struct App{
    key_type: rand::PasswordType,
    key_type_name :String,
    key: String,
}
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>)->Self{
        setup_custom_fonts(&cc.egui_ctx);
        install_image_loaders(&cc.egui_ctx);
        App { 
            key_type: rand::PasswordType::Alphanumeric,
            key_type_name: String::from(rand::AlPHANUMERIC),
            key: String::new()
         }
    }
    pub fn get_key_type_name(&mut self) ->&str{
        match self.key_type{
            rand::PasswordType::Alphanumeric =>{
                self.key_type_name = String::from(rand::AlPHANUMERIC);
            }
            rand::PasswordType::Alphabetic =>{
                self.key_type_name = String::from(rand::AIPHABETIC);
            }
            rand::PasswordType::Numeric =>{
                self.key_type_name = String::from(rand::NUMERIC);
            }
            rand::PasswordType::SpecialChars =>{
                self.key_type_name = String::from(rand::SPECIALCHARS);
            }
            rand::PasswordType::All =>{
                self.key_type_name = String::from(rand::ALL);
            }
        }
        &self.key_type_name
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_id_source("key_type")
                        .width(1f32)
                        .selected_text(self.get_key_type_name())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.key_type, rand::PasswordType::Alphanumeric, rand::AlPHANUMERIC);
                            ui.selectable_value(&mut self.key_type, rand::PasswordType::Alphabetic, rand::AIPHABETIC);
                            ui.selectable_value(&mut self.key_type, rand::PasswordType::Numeric, rand::NUMERIC);
                            ui.selectable_value(&mut self.key_type, rand::PasswordType::SpecialChars, rand::SPECIALCHARS);
                            ui.selectable_value(&mut self.key_type, rand::PasswordType::All, rand::ALL);
                        });
        });
    }
}
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Song".to_owned(),
        egui::FontData::from_static(include_bytes!("./font/STSong.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Song".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("Song".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
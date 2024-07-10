use eframe::egui::{self, Color32, Label, RichText};
use egui::*;
use crate::selection::rand;
use egui_extras::*;

pub struct KeyInfo{
    key_name: String,
    key: String,
    key_remark: String
}

pub struct App{
    key_type: rand::PasswordType,
    key_type_name :String,
    key_len: usize,
    key_name: String,
    key: String,
    key_info: Vec<KeyInfo>
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>)->Self{
        setup_custom_fonts(&cc.egui_ctx);
        install_image_loaders(&cc.egui_ctx);
        App { 
            key_type: rand::PasswordType::All,
            key_type_name: String::from(rand::ALL),
            key_len: 12usize,
            key_name: String::new(),
            key: String::new(),
            key_info: Vec::<KeyInfo>::new()
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
    pub fn key_type_selection(&mut self, ui: &mut Ui){
        ui.horizontal(|ui|{
            ui.add(Label::new(RichText::new("密码类型:").color(Color32::BLUE)));
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
        ui.horizontal(|ui|{
            ui.add(Label::new(RichText::new("密码长度:").color(Color32::BLUE)));
            ui.add(egui::Slider::new(&mut self.key_len, 0..=120).text_color(Color32::RED));
            if ui.button("增加").clicked() {
                self.key_len += 1;
            }
            if ui.button("减少").clicked() {
                self.key_len -= 1;
            }
            if ui.button(RichText::new("生成密码").color(Color32::RED)).clicked() {
                self.key = rand::generate_random_password(self.key_len, self.key_type.clone());
            }
        });
    }
    pub fn key_generation(&mut self, ui: &mut Ui){
        ui.horizontal(|ui|{
            ui.add(Label::new(RichText::new("密码:").size(18.0).color(Color32::BLUE)));
            ui.add(Label::new(RichText::new(&self.key).size(18.0)).extend());
        });
        ui.centered_and_justified(|ui| {
            ui.image(include_image!("./picture/rust_zh.png"));
        });
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.key_type_selection(ui);
            self.key_generation(ui);
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
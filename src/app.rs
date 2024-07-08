use eframe::egui::{self, RichText, Color32};
use crate::selection::*;

pub struct App{
    key: String,
}
impl App {
    pub fn new()->Self{
        App { key: String::from("hi") }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new(self.key.clone()).color(Color32::RED));
        });
    }
}
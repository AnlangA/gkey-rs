#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
//#![allow(rustdoc::missing_crate_level_docs)]

use eframe::egui;
use tokio::sync::mpsc;
use std::sync::Arc;
use parking_lot::Mutex;

use gkey_rs::app::App;
use gkey_rs::data_deal;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, rx) = mpsc::channel(32);

    // 启动Tokio任务接收egui传递的数据
    rt.spawn(async move {
        data_deal::receive_app_data(rx).await;
    });

    let tx_mute = Arc::new(Mutex::new(tx));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 230.0]),
        ..Default::default()
    };
    eframe::run_native(
        "密码生成器",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc, rt, tx_mute)))),
    )
}


#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use eframe::{App, Frame, NativeOptions};
use eframe::egui::{CentralPanel, Context, SidePanel, Style, TopBottomPanel, Vec2, Visuals};
use tokio::runtime::Runtime;
use crate::Model::{load_icon, MainBody};
use crate::model_sql::{establish_connection, fill_with_data};

mod Model;
mod main_render;
mod custom;
mod bottom_renderer;
mod monitor_main;
mod model_sql;
mod monitor_bottom;

impl App for MainBody {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if let Ok(array) = self.data_table_receiver.try_recv() {
            self.data_table = array;
            println!("{:#?}", self.data_table)
        }

        if let Ok(month_vec) = self.month_filter_receiver.try_recv() {
            self.month_filter = month_vec;
        }

        if !self.monitor_mode {
            TopBottomPanel::bottom("Results").show_separator_line(false).exact_height(90f32).show_animated(ctx, self.bottom_active, |ui| {
                self.render_bottom(ui)
            });

            CentralPanel::default().frame(self.main_frame()).show(ctx, |ui| {
                self.render_main(ui)
            });
        }
        else {
            CentralPanel::default().frame(self.main_frame()).show(ctx, |ui| {
                self.render_main_monitor(ui)
            });

            TopBottomPanel::bottom("Modifier").show_separator_line(false).exact_height(150f32).show_animated(ctx, self.monitor_bottom_panel, |ui| {
                self.render_bottom_monitor(ui)
            });
        }
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let _enter = runtime.enter();
    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await
            }
        })
    });

    let mut window = NativeOptions::default();
    // window.icon_data = Some(load_icon("icon/app.png"));
    window.initial_window_size = Some(Vec2::new(400f32, 425f32));
    window.resizable = false;

    eframe::run_native("Tax calculator", window, Box::new(|cc| {
        let mut style = Style::default();
        style.visuals = Visuals::dark();
        cc.egui_ctx.set_style(style);
        Box::new(MainBody::new(cc, establish_connection()))
    })).unwrap()
}
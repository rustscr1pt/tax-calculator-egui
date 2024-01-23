use std::sync::Arc;
use eframe::egui::{Button, Color32, RichText, TextEdit, Ui, Vec2};
use crate::Model::{BottomMonitorPlacer, MainBody, text_formatter, TextStyler};
use crate::model_sql::{add_data_sql, bool_to_u8, string_to_int};

impl MainBody {
    pub fn render_bottom_monitor(&mut self, ui : &mut Ui) -> () {
        ui.vertical(|ui| {
            ui.add_space(5f32);
            self.render_title(ui);
            self.render_text_fields(ui, BottomMonitorPlacer::Amount);
            self.render_text_fields(ui, BottomMonitorPlacer::Comment);
        });
    }

    fn render_title(&self, ui : &mut Ui) -> () {
        ui.horizontal(|ui| {
            ui.add_space(10f32);
            ui.add(text_formatter("Добавить свою запись", TextStyler::Annotation))
        });
    }

    fn clear_text_fields(&mut self) -> () {
        self.bottom_monitor_texts.comment = "".to_owned();
        self.bottom_monitor_texts.income = "".to_owned();
        self.bottom_monitor_texts.is_possitive = false
    }

    fn render_text_fields(&mut self, ui : &mut Ui, typo : BottomMonitorPlacer) -> () {
        match typo {
            BottomMonitorPlacer::Amount => {
                ui.horizontal(|ui| {
                    ui.add_space(10f32);
                    ui.add_sized(Vec2::new(ui.available_width() * 0.3, 10f32), TextEdit::singleline(&mut self.bottom_monitor_texts.income).text_color(Color32::GREEN));
                    ui.add_space(10f32);
                    ui.add(text_formatter("сумма", TextStyler::BottomScroll));
                    ui.add_space(60f32);
                    if ui.add(Button::new(RichText::new("Тип транзакции"))).context_menu(|ui| {
                        if ui.button("заработок").clicked() {
                            self.bottom_monitor_texts.is_possitive = true;
                            ui.close_menu()
                        }
                        if ui.button("потеря").clicked() {
                            self.bottom_monitor_texts.is_possitive = false;
                            ui.close_menu()
                        }
                    }).clicked() {
                        // No action for left press
                    }
                });
            }
            BottomMonitorPlacer::Comment => {
                ui.horizontal(|ui| {
                    ui.add_space(10f32);
                    ui.add_sized(Vec2::new(ui.available_width() * 0.3, 10f32), TextEdit::singleline(&mut self.bottom_monitor_texts.comment).text_color(Color32::GREEN));
                    ui.add_space(10f32);
                    ui.add(text_formatter("комментарий", TextStyler::BottomScroll));
                    ui.add_space(10f32);
                    if ui.add(Button::new(RichText::new("Подтвердить"))).clicked() {
                        add_data_sql(Arc::clone(&self.sql_connection), string_to_int(self.bottom_monitor_texts.income.clone()), "JAN".to_string(), self.bottom_monitor_texts.comment.clone(), bool_to_u8(self.bottom_monitor_texts.is_possitive), 2024);
                        self.clear_text_fields();
                    }
                });
            }
        }
    }
}
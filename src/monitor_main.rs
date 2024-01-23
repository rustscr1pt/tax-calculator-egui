use eframe::egui::{Button, Color32, Label, RichText, Rounding, ScrollArea, Ui};
use crate::Model::{MainBody, OfficeEvent, text_formatter, TextStyler};
use crate::model_sql::{fill_with_data, filtered_request};

impl MainBody {
    pub fn render_main_monitor(&mut self, ui : &mut Ui) -> () {
        ui.vertical(|ui| {
            ui.add_space(10f32);
            self.render_top_interface(ui);
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(15f32);
                ui.add(text_formatter("Список записей:", TextStyler::Annotation))
            });
            ui.separator();
            ScrollArea::vertical().auto_shrink([false, false]).max_width(400f32).show(ui, |ui| {
                for element in &self.data_table {
                    self.render_card(element, ui)
                }
            })
        });
    }

    fn render_top_interface(&mut self, ui : &mut Ui) -> () {
        ui.horizontal(|ui| {
            ui.add_space(10f32);
            ui.scope(|ui| {
                let mut style = ui.style_mut();
                style.visuals.widgets.hovered.weak_bg_fill = Color32::BLACK;
                style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(53, 57, 53);
                if ui.add(Button::new(RichText::new("<=").size(15f32).color(Color32::WHITE)).rounding(Rounding::from(15f32))).clicked() {
                    self.mode_clicker()
                }
            });
            ui.add_space(240f32);
            ui.scope(|ui| {
                let mut style = ui.style_mut();
                style.visuals.widgets.hovered.weak_bg_fill = Color32::BLACK;
                style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(53, 57, 53);
                if ui.add(Button::new(RichText::new("Месяц").size(15f32).color(Color32::WHITE)).rounding(Rounding::from(15f32))).context_menu(|ui| {
                    if ui.button("Сбросить").clicked() {
                        fill_with_data(self.data_table_sender.clone(), self.sql_connection.clone(), self.month_filter_sender.clone());
                        ui.close_menu();
                    }
                    for elements in &self.month_filter {
                        if ui.button(elements.clone()).clicked() {
                            filtered_request(elements.clone(), self.sql_connection.clone(), self.data_table_sender.clone());
                            ui.close_menu();
                        }
                    }
                }).clicked() {
                    // No action in this block
                }
            });
            ui.add_space(10f32);
            ui.scope(|ui| {
                let mut style = ui.style_mut();
                style.visuals.widgets.hovered.weak_bg_fill = Color32::BLACK;
                style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(53, 57, 53);
                if ui.add(Button::new(RichText::new("+").size(15f32).color(Color32::WHITE)).rounding(Rounding::from(15f32))).clicked() {
                    self.bottom_panel_clicker()
                }
            });
        });
    }

    fn render_card(&self, data : &OfficeEvent, ui: &mut Ui) -> () {
        ui.vertical(|ui| {
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                let max_frame = ui.available_width();
                //println!("Max frame : {}", max_frame);
                let reserved_frame = max_frame - 300f32;
                ui.add_space(10f32);
                ui.add(text_formatter(data.amount.to_string().as_str(), TextStyler::TitleStyle));
                //println!("Other frame : {}", ui.available_width());
                ui.add_space(ui.available_width() - reserved_frame);
                ui.add(text_formatter(data.month.as_str(), TextStyler::TitleStyle));
                ui.add_space(5f32);
                ui.add(text_formatter(data.year.to_string().as_str(), TextStyler::TitleStyle));
            });
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(Label::new(RichText::new(data.my_comment.to_string()).monospace().size(12f32).color(Color32::WHITE)))
            });
            ui.horizontal(|ui| {
                match data.is_possitive {
                    0 => {
                        ui.add_space(350f32);
                        ui.add(Label::new(RichText::new("Loss").underline().monospace().size(14f32).color(Color32::RED)))
                    },
                    1 => {
                        ui.add_space(335f32);
                        ui.add(Label::new(RichText::new("Profit").underline().monospace().size(14f32).color(Color32::GREEN)))
                    },
                    _ => {unreachable!()}
                }
            })
        });
        ui.separator();
    }
}


// let sample : RectShape = RectShape {
// rect: eframe::egui::Rect {
// min : Pos2::new(10f32, 65f32),
// max : Pos2::new(390f32, 370f32)
// },
// rounding: Rounding::from(8f32),
// fill: Color32::WHITE,
// stroke: Default::default(),
// fill_texture_id: Default::default(),
// uv: eframe::egui::Rect::ZERO,
// };
// ui.painter().add(sample)
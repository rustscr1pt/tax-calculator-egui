use eframe::egui::{Button, Color32, Label, Pos2, Rect, RichText, Rounding, Slider, TextEdit, Ui, vec2, Widget};
use eframe::egui::TextStyle::Monospace;
use eframe::epaint::RectShape;
use crate::custom::{toggle};
use crate::Model::{ElementStyler, horizontal_padding, MainBody, Padding, text_formatter, TextStyler};
use crate::model_sql::fill_with_data;

pub const MULTIPLIER: f32 = 60f32;
pub const DISTANCE : f32 = 14f32;

impl MainBody {
    pub fn render_main(&mut self, ui : &mut Ui) -> () {
        self.draw_boxes(ui);
        ui.vertical(|ui| {
            ui.add_space(10f32);
            self.draw_header(ui);
            self.draw_elements(ui, ElementStyler::ProfitType);
            self.draw_elements(ui, ElementStyler::TaxType);
            self.draw_elements(ui, ElementStyler::InsuranceType);
            self.draw_elements(ui, ElementStyler::SpendsType);
            ui.add_space(8f32);
            self.draw_buttons(ui)
        });
    }

    pub fn draw_header(&self, ui : &mut Ui) -> () {
        ui.horizontal(|ui| {
            ui.add_space(15f32);
            ui.add(text_formatter("Введите данные ниже чтобы получить рассчет.", TextStyler::Annotation))
        });
    }

    pub fn draw_buttons(&mut self, ui : &mut Ui) -> () {
        ui.horizontal(|ui| {
            if ui.add(toggle(&mut self.smart_calculation)).clicked() {
                // No action cause bool has already been changed by a click.
            }
            ui.add_space(10f32);
            ui.add(text_formatter("Вычесть из налога страховые взносы", TextStyler::Annotation));
            ui.add_space(10f32);
            ui.scope(|ui| {
                let mut style = ui.style_mut();
                style.visuals.widgets.hovered.weak_bg_fill = Color32::BLACK;
                style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(53, 57, 53);
                if ui.add(Button::new(RichText::new("рассчитать").color(Color32::WHITE)).rounding(Rounding::from(15f32))).clicked() {
                    self.bottom_clicker();
                }
            })
        });
        ui.horizontal(|ui| {
            ui.add_space(316f32);
            ui.scope(|ui| {
                let mut style = ui.style_mut();
                style.visuals.widgets.hovered.weak_bg_fill = Color32::BLACK;
                style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(53, 57, 53);
                if ui.add(Button::new(RichText::new("монитор").color(Color32::WHITE)).rounding(Rounding::from(15f32))).clicked() {
                    self.mode_clicker();
                    fill_with_data(self.data_table_sender.clone(), self.sql_connection.clone(), self.month_filter_sender.clone());
                }
            })
        });
    }

    pub fn draw_boxes(&self, ui : &mut Ui) -> () {
        let mut draw_vec : Vec<RectShape> = Vec::with_capacity(4);
        for count in 0..4 {
            draw_vec.push(RectShape {
                rect: Rect {
                    min : Pos2::new(10f32, 40f32 + (MULTIPLIER * count as f32)),
                    max : Pos2::new(390f32, 85f32 + (MULTIPLIER * count as f32))
                },
                rounding: Rounding::from(8f32),
                fill: Color32::WHITE,
                stroke: Default::default(),
                fill_texture_id: Default::default(),
                uv: Rect::ZERO,
            })
        };
        for element in draw_vec {
            ui.painter().add(element);
        }
    }

    pub fn draw_elements(&mut self, ui : &mut Ui, styler_enum : ElementStyler) -> () {
        match styler_enum {
            ElementStyler::ProfitType => {
                ui.add_space(DISTANCE - 1f32);
                ui.scope(|ui| {
                    let style = ui.style_mut();

                    style.spacing.item_spacing = vec2(0f32, 0f32);
                    style.visuals.widgets.inactive.rounding = Rounding {
                        nw: 5f32,
                        ne: 5f32,
                        sw: 5f32,
                        se: 5f32,
                    };

                    ui.horizontal(|ui| {
                        ui.add_space(15f32);
                        ui.add(text_formatter("Ваш номинальный доход", TextStyler::BoxStyle))
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(15f32);
                        ui.add(TextEdit::singleline(&mut self.profit).font(Monospace).text_color(Color32::GREEN).desired_width(140f32))
                    })
                });
            }
            ElementStyler::TaxType => {
                ui.add_space(DISTANCE + 6f32);
                ui.scope(|ui| {
                    let style = ui.style_mut();

                    style.spacing.item_spacing = vec2(0f32, 0f32);
                    style.visuals.widgets.inactive.rounding = Rounding {
                        nw: 5f32,
                        ne: 5f32,
                        sw: 5f32,
                        se: 5f32,
                    };

                    ui.horizontal(|ui| {
                        ui.add_space(15f32);
                        ui.add(text_formatter("Ваша налоговая ставка", TextStyler::BoxStyle))
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(15f32);
                        ui.add(TextEdit::singleline(&mut self.tax).font(Monospace).text_color(Color32::GREEN).desired_width(140f32));
                        ui.add_space(10f32);
                        ui.add(text_formatter("процентов", TextStyler::BoxStyle))
                    })
                });
            }
            ElementStyler::InsuranceType => {
                ui.add_space(DISTANCE + 8f32);
                ui.scope(|ui| {
                    let style = ui.style_mut();

                    style.spacing.item_spacing = vec2(0f32, 0f32);
                    style.visuals.widgets.inactive.rounding = Rounding {
                        nw: 5f32,
                        ne: 5f32,
                        sw: 5f32,
                        se: 5f32,
                    };

                    ui.horizontal(|ui| {
                        ui.add_space(15f32);
                        ui.add(text_formatter("Введите годовой страховой взнос", TextStyler::BoxStyle))
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(15f32);
                        ui.add(TextEdit::singleline(&mut self.yearly_insurance).font(Monospace).text_color(Color32::GREEN).desired_width(140f32))
                    })
                });
            }
            ElementStyler::SpendsType => {
                ui.add_space(DISTANCE + 7f32);
                ui.scope(|ui| {
                    let style = ui.style_mut();

                    style.spacing.item_spacing = vec2(0f32, 0f32);
                    style.visuals.widgets.inactive.rounding = Rounding {
                        nw: 5f32,
                        ne: 5f32,
                        sw: 5f32,
                        se: 5f32,
                    };

                    ui.horizontal(|ui| {
                        ui.add_space(15f32);
                        ui.add(text_formatter("Введите дополнительные траты", TextStyler::BoxStyle))
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(15f32);
                        ui.add(TextEdit::singleline(&mut self.other_spends).font(Monospace).text_color(Color32::GREEN).desired_width(140f32));
                        ui.add_space(10f32);
                        //ui.add(text_formatter("(поставьте 0 если их нет)", TextStyler::BoxStyle))
                    })
                });
            }
        }
    }
}
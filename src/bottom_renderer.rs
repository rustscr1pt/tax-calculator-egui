use eframe::egui::{Button, Color32, Label, RichText, Rounding, ScrollArea, Ui};
use crate::Model::{MainBody, ModeType, text_formatter, TextStyler};

impl MainBody {
    pub fn render_bottom(&mut self, ui : &mut Ui) -> () {
        ui.vertical(|ui| {
            ui.add_space(3f32);
            ui.horizontal(|ui| {
                ui.add_space(5f32);
                ui.add(text_formatter("Итоги:", TextStyler::BottomTitle));
                ui.add_space(254f32);
                ui.scope(|ui| {
                    let mut style = ui.style_mut();
                    style.visuals.widgets.hovered.weak_bg_fill = Color32::BLACK;
                    if ui.add(Button::new(RichText::new("сбросить").color(Color32::WHITE)).rounding(Rounding::from(15f32))).clicked() {
                        self.reset_attr()
                    }
                })
            });
            ScrollArea::vertical().auto_shrink([false, false]).max_width(400f32).show(ui, |ui| {
                if self.smart_calculation {
                    self.scroll_builder(ui, ModeType::Smart)
                }
                else {
                    self.scroll_builder(ui, ModeType::Static)
                }
            });
            ui.add_space(10f32);
            if ui.add(Button::new("сбросить")).clicked() {}
        });
    }

    pub fn scroll_builder(&self, ui : &mut Ui, mode : ModeType) -> () {
        match mode {
            ModeType::Smart => {
                let (profit, taxes, insurance, spends, result) = self.tuple_calc(mode);
                ui.add(Label::new(RichText::new(format!("· Ваш номинальный доход : {}", f32_string(profit)))));
                ui.add(Label::new(RichText::new(format!("· Количество налогов за месяц : {}", f32_string(taxes)))));
                ui.add(Label::new(RichText::new(format!("· Страховые взносы за месяц {}", f32_string(insurance)))));
                ui.add(Label::new(RichText::new(format!("· Другие обязательные траты : {}", f32_string(spends)))));
                ui.add(Label::new(RichText::new(format!("· Чистый доход : {}", f32_string(result))).underline().color(Color32::WHITE)));
            }
            ModeType::Static => {
                let (profit, taxes, insurance, spends, result) = self.tuple_calc(mode);
                ui.add(Label::new(RichText::new(format!("· Ваш номинальный доход : {}", f32_string(profit)))));
                ui.add(Label::new(RichText::new(format!("· Количество налогов за месяц : {}", f32_string(taxes)))));
                ui.add(Label::new(RichText::new(format!("· Страховые взносы за месяц {}", f32_string(insurance)))));
                ui.add(Label::new(RichText::new(format!("· Другие обязательные траты : {}", f32_string(spends)))));
                ui.add(Label::new(RichText::new(format!("· Чистый доход : {}", f32_string(result))).underline().color(Color32::WHITE)));
            }
        }
    }

    pub fn tuple_calc(&self, mode : ModeType) -> (f32, f32, f32, f32, f32) {
        return match mode {
            ModeType::Smart => {
                let profit = string_f32(&self.profit);
                let taxes = string_f32(&self.profit) * 0.06;
                let insurance = string_f32(&self.yearly_insurance) / 12f32;
                let spends = string_f32(&self.other_spends);
                let result = profit - insurance - tax_exception(taxes, insurance) - spends;
                (profit, taxes, insurance, spends, result)
            }
            ModeType::Static => {
                let profit = string_f32(&self.profit);
                let taxes = string_f32(&self.profit) * 0.06;
                let insurance = string_f32(&self.yearly_insurance) / 12f32;
                let spends = string_f32(&self.other_spends);
                let result = profit - taxes - insurance - spends;
                (profit, taxes, insurance, spends, result)
            }
        }
    }
}

pub fn tax_exception(tax : f32, insurance : f32) -> f32 {
    let result = tax - insurance;
    if result >= 0f32 {
        return result
    }
    else { return 0f32 }
}

pub fn string_f32(object : &String) -> f32 {
    match object.trim().parse::<f32>() {
        Ok(ok) => {
            return ok
        }
        Err(_) => {
            return 0f32
        }
    }
}

pub fn f32_string(object : f32) -> String {format!("{:.1$}", object, 2)}
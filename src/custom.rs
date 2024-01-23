use eframe::egui;
use eframe::egui::{Color32, Pos2, Rect, Response, Rounding, Stroke, Ui, Widget};

pub fn custom_button(ui : &mut Ui, on : &mut bool) -> Response {
    let size = egui::vec2(30f32, 30f32);
    let (button_ui, mut response) = ui.allocate_exact_size(size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed()
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));
    if ui.is_rect_visible(button_ui) {
        let _ = ui.ctx().animate_bool(response.id, *on);
        let _ = ui.style().interact_selectable(&response, *on);
        ui.painter().rect(Rect{ min: Pos2::new(10f32, 270f32), max: Pos2::new(40f32, 300f32)}, Rounding::from(8f32), Color32::BLACK, Stroke::default());
        if *on {
            // checkmark_drawer(ui);
            ui.painter().circle(Pos2::new(25f32, 285f32), 10f32, Color32::BLACK, Stroke::new(3f32, Color32::GREEN));
        }
    }
    return response
}

pub fn toggle(on: &mut bool) -> impl Widget + '_ {
    move |ui: &mut Ui| custom_button(ui, on)
}

pub fn checkmark_drawer(ui : &mut Ui) -> () {
    ui.painter().line_segment([Pos2::new(20f32, 280f32), Pos2::new(30f32, 290f32)], Stroke::new(3f32, Color32::GREEN))
}

pub fn setup_custom_fonts(ctx : &egui::Context) -> () {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert("Custom".to_owned(), egui::FontData::from_static(include_bytes!(r#"C:\Users\User\RustroverProjects\tax_calculator\fonts\Tektur-Regular.ttf"#)));
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "Custom".to_string());
    ctx.set_fonts(fonts);
}


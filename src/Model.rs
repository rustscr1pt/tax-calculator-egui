use std::sync::{Arc};
use tokio::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender};
use eframe::egui::{Color32, FontFamily, Frame, Label, RichText, Rounding, Stroke, Vec2};
use eframe::epaint::Shadow;
use mysql::PooledConn;
use crate::custom::setup_custom_fonts;

pub struct BottomMonitorTextField {
    pub income : String,
    pub comment : String,
    pub is_possitive : bool
}

pub struct MainBody {
    pub data_table_sender : Sender<Vec<OfficeEvent>>,
    pub data_table_receiver : Receiver<Vec<OfficeEvent>>,
    pub data_table : Vec<OfficeEvent>,

    pub month_filter : Vec<String>,
    pub month_filter_sender : Sender<Vec<String>>,
    pub month_filter_receiver : Receiver<Vec<String>>,

    pub sql_connection : Arc<Mutex<PooledConn>>,

    pub profit : String,
    pub tax : String,
    pub yearly_insurance : String,
    pub other_spends: String,
    pub cleaned_profit : f32,
    pub bottom_active : bool,
    pub smart_calculation : bool,
    pub monitor_mode : bool,
    pub plus_shown : bool,
    pub monitor_bottom_panel : bool,
    
    pub bottom_monitor_texts : BottomMonitorTextField
}
#[derive(Debug, Clone)]
pub struct OfficeEvent {
    pub amount : u64,
    pub month : String,
    pub my_comment : String,
    pub is_possitive : u8,
    pub year : u16
}

pub enum BottomMonitorPlacer {
    Amount,
    Comment,
}

#[derive(PartialEq)]
pub enum ElementStyler {
    ProfitType,
    TaxType,
    InsuranceType,
    SpendsType
}

pub enum TextStyler {
    TitleStyle,
    BoxStyle,
    Annotation,
    BottomTitle,
    BottomScroll
}

pub enum Padding {
    Main,
    Bottom,
    Box
}

pub enum ModeType {
    Smart,
    Static
}

pub enum CalcType {
    TaxStatic,
    TaxSmart,
    Insurance,
    OtherSpends,
}

impl MainBody {
    pub fn new(cc : &eframe::CreationContext<'_>, connection : PooledConn) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        let (tx, rx) = std::sync::mpsc::channel();
        let (tx2, rx2) = std::sync::mpsc::channel();

        return MainBody {
            data_table_sender: tx,
            data_table_receiver: rx,
            data_table : Vec::new(), // Table for displaying at the second screen

            month_filter: Vec::new(), // Displays variants at the right click panel in the second screen
            month_filter_sender: tx2,
            month_filter_receiver: rx2,

            sql_connection : Arc::new(Mutex::new(connection)), // established connection with pool MySQL

            profit: String::from("0"),
            tax: String::from("6"),
            yearly_insurance: String::from("49500"),
            other_spends: String::from("0"),
            cleaned_profit: 0.0,
            bottom_active: false, // activates the bottom panel at the main screen
            smart_calculation : false, // toggles smart calculation for the first screen
            monitor_mode : false, // activates the second screen
            plus_shown : false,
            monitor_bottom_panel: false, // shows bottom panel at the second screen if activated
            bottom_monitor_texts: BottomMonitorTextField {
                income: "".to_string(),
                comment: "".to_string(),
                is_possitive: false,
            },
        }
    }

    pub fn reset_attr(&mut self) -> () {
        self.profit = String::from("0");
        self.tax = String::from("0");
        self.yearly_insurance = String::from("0");
        self.other_spends = String::from("0");
        self.cleaned_profit = 0.0;
        self.bottom_active = false;
        self.smart_calculation = false;
    }

    pub fn main_frame(&self) -> Frame {
        return Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Rounding {
                nw: 10f32,
                ne: 10f32,
                sw: 10f32,
                se: 10f32,
            },
            shadow: Shadow::big_dark(),
            fill: Color32::from_rgb(53, 57, 53),
            stroke: Stroke::new(2f32, Color32::from_rgb(0,0,0)),
        }
    }

    pub fn bottom_clicker(&mut self) -> () {
        if self.bottom_active {
            self.bottom_active = false
        }
        else {
            self.bottom_active = true
        }
    }

    pub fn smart_clicker(&mut self) -> () {
        if self.smart_calculation {
            self.smart_calculation = false
        }
        else {
            self.smart_calculation = true
        }
    }

    pub fn mode_clicker(&mut self) -> () {
        if self.monitor_mode {
            self.monitor_mode = !self.monitor_mode
        }
        else {
            self.monitor_mode = true
        }
    }

    pub fn bottom_panel_clicker(&mut self) -> () {
        if self.monitor_bottom_panel {
            self.monitor_bottom_panel = !self.monitor_bottom_panel
        }
        else {
            self.monitor_bottom_panel = true
        }
    }
}

pub fn text_formatter(text : &str, styled : TextStyler) -> Label {
    match styled {
        TextStyler::TitleStyle => {return Label::new(RichText::new(text).family(FontFamily::Proportional).size(15f32).color(Color32::WHITE))}
        TextStyler::BoxStyle => {return Label::new(RichText::new(text).monospace().size(12f32).color(Color32::BLACK))}
        TextStyler::Annotation => {return Label::new(RichText::new(text).size(12f32).color(Color32::WHITE))}
        TextStyler::BottomTitle => {return Label::new(RichText::new(text).size(12f32).underline().color(Color32::WHITE))}
        TextStyler::BottomScroll => {return Label::new(RichText::new(text).size(14f32).monospace().underline().color(Color32::WHITE))}
    }
}

pub fn horizontal_padding(x : f32, y : f32, background_type : Padding) -> (Vec2, Label) {
    return (Vec2::new(x, y), match background_type {
        Padding::Main => {Label::new(RichText::new(" ").color(Color32::from_rgb(53, 57, 53)))}
        Padding::Bottom => {Label::new(RichText::new(" ").color(Color32::BLACK))}
        Padding::Box => {Label::new(RichText::new(" ").color(Color32::WHITE))}
    })
}

pub fn load_icon(path : &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path).unwrap();
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        (rgba.into_raw(), width, height)
    };
    return eframe::IconData {
        rgba : icon_rgba,
        width : icon_width,
        height : icon_height
    }
}
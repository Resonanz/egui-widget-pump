use std::f32::consts::PI;

use egui::{pos2, vec2, Color32, CornerRadius, FontId, Id, Rect, Stroke, StrokeKind, Vec2};
use egui::{Response, Ui, Widget};

// This struct is the data structure stored in the
// pump hashmap and holds all pump info and status
//
// This struct is passed into fn new() as a reference.
// The PumpPortal fields become the shared references.
#[derive(Default, Debug)]
pub struct PumpData {
    pub name: String,
    pub pitch: f32,
    pub sound: bool,
    pub syringeset: Syringeset,
    pub linkset: Linkset,
    pub info: bool,
    pub uid: u32,
    pub pump_portal: PumpPortal,
}

// These become the shared references
// between widget and main code base
#[derive(Default, Debug)]
pub struct PumpPortal {
    pub dispense_button: PumpDispenseWithdrawState,
    pub withdraw_button: PumpDispenseWithdrawState,
    pub menu_syringe_icon: OffClicked,
    pub action: Actions,
}

/// Link set
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Syringeset {
    #[default]
    None,
    UL10, // 10 uL
    UL25,
    UL50,
    UL100,
    UL250,
    UL500,
    UL1000, // 1 mL
    UL2500,
    UL3000,
    UL5000,
    UL10000,
    UL25000,
    UL50000,
    UL100000, // 100 mL
}

/// Link set
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Linkset {
    #[default]
    None,
    A,
    B,
    C,
    D,
    E,
    F,
}

/// Pump mouse interaction states
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum OffOn {
    #[default]
    Off,
    On,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum OffClicked {
    #[default]
    Off,
    Clicked,
}

#[derive(Default, Debug, PartialEq)]
pub enum PumpDispenseWithdrawState {
    #[default]
    None,
    Pressed,
    Held,
}

#[derive(Default, Debug, PartialEq)]
pub enum Actions {
    #[default]
    None,
    PumpClicked,
    MenuSyringeClicked,
    MenuSpeakerClicked,
    MenuLinksetClicked,
    MenuFlowSettingsClicked,
    MenuPumpSettingsClicked,
    MenuInfoClicked,
    ButtonDispenseClicked,
    ButtonWithdrawClicked,
    ButtonDispenseHeld,
    ButtonWithdrawHeld,
}

/// Pump information store
///
/// ```mouse_state``` and ```color_state``` are references, used
/// as state-sharing portals between widget and main code bases.
///
/// ```img``` is the Pump image (e.g. PNG, SVG). Images are unknown
/// to the widget so must be initialized within the main code base.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Pump<'a> {
    pub action: &'a mut Actions,
    pub dispense_state: &'a mut PumpDispenseWithdrawState,
    pub withdraw_state: &'a mut PumpDispenseWithdrawState,
    pub menu_syringe_icon: &'a mut OffClicked,
    pub pitch: &'a mut f32,
    pub sound_state: &'a mut bool,
    pub syringeset: &'a mut Syringeset,
    pub linkset: &'a mut Linkset,
    pub info: &'a mut bool,
    pub name: &'a mut String,
}

/// Default values for the Pump struct...
impl<'a> Pump<'a> {
    pub fn new(pump_data: &'a mut PumpData) -> Self {
        Pump {
            // TX: Inputs directed to main codebase
            action: &mut pump_data.pump_portal.action,
            // RX: Data originating from main codebase
            dispense_state: &mut pump_data.pump_portal.dispense_button,
            withdraw_state: &mut pump_data.pump_portal.withdraw_button,
            menu_syringe_icon: &mut pump_data.pump_portal.menu_syringe_icon,
            pitch: &mut pump_data.pitch,
            sound_state: &mut pump_data.sound,
            syringeset: &mut pump_data.syringeset,
            linkset: &mut pump_data.linkset,
            info: &mut pump_data.info,
            name: &mut pump_data.name,
        }
    }
}

// ==================================================================
// ==================================================================
// ==================================================================

/// Widget trait to enable the Pump widget to be displayed
/// using the standard egui ```ui.add(Pump::new(...))```
///
impl Widget for Pump<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let menu_bar_color = ui.style().visuals.text_color();
        let menu_items_color = ui.style().visuals.extreme_bg_color;

        // Pump panel rectangle
        let pump_panel = Vec2::new(300.0, 200.0); // Fixed size per panel

        // Allocate pump panel
        let (response, painter) = ui.allocate_painter(pump_panel, egui::Sense::hover());
        let rect = response.rect;

        // name text (Pos2)
        let name_text_pos = rect.min + vec2(6.0, 5.0);

        // Syringe rectangle
        let syringe_icon_rect = Rect {
            min: rect.min + vec2(80.0, 4.0),
            max: rect.min + vec2(96.0, 20.0),
        };

        // Syringe text (Pos2)
        let syringe_text_pos = rect.min + vec2(100.0, 5.0);

        // Speaker SVG rectangle
        let speaker_rect = Rect {
            min: rect.min + vec2(240.0, 2.0),
            max: rect.min + vec2(260.0, 22.0),
        };

        // Link SVG
        let link_rect = Rect {
            min: rect.min + vec2(200.0, 2.0),
            max: rect.min + vec2(220.0, 22.0),
        };

        // Info SVG
        let info_rect = Rect {
            min: rect.min + vec2(270.0, 2.0),
            max: rect.min + vec2(290.0, 22.0),
        };

        // Left arrow button SVG
        let left_arrow_rect = Rect {
            min: rect.min + vec2(15.0, 60.0),
            max: rect.min + vec2(45.0, 90.0),
        };

        // Right arrow button SVG
        let right_arrow_rect = Rect {
            min: rect.min + vec2(255.0, 60.0),
            max: rect.min + vec2(285.0, 90.0),
        };

        // Pump SVG
        let pump_rect = Rect {
            min: rect.min + vec2(40.0, 40.0),
            max: rect.min + vec2(240.0, 100.0),
        };

        // Name on pump text (Pos2)
        let name_on_pump_text_pos = pos2(
            pump_rect.min.x + ((pump_rect.max.x - pump_rect.min.x) / 2.0),
            pump_rect.max.y - 11.0,
        );

        // Device panel
        painter.rect_filled(rect, 5.0, Color32::TRANSPARENT); // Background color
                                                              // Border stroke
        painter.rect_stroke(
            rect,
            4.0,
            Stroke::new(2.0, menu_bar_color),
            StrokeKind::Inside,
        );

        // Menu bar
        painter.rect_filled(
            Rect {
                min: rect.min,
                max: rect.min + vec2(rect.width(), 24.0),
            },
            CornerRadius {
                nw: 4,
                ne: 4,
                sw: 0,
                se: 0,
            },
            menu_bar_color,
        );

        // Pump name
        painter.text(
            name_text_pos,
            egui::Align2::LEFT_TOP,
            &self.name,
            FontId::proportional(12.0),
            menu_items_color,
        );

        // // Add a label inside the rectangle
        // painter.text(
        //     rect.min + vec2(180.0, 5.0),
        //     egui::Align2::LEFT_TOP,
        //     &self.pitch,
        //     FontId::proportional(12.0),
        //     Color32::BLUE,
        // );

        // Syringe SVG
        egui::Image::new(egui::include_image!("../assets/pics/syringe.svg"))
            .tint(menu_items_color)
            .paint_at(ui, syringe_icon_rect);
        // Manually check for mouse interaction

        // Syringe text
        let syringe_text_rect = painter.text(
            syringe_text_pos,
            egui::Align2::LEFT_TOP,
            match self.syringeset {
                Syringeset::None => "None",
                Syringeset::UL10 => "10 uL",
                Syringeset::UL25 => "25 uL",
                Syringeset::UL50 => "50 uL",
                Syringeset::UL100 => "100 uL",
                Syringeset::UL250 => "250 uL",
                Syringeset::UL500 => "500 uL",
                Syringeset::UL1000 => "1 mL",
                Syringeset::UL2500 => "2.5 mL",
                Syringeset::UL3000 => "3 mL",
                Syringeset::UL5000 => "5 mL",
                Syringeset::UL10000 => "10 mL",
                Syringeset::UL25000 => "25 mL",
                Syringeset::UL50000 => "50 mL",
                Syringeset::UL100000 => "100 mL",
            },
            FontId::proportional(12.0),
            menu_items_color,
        );

        let syringe_icon_plus_text_rect = Rect {
            min: syringe_icon_rect.min,
            max: pos2(syringe_text_rect.max.x, syringe_icon_rect.max.y),
        };
        // Syringe icon + text mouse click detection
        let ctx = ui.ctx();
        if ctx.pointer_latest_pos().map_or(false, |pos| {
            syringe_icon_plus_text_rect.contains(pos)
                && ctx.input(|i| i.pointer.button_released(egui::PointerButton::Primary))
        }) {
            ui.label("Syringe menu icon clicked!");
            *self.action = Actions::MenuSyringeClicked;
        }

        // Speaker SVG
        if *self.sound_state {
            egui::Image::new(egui::include_image!("../assets/pics/speaker-high.svg"))
                .tint(menu_items_color)
                .paint_at(ui, speaker_rect);
        } else {
            egui::Image::new(egui::include_image!("../assets/pics/speaker-x.svg"))
                .tint(menu_items_color)
                .paint_at(ui, speaker_rect);
        }
        // Manually check for mouse interaction
        let ctx = ui.ctx();
        if ctx.pointer_latest_pos().map_or(false, |pos| {
            speaker_rect.contains(pos)
                && ctx.input(|i| i.pointer.button_released(egui::PointerButton::Primary))
        }) {
            ui.label("Speaker menu icon clicked!");
            *self.action = Actions::MenuSpeakerClicked;
        }

        if *self.linkset == Linkset::None {
            egui::Image::new(egui::include_image!("../assets/pics/link-break-light.svg"))
                .tint(menu_items_color)
                .paint_at(ui, link_rect);
        } else {
            egui::Image::new(egui::include_image!("../assets/pics/link-light.svg"))
                .tint(menu_items_color)
                .paint_at(ui, link_rect);
        }
        // Manually check for mouse interaction
        let ctx = ui.ctx();
        if ctx.pointer_latest_pos().map_or(false, |pos| {
            link_rect.contains(pos)
                && ctx.input(|i| i.pointer.button_released(egui::PointerButton::Primary))
        }) {
            ui.label("Linkset menu icon clicked!");
            *self.action = Actions::MenuLinksetClicked;
        }

        // Link text
        painter.text(
            rect.min + vec2(220.0, 5.0),
            egui::Align2::LEFT_TOP,
            match self.linkset {
                Linkset::None => "",
                Linkset::A => "A",
                Linkset::B => "B",
                Linkset::C => "C",
                Linkset::D => "D",
                Linkset::E => "E",
                Linkset::F => "F",
            },
            FontId::proportional(12.0),
            menu_items_color,
        );

        egui::Image::new(egui::include_image!("../assets/pics/info.svg"))
            .tint(menu_items_color)
            .paint_at(ui, info_rect);
        // Manually check for mouse interaction
        let ctx = ui.ctx();
        if ctx.pointer_latest_pos().map_or(false, |pos| {
            info_rect.contains(pos)
                && ctx.input(|i| i.pointer.button_released(egui::PointerButton::Primary))
        }) {
            ui.label("Linkset menu icon clicked!");
            *self.action = Actions::MenuInfoClicked;
        }

        egui::Image::new(egui::include_image!("../assets/pics/triangle.svg"))
            .tint(Color32::GRAY)
            .rotate(PI / 6.0, Vec2::splat(0.5))
            .paint_at(ui, left_arrow_rect);
        // Manually check for mouse interaction
        let ctx = ui.ctx();
        if ctx.pointer_latest_pos().map_or(false, |pos| {
            left_arrow_rect.contains(pos)
                && ctx.input(|i| i.pointer.button_released(egui::PointerButton::Primary))
        }) {
            *self.action = Actions::ButtonDispenseClicked;
        }

        egui::Image::new(egui::include_image!("../assets/pics/triangle.svg"))
            .tint(Color32::GRAY)
            .rotate(-PI / 6.0, Vec2::splat(0.5))
            .paint_at(ui, right_arrow_rect);
        // Manually check for mouse interaction
        let ctx = ui.ctx();
        if ctx.pointer_latest_pos().map_or(false, |pos| {
            right_arrow_rect.contains(pos)
                && ctx.input(|i| i.pointer.button_released(egui::PointerButton::Primary))
        }) {
            *self.action = Actions::ButtonWithdrawClicked;
        }

        egui::Image::new(egui::include_image!("../assets/pics/pump.svg")).paint_at(ui, pump_rect);

        // Manually check if the mouse is over the syringe_rect
        let ctx = ui.ctx();
        if ctx.pointer_latest_pos().map_or(false, |pos| {
            pump_rect.contains(pos)
                && ctx.input(|i| i.pointer.button_released(egui::PointerButton::Primary))
        }) {
            *self.action = Actions::PumpClicked;
        }

        // Pump name on pump SVG
        painter.text(
            name_on_pump_text_pos,
            egui::Align2::CENTER_CENTER,
            // &self.name.to_uppercase(),
            &self.name,
            FontId::proportional(10.0),
            ui.style().visuals.text_color(),
        );

        // Dummy code to generate Response
        let image_rect = Rect {
            min: pos2(0.0, 0.0),
            max: pos2(300.0, 300.0),
        };

        // Create an interactable area over the image
        let response = ui.interact(image_rect, Id::new(123), egui::Sense::click());
        response
    }
}

// ==================================================================
// ==================================================================
// ==================================================================

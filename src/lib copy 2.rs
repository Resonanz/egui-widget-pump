pub mod constants;
use constants::{
    FRAME_INNER_MARGIN, FRAME_OUTER_MARGIN, FRAME_ROUNDING, FRAME_STROKE_COLOR,
    FRAME_STROKE_COLOR_HOVER, FRAME_STROKE_WIDTH, SHAPE_1_TOP_LEFT_X_Y_COORDS, SHAPE_1_X_Y_DIMS,
    SHAPE_2_TOP_LEFT_X_Y_COORDS, SHAPE_2_X_Y_DIMS,
};

use egui::{Color32, Frame, Margin, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

//
// ======================================================================
// ======================================================================
// ======================================================================
//

#[derive(Clone, Default)]
pub struct Config<'a> {
    text: &'a str,
    frame_size: Vec2,
    frame_fill: Color32,
    frame_fill_hover: Color32,
    frame_stroke: Stroke,
    frame_stroke_hover: Stroke,
    frame_inner_margin: Margin,
    frame_outer_margin: Margin,
    frame_rounding: Rounding,
    text_size: f32,
    // frame_outline_width: f32,
    // frame_outline_color: Color32,
}

//
// ======================================================================
// ======================================================================
// ======================================================================
//

// Builder starts here
pub struct ConfigBuilder<'a> {
    text: &'a str,
    frame_size: Vec2,
    frame_fill: Color32,
    frame_fill_hover: Color32,
    frame_stroke: Stroke,
    frame_stroke_hover: Stroke,
    frame_inner_margin: Margin,
    frame_outer_margin: Margin,
    frame_rounding: Rounding,
    text_size: f32,
    // frame_outline_width: f32,
    // frame_outline_color: Color32,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        ConfigBuilder {
            text: "Default text",
            frame_size: Vec2 { x: 300.0, y: 240.0 },
            frame_fill: Color32::BLACK,
            frame_fill_hover: Color32::from_black_alpha(123),
            frame_stroke: Stroke::new(2.0, Color32::from_gray(100)),
            frame_stroke_hover: Stroke::new(2.0, Color32::WHITE),
            frame_inner_margin: Margin::same(0),
            frame_outer_margin: Margin::same(0),
            frame_rounding: egui::CornerRadius::same(0),
            text_size: 16.0,
            // frame_outline_width: 1.0,
            // frame_outline_color: Color32::TRANSPARENT,
        }
    }

    pub fn text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    pub fn frame_size(mut self, frame_size: Vec2) -> Self {
        self.frame_size = frame_size;
        self
    }

    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    pub fn frame_inner_margin(mut self, inner_margin: Margin) -> Self {
        self.frame_inner_margin = inner_margin;
        self
    }

    pub fn frame_outer_margin(mut self, outer_margin: Margin) -> Self {
        self.frame_outer_margin = outer_margin;
        self
    }

    pub fn frame_rounding(mut self, rounding: Rounding) -> Self {
        self.frame_rounding = rounding;
        self
    }

    // pub fn frame_outline_width(mut self, frame_outline_width: f32) -> Self {
    //     self.frame_outline_width = frame_outline_width;
    //     self
    // }

    // pub fn frame_outline_color(mut self, frame_outline_color: Color32) -> Self {
    //     self.frame_outline_color = frame_outline_color;
    //     self
    // }

    pub fn build(self) -> Config<'a> {
        Config {
            text: self.text,
            frame_size: self.frame_size,
            frame_fill: self.frame_fill,
            frame_fill_hover: self.frame_fill_hover,
            frame_stroke: self.frame_stroke,
            frame_stroke_hover: self.frame_stroke_hover,
            frame_inner_margin: self.frame_inner_margin,
            frame_outer_margin: self.frame_outer_margin,
            frame_rounding: self.frame_rounding,
            text_size: self.text_size,
            // frame_outline_width: self.frame_outline_width,
            // frame_outline_color: self.frame_outline_color,
        }
    }
}
//
// ======================================================================
// ======================================================================
// ======================================================================
//

///
/// A number of different methods are used
/// to share information between your app
/// and the widget.
///
/// 1. A ```Config``` struct is used to configure
///    the properties of the widget.
/// 2. A ```SidebarTexicon``` struct is used to
///    share state information between the widget
///    and the main application.
///
///

#[derive(Default)]
enum Geometry {
    #[default]
    Rectangle,
    Triangle,
    Circle,
}

#[derive(Default, PartialEq)]
pub enum ShapeState {
    #[default]
    Off,
    On,
}

#[derive(Default, PartialEq)]
pub enum PumpMouseState {
    #[default]
    None,
    Clicked,
    Hovering,
}

#[derive(Default, PartialEq)]
pub enum PumpColorState {
    #[default]
    Dim,
    On,
    Highlight,
}
// WARNING: Complicated
//
// The purpose of this struct's contents is to
// create portals between the main code and this
// widget code. It does this using references and
// pointers to communicate via e.g. bools and enums.
//
// This struct is instantiated in the calling crate.
//
//
pub struct PumpItem<'a> {
    pub widget_hovered: bool,
    pub pump_mouse_fwd: PumpMouseState,
    pub pump_mouse_rev: PumpMouseState,
    pub pump_color_fwd: PumpColorState,
    pub pump_color_rev: PumpColorState,
    pub config: Config<'a>,
}

impl<'a> Default for PumpItem<'a> {
    // Can't seem to simplify this
    // using ..Default::default()
    // due to a recursion issue ?!
    fn default() -> Self {
        Self {
            widget_hovered: Default::default(),
            pump_mouse_fwd: Default::default(),
            pump_mouse_rev: Default::default(),
            pump_color_fwd: Default::default(),
            pump_color_rev: Default::default(),
            config: Default::default(),
        }
    }
}

impl<'a> PumpItem<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Self {
            widget_hovered: Default::default(),
            pump_mouse_fwd: Default::default(),
            pump_mouse_rev: Default::default(),
            pump_color_fwd: Default::default(),
            pump_color_rev: Default::default(),
            config, // config calls ConfigBuilder to populate
        }
    }
}

//
// ======================================================================
// ======================================================================
// ======================================================================
//

/// Pump is the portal that
/// stores shared state between the
/// widget crate and other crates.
//#[derive(Default)]
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Pump<'a> {
    widget_hovered: &'a mut bool,
    mouse_state_fwd: &'a mut PumpMouseState,
    mouse_state_rev: &'a mut PumpMouseState,
    color_state_fwd: &'a mut PumpColorState,
    color_state_rev: &'a mut PumpColorState,
    config: &'a Config<'a>,
}

impl<'a> Pump<'a> {
    pub fn new(p: &'a mut PumpItem) -> Self {
        Self {
            widget_hovered: &mut p.widget_hovered,
            mouse_state_fwd: &mut p.pump_mouse_fwd,
            mouse_state_rev: &mut p.pump_mouse_rev,
            color_state_fwd: &mut p.pump_color_fwd,
            color_state_rev: &mut p.pump_color_rev,
            config: &mut p.config,
        }
    }
}

impl<'a> Widget for Pump<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        /*

        This may be useful for this complex widget:

        // 4. Paint!
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

        */

        //
        // Define the frame
        //
        let frame = Frame::none()
            .inner_margin(FRAME_INNER_MARGIN)
            .outer_margin(FRAME_OUTER_MARGIN)
            //
            // Change frame fill if hovered
            //
            .fill(if *self.widget_hovered {
                self.config.frame_fill_hover
            } else {
                self.config.frame_fill
            })
            //
            // Change frame outline stroke if hovered
            //
            .stroke(if *self.widget_hovered {
                Stroke::new(FRAME_STROKE_WIDTH, FRAME_STROKE_COLOR_HOVER)
            } else {
                Stroke::new(FRAME_STROKE_WIDTH, FRAME_STROKE_COLOR)
            })
            .rounding(FRAME_ROUNDING);

        //
        // Show the frame
        //
        let frame_response = frame.show(ui, |ui| {
            // Set the minimum size of
            // the ui (that is, the frame)
            ui.set_min_size(self.config.frame_size);
            ui.set_max_size(self.config.frame_size); // Layout the icon and text vertically with some spacing

            //
            // Left rectangle
            //
            let left_response = draw_shape(
                ui,
                // self.shape_state_1,
                self.mouse_state_fwd,
                self.color_state_fwd,
                &ShapeStruct {
                    geometry: Geometry::Rectangle,
                    top_left_x_y_coords: SHAPE_1_TOP_LEFT_X_Y_COORDS,
                    x_y_dims: SHAPE_1_X_Y_DIMS,
                    ..Default::default()
                },
            );

            //
            // Right rectangle
            //
            let right_response = draw_shape(
                ui,
                // self.shape_state_2,
                self.mouse_state_rev,
                self.color_state_rev,
                &ShapeStruct {
                    geometry: Geometry::Rectangle,
                    top_left_x_y_coords: SHAPE_2_TOP_LEFT_X_Y_COORDS,
                    x_y_dims: SHAPE_2_X_Y_DIMS,
                    ..Default::default()
                },
            );

            let origin = ui.min_rect().min;
            let r: Rect = Rect {
                min: Pos2 {
                    x: origin.x + 0.0,
                    y: origin.y + 0.0,
                },
                max: Pos2 {
                    x: origin.x + 200.0,
                    y: origin.y + 20.0,
                },
            };

            // Draw a white background
            ui.painter().rect_filled(r, 0.0, Color32::WHITE);
            ui.put(
                r,
                egui::Label::new(
                    egui::RichText::new("Bold Black Text")
                        .color(Color32::BLACK)
                        .strong(),
                ),
            );

            /*
                       //
                       // Draw slider
                       //
                       // fn show_slider(ui: &mut egui::Ui, slider_val: &mut f32) {
                       // Define the exact position and size of the slider
                       let position = egui::pos2(left_rect.left() - 50.0, left_rect.bottom() + 20.0); // X and Y coordinates
                       let size = Vec2::new(150.0, 30.0); // Width and height
                       let rect = Rect::from_min_size(position, size);

                       ui.style_mut().spacing.slider_width = 150.;

                       // Place the slider at the defined rectangle
                       ui.put(
                           rect,
                           Slider::new(&mut *self.slider_val, 0.0..=100.0).text("My value"),
                       );

                       // Define rect for slider to fit into
                       let position = egui::pos2(left_rect.left() - 50.0, left_rect.bottom() + 44.0); // X and Y coordinates
                       let size = Vec2::new(120.0, 30.0); // Width and height
                       let rect = Rect::from_min_size(position, size);

                       // Place the slider at the defined rectangle
                       ui.put(
                           rect,
                           Slider::new(&mut *self.slider_val, 0.0..=100.0).text("My value"),
                       );
            */
            // Return both responses for combining
            (left_response, right_response)
        });

        // Combine the responses
        let mut response = frame_response.response;
        let (left_response, right_response) = frame_response.inner;

        // Update the response's clicked state based on either rectangle being clicked
        if left_response.clicked() || right_response.clicked() {
            response.mark_changed();
        }

        // Handle frame hover effect
        if response.contains_pointer() {
            *self.widget_hovered = true;
        } else {
            *self.widget_hovered = false;
        }
        response
    }
}

//
//
//
//
//
fn draw_shape(
    ui: &mut Ui,
    // shape_state: &mut ShapeState,
    mouse_state: &mut PumpMouseState,
    color_state: &mut PumpColorState,
    shape: &ShapeStruct,
) -> Response {
    // NOTE: ui refers to the frame
    //
    // POSITION THE RECTANGLE.
    //
    // Rect::from_min_size takes two vectors
    // which are two sets of x,y coords.
    //
    // Resulting rect is absolute coords
    // (x1, y1) and (x2, y2) of shape
    // relative to the frame.
    //
    let rect_x1_y1_x2_y2 = Rect::from_min_size(
        ui.min_rect().min /* frame origin */ + shape.top_left_x_y_coords,
        shape.x_y_dims,
    );

    let response: Response;

    match shape.geometry {
        Geometry::Rectangle => {
            response = ui.allocate_rect(rect_x1_y1_x2_y2, Sense::click());
            if response.clicked() {
                // *shape_on = !*shape_on;
                *mouse_state = PumpMouseState::Clicked;
            } else if response.hovered() {
                *mouse_state = PumpMouseState::Hovering;
            } else {
                *mouse_state = PumpMouseState::None;
            }
        }
        Geometry::Triangle => {
            response = ui.allocate_rect(rect_x1_y1_x2_y2, Sense::click());
            println!("Triangle")
        }
        Geometry::Circle => {
            response = ui.allocate_rect(rect_x1_y1_x2_y2, Sense::click());
            println!("Circle")
        }
    }

    //
    //
    //
    let color = match color_state {
        PumpColorState::Dim => shape.inactive_color,
        PumpColorState::On => shape.active_color,
        PumpColorState::Highlight => shape.hover_color,
    };

    //
    // Draw the shape
    //
    // Rectangles require rounding parameter
    //
    match shape.geometry {
        Geometry::Rectangle => {
            ui.painter()
                .rect_filled(rect_x1_y1_x2_y2, shape.rounding, color);
        }
        Geometry::Triangle => {
            println!("Triangle")
        }
        Geometry::Circle => {
            println!("Circle")
        }
    }

    response

    // let origin = Pos2 {
    //     x: config_frame.frame_min_rect_abs_coords.min.x,
    //     y: config_frame.frame_min_rect_abs_coords.min.y,
    // };

    // let my_wee_rect = Rect {
    //     min: egui::pos2(0., 0.),
    //     max: egui::pos2(33., 33.),
    // };

    // for i in 1..50 {
    //     let rect = my_wee_rect.translate(Vec2 {
    //         x: origin.x + i as f32,
    //         y: origin.y + i as f32,
    //     });

    //     ui.painter()
    //         .rect_filled(rect, widget_config.rect_rounding, color);
    // }
}

//
// ======================================================================
// ======================================================================
// ======================================================================
//

//
// Data structure defining a shape
//
struct ShapeStruct {
    /// The min_x_y_coords refer
    /// to the top left corner of
    /// the rectangle, relative
    /// to the containing frame.
    geometry: Geometry,
    top_left_x_y_coords: Vec2,
    x_y_dims: Vec2,
    inactive_color: Color32,
    active_color: Color32,
    hover_color: Color32,
    rounding: f32,
}

//
// Default values
//
impl Default for ShapeStruct {
    fn default() -> Self {
        Self {
            geometry: Default::default(),
            top_left_x_y_coords: Vec2::default(),
            x_y_dims: Vec2::default(),
            inactive_color: Color32::from_gray(180),
            active_color: Color32::from_rgb(100, 200, 100),
            hover_color: Color32::from_rgb(150, 220, 150),
            rounding: 4.0,
        }
    }
}

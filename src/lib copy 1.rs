pub mod constants;
use constants::{
    FRAME_INNER_MARGIN, FRAME_OUTER_MARGIN, FRAME_ROUNDING, FRAME_STROKE_COLOR,
    FRAME_STROKE_COLOR_HOVER, FRAME_STROKE_WIDTH, SHAPE_1_TOP_LEFT_X_Y_COORDS, SHAPE_1_X_Y_DIMS,
    SHAPE_2_TOP_LEFT_X_Y_COORDS, SHAPE_2_X_Y_DIMS,
};

use egui::{Color32, Frame, Pos2, Rect, Response, Sense, Slider, Stroke, Ui, Vec2, Widget};
///
/// How this widget works
///
/// DEPENDENCY
/// The widget is a crate (of code) that is
/// imported in cargo.toml as a dependency.
/// It can then be used directly in app.rs.
/// e.g. ui.add(egui_widget_pump::etc...
///
/// ACCESSING WIDGET STATE
///
/// The widget contains objects such as
/// rectangles, text, etc. which are made
/// accessible in app.rs by calling the
/// WidgetState::new() constructor. The
/// struct fields are references e.g.
///
/// button_on_1: &'a mut bool
///
/// that are stored persistently in app.rs
/// from Self returned from the constructor.
///
/// DEFINING THE WIDGET META PROPERTIES
///
///
/// and each object
/// may need to maintain its state (e.g.
/// a shape may need to maintain its color
/// ),
///

/// Configuration for the
/// widget appearance
//#[derive(Clone)]
struct WidgetFrame {
    // Frame config
    size: Vec2,
    stroke: Stroke,
    stroke_hover: Stroke,
    fill: Color32,
    fill_hover: Color32,
}

impl Default for WidgetFrame {
    fn default() -> Self {
        Self {
            // Frame
            size: Vec2::new(300.0, 240.0),
            stroke: Stroke::new(2.0, Color32::from_gray(100)),
            stroke_hover: Stroke::new(2.0, Color32::WHITE),
            fill: Color32::BLACK,
            fill_hover: Color32::from_black_alpha(123),
        }
    }
}
//
// ======================================================================
// ======================================================================
// ======================================================================
//

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
pub enum MouseState {
    #[default]
    Off,
    Clicked,
    Hovering,
}

#[derive(Default, PartialEq)]
pub enum ColorState {
    #[default]
    Off,
    On,
    OnBright,
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
//
// ======================================================================
// ======================================================================
// ======================================================================
//

/// WidgetState is the portal that
/// stores shared state between the
/// widget crate and other crates.
//#[derive(Default)]
pub struct WidgetState<'a> {
    widget_hovered: &'a mut bool,
    shape_state_1: &'a mut ShapeState, // Button 1
    mouse_state_1: &'a mut MouseState,
    color_state_1: &'a mut ColorState,
    shape_state_2: &'a mut ShapeState, // Button 2
    mouse_state_2: &'a mut MouseState,
    color_state_2: &'a mut ColorState,
    slider_val: &'a mut f32, // Slider 1
}

impl<'a> WidgetState<'a> {
    pub fn new(
        widget_hovered: &'a mut bool,
        shape_state_1: &'a mut ShapeState, // Button 1
        mouse_state_1: &'a mut MouseState,
        color_state_1: &'a mut ColorState,
        shape_state_2: &'a mut ShapeState, // Button 2
        mouse_state_2: &'a mut MouseState,
        color_state_2: &'a mut ColorState,
        slider_val: &'a mut f32, // Slider 1
    ) -> Self {
        Self {
            widget_hovered,
            shape_state_1, // Button 1
            mouse_state_1,
            color_state_1,
            shape_state_2, // Button 2
            mouse_state_2,
            color_state_2,
            slider_val, // Slider 1
        }
    }
}

impl<'a> Widget for WidgetState<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        /*

        This may be useful for this complex widget:

        // 4. Paint!
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

        */

        // Instantiate the required
        // widget building blocks
        let widget_frame = WidgetFrame::default();

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
                widget_frame.fill_hover
            } else {
                widget_frame.fill
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
            ui.set_min_size(widget_frame.size);

            //
            // Left rectangle
            //
            let left_response = draw_shape(
                ui,
                // self.shape_state_1,
                self.mouse_state_1,
                self.color_state_1,
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
                self.mouse_state_2,
                self.color_state_2,
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
    mouse_state: &mut MouseState,
    color_state: &mut ColorState,
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
                *mouse_state = MouseState::Clicked;
            } else if response.hovered() {
                *mouse_state = MouseState::Hovering;
            } else {
                *mouse_state = MouseState::Off;
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
        ColorState::Off => shape.inactive_color,
        ColorState::On => shape.active_color,
        ColorState::OnBright => shape.hover_color,
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

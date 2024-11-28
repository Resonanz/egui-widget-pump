use egui::{Color32, Frame, Pos2, Rect, Response, Sense, Slider, Stroke, Ui, Vec2, Widget};

/// Configuration for the
/// widget appearance
//#[derive(Clone)]
pub struct ConfigFrame {
    // Frame config
    frame_size: Vec2,
    frame_rounding: f32,
    frame_stroke: Stroke,
    frame_stroke_hover: Stroke,
    frame_fill: Color32,
    frame_fill_hover: Color32,
}

impl Default for ConfigFrame {
    fn default() -> Self {
        Self {
            // Frame
            frame_size: Vec2::new(300.0, 240.0),
            frame_rounding: 14.0,
            frame_stroke: Stroke::new(2.0, Color32::from_gray(100)),
            frame_stroke_hover: Stroke::new(2.0, Color32::WHITE),
            frame_fill: Color32::BLACK,
            frame_fill_hover: Color32::from_black_alpha(123),
        }
    }
}

//
// Data structure defining
// a button rectangle
//
pub struct ButtonRectStruct {
    /// The min_x_y_coords refer
    /// to the top left corner of
    /// the rectangle, relative
    /// to the containing frame.
    abs_min_x_y_coords: Vec2,
    x_y_dims: Vec2,
    inactive_color: Color32,
    active_color: Color32,
    hover_color: Color32,
    rounding: f32,
}

//
// Default values
//
impl Default for ButtonRectStruct {
    fn default() -> Self {
        Self {
            abs_min_x_y_coords: Vec2::default(),
            x_y_dims: Vec2::default(),
            inactive_color: Color32::from_gray(180),
            active_color: Color32::from_rgb(100, 200, 100),
            hover_color: Color32::from_rgb(150, 220, 150),
            rounding: 4.0,
        }
    }
}

/// WidgetState is the portal that
/// stores shared state between the
/// widget crate and other crates.
//#[derive(Default)]
pub struct WidgetState<'a> {
    // state: ToggleState<'a>,
    // config: ToggleRectsConfig,
    button_on_1: &'a mut bool,
    button_on_2: &'a mut bool,
    button_pressed_1: &'a mut bool,
    button_pressed_2: &'a mut bool,
    widget_hovered: &'a mut bool,
    slider_val: &'a mut f32,
}

impl<'a> WidgetState<'a> {
    pub fn new(
        button_on_1: &'a mut bool,
        button_on_2: &'a mut bool,
        button_pressed_1: &'a mut bool,
        button_pressed_2: &'a mut bool,
        widget_hovered: &'a mut bool,
        slider_val: &'a mut f32,
    ) -> Self {
        Self {
            button_on_1,
            button_on_2,
            button_pressed_1,
            button_pressed_2,
            widget_hovered,
            slider_val,
        }
    }
}

//
//
//
//
//
impl<'a> Widget for WidgetState<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Instantiate the required
        // widget building blocks
        let config_frame = ConfigFrame::default();

        //
        // Define the frame
        //
        let frame = Frame::none()
            .inner_margin(10.0)
            .outer_margin(10.0)
            //
            // Change frame fill if hovered
            //
            .fill(if *self.widget_hovered {
                config_frame.frame_fill_hover
            } else {
                config_frame.frame_fill
            })
            //
            // Change frame outline stroke if hovered
            //
            .stroke(if *self.widget_hovered {
                config_frame.frame_stroke_hover
            } else {
                config_frame.frame_stroke
            })
            .rounding(config_frame.frame_rounding);

        //
        // Show the frame
        //
        let frame_response = frame.show(ui, |ui| {
            // Set the minimum size of the ui (that is, the frame)
            ui.set_min_size(config_frame.frame_size);

            //
            // Left rectangle
            //
            //            let left_response = self.draw_rect(
            let left_response = draw_rect(
                ui,
                self.button_on_1,
                self.button_pressed_1,
                &ButtonRectStruct {
                    abs_min_x_y_coords: Vec2 { x: 50.0, y: 50.0 },
                    x_y_dims: Vec2 { x: 50.0, y: 40.0 },
                    ..Default::default()
                },
            );

            //
            // Right rectangle
            //
            let right_response = draw_rect(
                ui,
                self.button_on_2,
                self.button_pressed_2,
                &ButtonRectStruct {
                    abs_min_x_y_coords: Vec2 { x: 150.0, y: 150.0 },
                    x_y_dims: Vec2 { x: 50.0, y: 40.0 },
                    ..Default::default()
                },
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
fn draw_rect(
    ui: &mut Ui,
    rect_on: &mut bool,
    rect_pressed: &mut bool,
    rectangle: &ButtonRectStruct,
) -> Response {
    let rect = Rect::from_min_size(
        // Position the rectangle.
        //
        // Rect::from_min_size takes two vectors
        // which are two sets of x,y coords.
        //
        // First, find abs pos of ui (frame) origin (0,0).
        // Then, add button coordinate offset. This would
        // be the top left corner.
        //
        // Second, find the bottom right corner.
        // Combined, these provide abs coords
        // (x1, y1) and (x2, y2) of rectangle.
        ui.min_rect().min + rectangle.abs_min_x_y_coords,
        rectangle.x_y_dims,
    );
    let response = ui.allocate_rect(rect, Sense::click());

    if response.clicked() {
        //   *self.button_on_2 = !*self.button_on_2;
        // *self.button_pressed_2 = true;
        *rect_on = !*rect_on;
        *rect_pressed = true;
        *rect_on = true;
    }

    //
    //
    //

    let color = if *rect_on {
        if response.hovered() {
            rectangle.hover_color
        } else {
            rectangle.active_color
        }
    } else {
        if response.hovered() {
            rectangle.hover_color
        } else {
            rectangle.inactive_color
        }
    };

    ui.painter().rect_filled(rect, rectangle.rounding, color);

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

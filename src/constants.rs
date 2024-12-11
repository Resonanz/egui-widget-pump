use egui::{Color32, Vec2};

pub const FRAME_SIZE: Vec2 = Vec2::new(300.0, 240.0);
pub const FRAME_INNER_MARGIN: f32 = 4.0;
pub const FRAME_OUTER_MARGIN: f32 = 4.0;
pub const FRAME_ROUNDING: f32 = 4.0;
pub const FRAME_FILL: Color32 = Color32::BLACK;
pub const FRAME_FILL_HOVER: Color32 = Color32::from_black_alpha(123);
pub const FRAME_STROKE_WIDTH: f32 = 2.0;
pub const FRAME_STROKE_COLOR: Color32 = Color32::from_gray(100);
pub const FRAME_STROKE_COLOR_HOVER: Color32 = Color32::WHITE;

// Shape (button) 1
// x-y coordinates are relative to the surrounding frame
pub const SHAPE_1_TOP_LEFT_X_Y_COORDS: Vec2 = Vec2 { x: 50.0, y: 50.0 };
pub const SHAPE_1_X_Y_DIMS: Vec2 = Vec2 { x: 50.0, y: 40.0 };

// Shape (button) 2
// x-y coordinates are relative to the surrounding frame
pub const SHAPE_2_TOP_LEFT_X_Y_COORDS: Vec2 = Vec2 { x: 150.0, y: 50.0 };
pub const SHAPE_2_X_Y_DIMS: Vec2 = Vec2 { x: 80.0, y: 40.0 };

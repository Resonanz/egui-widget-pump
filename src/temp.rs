fn draw_rect(painter: &egui::Painter, rect: Rect, ui: &mut Ui, ctx: &egui::Context, i: usize) {
    let start = Instant::now();

    painter.rect_filled(rect, 5.0, Color32::TRANSPARENT); // Background color
    painter.rect_stroke(
        rect,
        4.0,
        Stroke::new(2.0, Color32::WHITE),
        StrokeKind::Inside,
    ); // Border stroke
    painter.rect_filled(
        Rect {
            // min: rect.min,
            // max: pos2(rect.max.x, 24.0),
            min: rect.min,
            // max: pos2(rect.max.x, 24.0),
            max: rect.min + vec2(rect.width(), 24.0),
        },
        CornerRadius {
            nw: 4,
            ne: 4,
            sw: 0,
            se: 0,
        },
        Color32::WHITE,
    ); // Border stroke
       // Add a label inside the rectangle
    painter.text(
        rect.min + vec2(10.0, 6.0),
        egui::Align2::LEFT_TOP,
        "Label",
        FontId::proportional(12.0),
        Color32::BLUE,
    );

    // Syringe SVG
    let syringe_rect = Rect {
        min: rect.min + vec2(60.0, 2.0),
        max: rect.min + vec2(80.0, 22.0),
    };

    egui::Image::new(egui::include_image!("../assets/syringe.svg"))
        .tint(egui::Color32::RED)
        .paint_at(ui, syringe_rect);

    // Manually check for mouse interaction
    if ctx.pointer_latest_pos().map_or(false, |pos| {
        syringe_rect.contains(pos)
            && ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary))
    }) {
        ui.label("Rectangle clicked!");
    }

    // Pump SVG
    let pump_rect = Rect {
        min: rect.min + vec2(40.0, 40.0),
        max: rect.min + vec2(240.0, 100.0),
    };

    egui::Image::new(egui::include_image!("../assets/pump.svg")).paint_at(ui, pump_rect);

    // Manually check if the mouse is over the syringe_rect
    let mouse_pos = ctx.pointer_latest_pos(); // Get the latest mouse position
    let clicked = mouse_pos.map_or(false, |pos| {
        pump_rect.contains(pos)
            && ctx.input(|i| i.pointer.button_down(egui::PointerButton::Primary))
    });

    // If the rectangle was clicked, show a label
    if clicked {
        ui.label("Pump clicked!");
    }

    let duration = start.elapsed();
    println!("Elapsed time = {:?}", duration);
}

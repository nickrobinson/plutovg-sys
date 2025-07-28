// examples/smiley.rs
use plutovg_sys::*;
use std::ffi::CString;

fn main() {
    unsafe {
        const WIDTH: i32 = 150;
        const HEIGHT: i32 = 150;
        let center_x = WIDTH as f32 / 2.0;
        let center_y = HEIGHT as f32 / 2.0;
        let face_radius = 70.0;
        let mouth_radius = 50.0;
        let eye_radius = 10.0;
        let eye_offset_x = 25.0;
        let eye_offset_y = 20.0;
        let eye_x = center_x - eye_offset_x;
        let eye_y = center_y - eye_offset_y;

        // Create surface and canvas
        let surface = plutovg_surface_create(WIDTH, HEIGHT);
        if surface.is_null() {
            panic!("Failed to create surface");
        }

        let canvas = plutovg_canvas_create(surface);
        if canvas.is_null() {
            plutovg_surface_destroy(surface);
            panic!("Failed to create canvas");
        }

        // Draw face
        plutovg_canvas_save(canvas);
        plutovg_canvas_arc(
            canvas,
            center_x,
            center_y,
            face_radius,
            0.0,
            PLUTOVG_TWO_PI,
            false,
        );
        plutovg_canvas_set_rgb(canvas, 1.0, 1.0, 0.0); // Yellow
        plutovg_canvas_fill_preserve(canvas);
        plutovg_canvas_set_rgb(canvas, 0.0, 0.0, 0.0); // Black
        plutovg_canvas_set_line_width(canvas, 5.0);
        plutovg_canvas_stroke(canvas);
        plutovg_canvas_restore(canvas);

        // Draw eyes
        plutovg_canvas_save(canvas);
        plutovg_canvas_arc(canvas, eye_x, eye_y, eye_radius, 0.0, PLUTOVG_TWO_PI, false);
        plutovg_canvas_arc(
            canvas,
            center_x + eye_offset_x,
            eye_y,
            eye_radius,
            0.0,
            PLUTOVG_TWO_PI,
            false,
        );
        plutovg_canvas_set_rgb(canvas, 0.0, 0.0, 0.0); // Black
        plutovg_canvas_fill(canvas);
        plutovg_canvas_restore(canvas);

        // Draw mouth
        plutovg_canvas_save(canvas);
        plutovg_canvas_arc(
            canvas,
            center_x,
            center_y,
            mouth_radius,
            0.0,
            PLUTOVG_PI,
            false,
        );
        plutovg_canvas_set_rgb(canvas, 0.0, 0.0, 0.0); // Black
        plutovg_canvas_set_line_width(canvas, 5.0);
        plutovg_canvas_stroke(canvas);
        plutovg_canvas_restore(canvas);

        // Save to PNG
        let filename = CString::new("smiley.png").unwrap();
        let result = plutovg_surface_write_to_png(surface, filename.as_ptr());
        if result != true {
            eprintln!("Failed to write PNG file");
        } else {
            println!("Successfully created smiley.png");
        }

        // Clean up
        plutovg_canvas_destroy(canvas);
        plutovg_surface_destroy(surface);
    }
}

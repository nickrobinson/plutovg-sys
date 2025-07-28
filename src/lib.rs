#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Define constants as f32 to match function signatures
pub const PLUTOVG_PI: f32 = 3.141592653589793_f32;
pub const PLUTOVG_TWO_PI: f32 = 6.283185307179586_f32;
pub const PLUTOVG_HALF_PI: f32 = 1.5707963267948966_f32;
pub const PLUTOVG_SQRT2: f32 = 1.4142135623730951_f32;
pub const PLUTOVG_KAPPA: f32 = 0.5522847498307935_f32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface() {
        unsafe {
            let surface = plutovg_surface_create(400, 400);
            assert!(!surface.is_null());
            plutovg_surface_destroy(surface);
        }
    }

    #[test]
    fn test_create_canvas() {
        unsafe {
            let surface = plutovg_surface_create(400, 400);
            let canvas = plutovg_canvas_create(surface);
            assert!(!canvas.is_null());
            plutovg_canvas_destroy(canvas);
            plutovg_surface_destroy(surface);
        }
    }
}

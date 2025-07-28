use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rerun-if-changed=vendor/plutovg");

    // Build PlutoVG
    cc::Build::new()
        .file("vendor/plutovg/source/plutovg-surface.c")
        .file("vendor/plutovg/source/plutovg-canvas.c")
        .file("vendor/plutovg/source/plutovg-paint.c")
        .file("vendor/plutovg/source/plutovg-rasterize.c")
        .file("vendor/plutovg/source/plutovg-blend.c")
        .file("vendor/plutovg/source/plutovg-matrix.c")
        .file("vendor/plutovg/source/plutovg-path.c")
        .file("vendor/plutovg/source/plutovg-font.c")
        .file("vendor/plutovg/source/plutovg-ft-raster.c")
        .file("vendor/plutovg/source/plutovg-ft-stroker.c")
        .file("vendor/plutovg/source/plutovg-ft-math.c")
        .include("vendor/plutovg/include")
        .include("vendor/plutovg/source")
        .warnings(false)
        .compile("plutovg");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .blocklist_item("PLUTOVG_PI")
        .blocklist_item("PLUTOVG_TWO_PI")
        .blocklist_item("PLUTOVG_HALF_PI")
        .blocklist_item("PLUTOVG_SQRT2")
        .blocklist_item("PLUTOVG_KAPPA")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

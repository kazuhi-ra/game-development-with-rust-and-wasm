use wasm_bindgen::prelude::*;
use web_sys::console;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));
    console::info_1(&JsValue::from_str("unko"));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    draw_ierpinski_gasket(&context, 3);
    Ok(())
}

fn draw_ierpinski_gasket(context: &web_sys::CanvasRenderingContext2d, depth: u8) -> () {
    draw_triangle(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)]);
    draw_triangle(&context, [(300.0, 0.0), (150.00, 300.0), (450.0, 300.0)]);
    draw_triangle(&context, [(150.0, 300.0), (0.0, 600.0), (300.0, 600.0)]);
    draw_triangle(&context, [(450.0, 300.0), (300.0, 600.0), (600.0, 600.0)]);
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3]) -> () {
    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
}

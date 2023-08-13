use rand::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Your code goes here!
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let image = web_sys::HtmlImageElement::new().unwrap();
    image.set_src("walk_the_dog_assets-0.0.7/resized/rhb/Idle (1).png");

    Ok(())
}

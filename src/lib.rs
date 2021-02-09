use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


    let count = 7;

    let mut x_pos = 10.0;
    let mut y_pos = 10.0;
    let radius = 5.0;

    let mut r = 000;
    let mut g = 050;
    let mut b = 100;

    for _ in 0..count {
        
        let color = format!("rgb({}, {}, {})", r, g, b);
        context.begin_path();
        context.set_fill_style(&JsValue::from_str(&color));
        context.set_stroke_style(&JsValue::from_str(&color));

        // Draw the outer circle.
        context
            .arc(x_pos, y_pos, radius, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.fill();
        context.stroke();
        
        x_pos += 10.0;
        y_pos += 10.0;
        r += 10;
        g += 10;
        b -= 10;
    }


}

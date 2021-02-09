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
    let radius = 2.0;


    for _ in 0..count {
        
        context.begin_path();
        context.set_fill_style(&JsValue::from_str("green"));
        context.set_stroke_style(&JsValue::from_str("blue"));

        // Draw the outer circle.
        context
            .arc(x_pos, y_pos, radius, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.stroke();
        
        x_pos += 10.0;
        y_pos += 10.0;
    }


}

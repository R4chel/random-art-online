use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod circle;
use crate::circle::Circle;

fn draw_circle(context: &web_sys::CanvasRenderingContext2d, circle: &Circle) {
    let color = JsValue::from_str(&circle.color());
    context.begin_path();
    context.set_fill_style(&color);
    context.set_stroke_style(&color);

    context
        .arc(
            circle.x_position(),
            circle.y_position(),
            circle.radius(),
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    context.fill();
    context.stroke();
}

fn clear_board() {
    web_sys::console::log(&js_sys::Array::from(&JsValue::from_str("CLEAR")));
    let canvas = canvas();
    let context = context();

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}

fn make_unanimated_art() {
    let context = context();
    let mut circle = Circle::new();

    let count = count_slider_value();
    let position_delta = distance_slider_value();
    let color_delta = color_slider_value() as u8;

    for _ in 0..count {
        draw_circle(&context, &circle);
        circle.update(position_delta, color_delta);
    }
}
fn make_animated_art() {
    web_sys::console::log(&js_sys::Array::from(&JsValue::from_str(
        "I love making art!",
    )));
    let context = context();
    let mut circle = Circle::new();
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    let count = count_slider_value();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > count {
            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        i += 1;

        let position_delta = distance_slider_value();
        let color_delta = color_slider_value() as u8;

        draw_circle(&context, &circle);
        circle.update(position_delta, color_delta);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

fn context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

fn color_slider_value() -> u8 {
    document()
        .get_element_by_id("colorSlider")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value_as_number() as u8
}

fn count_slider_value() -> u32 {
    document()
        .get_element_by_id("countSlider")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value_as_number() as u32
}

fn distance_slider_value() -> f64 {
    document()
        .get_element_by_id("distanceSlider")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value_as_number()
}

#[wasm_bindgen(start)]
pub fn start() {
    web_sys::console::log(&js_sys::Array::from(&JsValue::from_str(
        "I love printf debugging!",
    )));

    let document = document();

    let trash_button = document
        .get_element_by_id("trashButton")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();

    let trash_onclick_handler = Closure::wrap(Box::new(move || {
        clear_board();
    }) as Box<dyn FnMut()>);
    trash_button.set_onclick(Some(trash_onclick_handler.as_ref().unchecked_ref()));
    trash_onclick_handler.forget();

    let art_button = document
        .get_element_by_id("artButton")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();

    let art_onclick_handler = Closure::wrap(Box::new(move || {
        make_animated_art();
    }) as Box<dyn FnMut()>);
    art_button.set_onclick(Some(art_onclick_handler.as_ref().unchecked_ref()));
    art_onclick_handler.forget();

    let still_button = document
        .get_element_by_id("stillButton")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();

    let still_onclick_handler = Closure::wrap(Box::new(move || {
        make_unanimated_art();
    }) as Box<dyn FnMut()>);
    still_button.set_onclick(Some(still_onclick_handler.as_ref().unchecked_ref()));
    still_onclick_handler.forget();
}

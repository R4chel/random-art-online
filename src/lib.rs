use js_sys::Math::random;
use std::cell::RefCell;
use std::f64;
use std::fmt::{self, Display};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::EventListener;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
}

const POS_DELTA: f64 = 2.64;
const MIN_POS: f64 = 0.0;
const MAX_X_POS: f64 = 500.0;
const MAX_Y_POS: f64 = 250.0;
const RADIUS: f64 = 2.2;
impl Position {
    fn rand() -> Self {
        Position {
            x: f64::floor(random() * (MAX_X_POS - MIN_POS) as f64) + MIN_POS,
            y: f64::floor(random() * (MAX_Y_POS - MIN_POS) as f64) + MIN_POS,
        }
    }

    fn validate(&self) -> bool {
        self.x > MIN_POS && self.x < MAX_X_POS && self.y > MIN_POS && self.y < MAX_Y_POS
    }

    fn update(&mut self) {
        let mut options: Vec<Self> = Vec::new();

        for x_multiplier in -1..=1 {
            for y_multiplier in -1..=1 {
                let new_position = Position {
                    x: self.x + POS_DELTA * (x_multiplier as f64),
                    y: self.y + POS_DELTA * (y_multiplier as f64),
                };

                if (x_multiplier == 0 && y_multiplier == 0) || !new_position.validate() {
                    continue;
                } else {
                    options.push(new_position)
                };
            }
        }

        let random_index = f64::floor(random() * options.len() as f64) as usize;

        let random_element = &options[random_index];

        self.x = random_element.x;
        self.y = random_element.y;
    }
}

#[derive(Debug)]
struct ColorBit {
    bit: u8,
}

impl Display for ColorBit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.bit)
    }
}

impl ColorBit {
    fn rand() -> Self {
        ColorBit {
            bit: f64::floor(random() * 255 as f64) as u8,
        }
    }

    fn update(&mut self, color_delta: u8) -> () {
        if random() > 0.5 {
            self.bit = self.bit.saturating_add(color_delta);
        } else {
            self.bit = self.bit.saturating_sub(color_delta);
        };
    }
}

#[derive(Debug)]
struct Color {
    r: ColorBit,
    g: ColorBit,
    b: ColorBit,
}

impl Color {
    fn rand() -> Self {
        Color {
            r: ColorBit::rand(),
            g: ColorBit::rand(),
            b: ColorBit::rand(),
        }
    }

    fn to_js_value(&self) -> JsValue {
        JsValue::from_str(&format!(
            "rgb({}, {}, {})",
            self.r.bit, self.g.bit, self.b.bit
        ))
    }

    fn update(&mut self, color_delta: u8) {
        let update_with_delta = move | x | ColorBit::update(x, color_delta);

        update_with_delta(&mut self.r);
        update_with_delta(&mut self.g);
        update_with_delta(&mut self.b);
    }
}

#[derive(Debug)]
struct Circle {
    position: Position,
    color: Color,
    radius: f64,
}

impl Circle {
    fn new() -> Self {
        Circle {
            position: Position::rand(),
            color: Color::rand(),
            radius: RADIUS,
        }
    }

    fn update(&mut self, color_delta: u8) {
        self.position.update();
        self.color.update(color_delta);
    }
}

fn draw_circle(context: &web_sys::CanvasRenderingContext2d, circle : &Circle) {
    context.begin_path();
    context.set_fill_style(&circle.color.to_js_value());
    context.set_stroke_style(&circle.color.to_js_value());

    context
        .arc(
            circle.position.x,
            circle.position.y,
            circle.radius,
            0.0,
            f64::consts::PI * 2.0,
        )
        .unwrap();

    context.fill();
    context.stroke();
}

fn make_art() {
    let canvas = canvas();
    let context = context();

    context.clear_rect(
            MIN_POS,
            MIN_POS,
            canvas.width() as f64,
            canvas.height() as f64,
        );

    let mut circle = Circle::new();
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    let count = 100000;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > count {

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;

        let MIN_COLOR_DELTA = 10.0;
        let MAX_COLOR_DELTA = 50.0;

        let color_delta = color_slider_value() as u8;

        draw_circle(&context, &circle);
        circle.update(color_delta);

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

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn canvas() -> web_sys::HtmlCanvasElement{
    document().get_element_by_id("canvas").unwrap()
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
    .value_as_number()
    as u8
    
}

#[wasm_bindgen(start)]
pub fn start() {
    web_sys::console::log(&js_sys::Array::from(&JsValue::from_str(
        "I love printf debugging!",
    )));
    let document = document();

    let button = document
        .get_element_by_id("button")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();

    // TODO: disabling on click weird stuff with closure for now
    // let onclick_handler = Closure::wrap(Box::new(move || {
        // make_art(&canvas, &context);
    // }) as Box<dyn FnMut()>);
    // button.set_onclick(Some(onclick_handler.as_ref().unchecked_ref()));
    // onclick_handler.forget();

    make_art();
}

use js_sys::Math::random;
use std::f64;
use std::fmt::{self, Display};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::EventListener;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
}

const POS_DELTA: f64 = 2.1;
const MIN_POS: f64 = 0.0;
const MAX_X_POS: f64 = 500.0;
const MAX_Y_POS: f64 = 250.0;
const RADIUS: f64 = 2.0;
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

const COLOR_DELTA: u8 = 10;

impl ColorBit {
    fn rand() -> Self {
        ColorBit {
            bit: f64::floor(random() * 255 as f64) as u8,
        }
    }

    fn update(&mut self) -> () {
        if random() > 0.5 {
            self.bit = self.bit.saturating_add(COLOR_DELTA);
        } else {
            self.bit = self.bit.saturating_sub(COLOR_DELTA);
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

    fn update(&mut self) {
        self.r.update();
        self.g.update();
        self.b.update();
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

    fn update(&mut self) {
        self.position.update();
        self.color.update();
    }
}

fn make_art(canvas: &web_sys::HtmlCanvasElement, context: &web_sys::CanvasRenderingContext2d) {
    context.clear_rect(
        MIN_POS,
        MIN_POS,
        canvas.width() as f64,
        canvas.height() as f64,
    );

    let count = 7000;

    let mut circle = Circle::new();

    for _ in 0..count {
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

        circle.update();
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    web_sys::console::log(&js_sys::Array::from(&JsValue::from_str(
        "I love printf debugging!",
    )));
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

    let button = document
        .get_element_by_id("button")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();

    let onclick_handler = Closure::wrap(Box::new(move || {
        make_art(&canvas, &context);
    }) as Box<dyn FnMut()>);
    button.set_onclick(Some(onclick_handler.as_ref().unchecked_ref()));
    onclick_handler.forget();
}

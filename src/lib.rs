use js_sys::Math::random;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
struct Position {
    x: f64,
    y: f64,
}

const POS_DELTA: f64 = 5.0;
const MIN_POS: f64 = 0.0;
const MAX_POS: f64 = 100.0;

impl Position {
    fn new() -> Self {
        Position { x: 10.0, y: 10.0 }
    }

    fn validate(&self) -> bool {
        self.x > MIN_POS && self.x < MAX_POS && self.y > MIN_POS && self.y < MAX_POS
    }

    fn update(&mut self) {
        let mut options: Vec<Self> = Vec::new();

        for x_multiplier in -1..2 {
            for y_multiplier in -1..2 {
                let new_position = Position {
                    x: self.x + POS_DELTA * f64::from(x_multiplier),
                    y: self.y + POS_DELTA * f64::from(y_multiplier),
                };

                if x_multiplier == 0 && y_multiplier == 0 || !new_position.validate() {
                    continue;
                } else {
                    options.push(new_position)
                };
            }
        }

        let mut random_index = 0;
        unsafe {
            random_index = f64::floor(random() * options.len() as f64) as usize;
        }

        let random_element = &options[random_index];

        self.x = random_element.x;
        self.y = random_element.y;
    }
}
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new() -> Self {
        Color { r: 0, g: 0, b: 255 }
    }

    fn to_js_value(&self) -> JsValue {
        JsValue::from_str(&format!("rgb({}, {}, {})", self.r, self.g, self.b))
    }
}
struct Circle {
    position: Position,
    color: Color,
    radius: f64,
}

impl Circle {
    fn new() -> Self {
        Circle {
            position: Position::new(),
            color: Color::new(),
            radius: 5.0,
        }
    }

    fn update(&mut self) {
        self.position.update();
    }
}

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

    let mut circle = Circle::new();

    for _ in 0..count {
        context.begin_path();

        context.set_fill_style(&circle.color.to_js_value());
        context.set_stroke_style(&circle.color.to_js_value());

        // Draw the outer circle.
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

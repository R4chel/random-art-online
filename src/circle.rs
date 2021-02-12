use js_sys::Math::random;
use std::cell::RefCell;
use std::f64;
use std::fmt::{self, Display};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
}

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

    fn update(&mut self, position_delta: f64) {
        let mut options: Vec<Self> = Vec::new();

        for x_multiplier in -1..=1 {
            for y_multiplier in -1..=1 {
                let new_position = Position {
                    x: self.x + position_delta * (x_multiplier as f64),
                    y: self.y + position_delta * (y_multiplier as f64),
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
        let update_with_delta = move |x| ColorBit::update(x, color_delta);

        update_with_delta(&mut self.r);
        update_with_delta(&mut self.g);
        update_with_delta(&mut self.b);
    }
}

#[derive(Debug)]
pub struct Circle {
    position: Position,
    color: Color,
    radius: f64,
}

impl Circle {
    pub fn new() -> Self {
        Circle {
            position: Position::rand(),
            color: Color::rand(),
            radius: RADIUS,
        }
    }

    pub fn update(&mut self, position_delta: f64, color_delta: u8) {
        self.position.update(position_delta);
        self.color.update(color_delta);
    }

    pub fn color(&self) -> JsValue {
        self.color.to_js_value()
    }

    pub fn x_position(&self) -> f64 {
        self.position.x
    }
    pub fn y_position(&self) -> f64 {
        self.position.y
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

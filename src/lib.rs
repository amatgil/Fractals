use std::f64::consts::TAU;

use ppmitzador::Coord;

use wasm_bindgen::prelude::*;

const WIDTH: usize = 1024;
const HEIGHT: usize = WIDTH;
const COLOR: &str = "#f0c6c6";

mod fancy_tree;
mod basic_tree;
mod sierp;
mod snowflake;

// Wasm "constants"
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    let x = ((a.x as f64) * (1.0 - t) + (b.x as f64) * t).round() as usize;
    let y = ((a.y as f64) * (1.0 - t) + (b.y as f64) * t).round() as usize;

    Coord { x, y }
}


// Taken inspiration from SO
fn rotate_point_around_pivot(mut p: Coord, pivot: Coord, theta: f64) -> Coord {
    let s = theta.sin();
    let c = theta.cos();

    Coord {
	x: (c * (p.x as isize - pivot.x as isize) as f64 - s * (p.y as isize - pivot.y as isize) as f64 + pivot.x as f64).round() as usize,
	y: (s * (p.x as isize - pivot.x as isize) as f64 + c * (p.y as isize - pivot.y as isize) as f64 + pivot.y as f64).round() as usize,
    }
}

fn get_angle(l: &Line) -> f64 {
    let x = (l.b.x - l.a.x) as f64;
    let y = (l.b.y - l.a.y) as f64;
    return (x/y).atan();
}

#[derive(Debug, Clone, Copy)]
struct Line {
    a: Coord,
    b: Coord,
}

#[derive(Debug, Clone, Copy)]
struct Triangle {
    a: Coord,
    b: Coord,
    c: Coord,
}

#[derive(Debug, Clone, Copy)]
struct Square {
    a: Coord,
    b: Coord,
    c: Coord,
    d: Coord,
}

type Shapes = Vec<Box<dyn Shape>>;

trait Shape {
    fn points(&self) -> Vec<Coord>;
}

impl Shape for Triangle {
    fn points(&self) -> Vec<Coord> {
        vec![self.a, self.b, self.c]
    }
}
impl Shape for Square {
    fn points(&self) -> Vec<Coord> {
        vec![self.a, self.b, self.c, self.d]
    }
}
use std::{f64::consts::TAU, ops::{Add, Mul, Sub}};

use wasm_bindgen::prelude::*;

const WIDTH: usize = 1024;
const HEIGHT: usize = WIDTH;
const COLOR: &str = "#f0c6c6";

mod fancy_tree;
mod basic_tree;
mod sierp;
mod snowflake;
mod system_l;

// Wasm "constants"
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize
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

impl Coord {
    pub fn new(x: usize, y:usize) -> Self { Self { x, y } }
    pub fn abs(&self) -> f64 { ((self.x*self.x + self.y*self.y) as f64).sqrt() }
    pub fn distance(&self, rhs: Self) -> f64 {
        let dx = (self.x as isize - rhs.x as isize).abs() as usize;
        let dy = (self.y as isize - rhs.y as isize).abs() as usize;
        Coord::new(dx, dy).abs()
    }
    fn lerp_c(&self, b: &Coord, t: f64) -> Coord {
	let a = self; // Too lazy to rewrite, just reassign
	let x = ((a.x as f64) * (1.0 - t) + (b.x as f64) * t).round() as usize;
	let y = ((a.y as f64) * (1.0 - t) + (b.y as f64) * t).round() as usize;

	Coord { x, y }
    }
    fn rotate_around_pivot(&self, pivot: Coord, theta: f64) -> Coord {
	let p = self;
	let s = theta.sin();
	let c = theta.cos();

	Coord {
	    x: (c * (p.x as isize - pivot.x as isize) as f64 - s * (p.y as isize - pivot.y as isize) as f64 + pivot.x as f64).round() as usize,
	    y: (s * (p.x as isize - pivot.x as isize) as f64 + c * (p.y as isize - pivot.y as isize) as f64 + pivot.y as f64).round() as usize,
	}
    }

}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self { x: self.x + rhs.x, y: self.y + rhs.y, } }
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self { x: self.x - rhs.x, y: self.y - rhs.y, } }
}

impl Mul<usize> for Coord {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output { Self { x: self.x * rhs, y: self.y * rhs, } }
}




fn get_angle(l: &Line) -> f64 {
    let x = (l.b.x - l.a.x) as f64;
    let y = (l.b.y - l.a.y) as f64;
    return (x/y).atan();
}

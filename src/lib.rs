use std::f64::consts::TAU;

use ppmitzador::Coord;
use std::io::BufWriter;
use std::io::Write;

use wasm_bindgen::prelude::*;

const THETA: f64 = TAU/6.0;

const WIDTH: usize  = 1000;
const HEIGHT: usize = WIDTH;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    let x = ((a.x as f64)*(1.0 - t) + (b.x as f64)*t).round() as usize;
    let y = ((a.y as f64)*(1.0 - t) + (b.y as f64)*t).round() as usize;

    Coord { x, y }
}

fn get_flake_points(n: usize, anti: bool) -> Vec<Coord> {
    if n > 9 { panic!("n values above 8 make svg viewers struggle, so I'm disallowing them") }
    println!("[INFO]: Initializing...");
    let lowest_y  = (0.5 - (3.0 as f64).sqrt()/12.0)*(HEIGHT as f64);
    let highest_y = lowest_y + (3.0 as f64).sqrt()*(HEIGHT as f64)/4.0;

    let mut points = vec![
        Coord { x: 1*WIDTH/4, y: lowest_y  as usize},
        Coord { x: 2*WIDTH/4, y: highest_y as usize},
        Coord { x: 3*WIDTH/4, y: lowest_y  as usize},
        Coord { x: 1*WIDTH/4, y: lowest_y  as usize}, // repetida pel cicle
    ];

    println!("[INFO]: Computating for n = {n}...");
    for _ in 0..n {
        let mut new_points = vec![];
        
        for pair in points.windows(2) {
            //        d
            //       / \            <- Koch pattern
            // a -- c   e -- b
            //
            // Here, theta is the angle fomed by the segments c-d and d-e 

            let a = pair[0];
            let b = pair[1];

            let c = lerp_c(a, b,       0.5/(1.0 + (THETA/2.0).sin()));
            let e = lerp_c(a, b, 1.0 - 0.5/(1.0 + (THETA/2.0).sin()));

            // place d |c-a|*cos(theta/2) units above the midpoint of a, b 
            let d = { 

                // find the midpoint of a and b
                let mid = lerp_c(a, b, 0.5);
                
                // find vector from a to c
                let ca_vec_x = (c.x as f64 - a.x as f64)*if anti {-1.0} else {1.0};
                let ca_vec_y = (c.y as f64 - a.y as f64)*if anti {-1.0} else {1.0};
                
                // place d |c-a|*cos(theta/2) units perpendicular to the c-a vector
                // this comes from the c-d-mid right angle triangle where |d-c| = |c-a|
                let dx = mid.x as f64 - (THETA/2.0).cos()*(ca_vec_y as f64);
                let dy = mid.y as f64 + (THETA/2.0).cos()*(ca_vec_x as f64);
                
                Coord { x: dx as usize, y: dy as usize}
            };
            new_points.extend([a, c, d, e, b]);
        }
        points = new_points
    }

    println!("[INFO]: Returning points");

    points
}

#[wasm_bindgen]
pub fn vonkoch(n: usize, anti: bool) -> String {
    let mut buffer = String::new();

    let points = get_flake_points(n, anti);

    buffer.push_str(&format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"vonkoch-holder\">\n"));
    buffer.push_str(&format!("<polygon points=\""));
    for Coord { x, y } in points {
	buffer.push_str(&format!("{x},{y} "));
    }
    buffer.push_str(&format!("\" fill=\"none\" stroke=\"#f0c6c6\"/>\n"));
    buffer.push_str(&format!("</svg>\n"));

    println!("[INFO]: Finished, file should have been written");

    return buffer;
}

#[wasm_bindgen]
pub fn sierp(n: usize) -> String {
    let lowest_y  = (0.5 - (3.0 as f64).sqrt()/12.0)*(HEIGHT as f64);
    let highest_y = lowest_y + (3.0 as f64).sqrt()*(HEIGHT as f64)/4.0;

    let mut filled_triangles: Vec<[Coord; 3]> =
	vec![
	    [
		Coord { x: 1*WIDTH/4, y: lowest_y  as usize},
		Coord { x: 2*WIDTH/4, y: highest_y as usize},
		Coord { x: 3*WIDTH/4, y: lowest_y  as usize},
	    ]
	];

    for _ in 0..n {
	let mut new_filled_triangles: Vec<[Coord; 3]> = Vec::new();
	let mut new_points: Vec<Coord> = Vec::new();

        for c in filled_triangles.concat() {
	    let closest_left: Coord = todo!();
	    let closest_right: Coord = todo!();

	    let new_left = {
		let x = (c.x + closest_left.x) / 2;
		let y = (c.y + closest_left.y) / 2;
		Coord { x, y }
	    };

	    let new_right = {
		let x = (c.x + closest_right.x) / 2;
		let y = (c.y + closest_right.y) / 2;
		Coord { x, y }
	    };
	    let new_bottom = {
		let x = (closest_left.x + closest_right.x) / 2;
		let y = (closest_left.y + closest_right.y) / 2;
		Coord { x, y }
	    };
	    new_filled_triangles.push([c, new_left, new_right]);
	    new_filled_triangles.push([closest_left, new_left, new_bottom]);
	    new_filled_triangles.push([closest_right, new_right, new_bottom]);
	}
    }
    todo!()
}

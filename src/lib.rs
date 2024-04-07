use std::f64::consts::TAU;

use ppmitzador::Coord;

use wasm_bindgen::prelude::*;


const WIDTH: usize  = 1024;
const HEIGHT: usize = WIDTH;
const COLOR: &str = "#f0c6c6";

// Von Koch constants
const THETA: f64 = TAU/6.0;

// Wasm "constants"
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// Misc
fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    let x = ((a.x as f64)*(1.0 - t) + (b.x as f64)*t).round() as usize;
    let y = ((a.y as f64)*(1.0 - t) + (b.y as f64)*t).round() as usize;

    Coord { x, y }
}


// Von Koch Functions
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
    buffer.push_str(&format!("\" fill=\"none\" stroke=\"{COLOR}\"/>\n"));
    buffer.push_str(&format!("</svg>\n"));

    return buffer;
}

// Sierp functions
#[wasm_bindgen]
pub fn sierp(n: usize) -> String {
    let lowest_y  = (0.5 - (3.0 as f64).sqrt()/12.0)*(HEIGHT as f64);
    let highest_y = lowest_y + (3.0 as f64).sqrt()*(HEIGHT as f64)/4.0;
    let lowest_y = lowest_y as usize;
    let highest_y = highest_y as usize;



    let base_side_length = Coord { x: WIDTH/4 , y: highest_y - lowest_y };

    let mut points = Vec::new();
    sierp_go(n, &mut points, Coord { x: WIDTH/4, y: lowest_y as usize}, base_side_length);
    points = points.into_iter().map(|triplet| triplet.map(|p| Coord { x: p.x, y: HEIGHT - p.y - 1 })).collect(); // Flip vertically

    let mut buffer = String::new();

    buffer.push_str(&format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"sierp-holder\">\n"));
    for triplet in points {
	buffer.push_str(&format!("<polygon points=\""));
	for Coord { x, y } in triplet {buffer.push_str(&format!("{x},{y} "));}
	buffer.push_str(&format!("\" fill=\"{COLOR}\" stroke=\"#000000\" stroke-width=\"0.1%\"/>\n"));
    }
    buffer.push_str(&format!("</svg>\n"));

    return buffer;
}

pub fn sierp_go(n: usize, points: &mut Vec<[Coord; 3]>, start_pos: Coord, side_length: Coord) {
    // Triambgle:
    //     c
    //    / \
    //   /   \
    //  /     \
    // a ----- b     a = start_pos

    let c = start_pos + side_length;
    let b = start_pos + Coord { x: side_length.x*2, y: 0 };

    if n <= 0 {
	points.push([start_pos, c, b]);
    } else {
	let half_side_length = Coord { x: side_length.x / 2, y: side_length.y / 2 };

	let c = start_pos + half_side_length;
	let b = start_pos + Coord { x: half_side_length.x*2, y: 0 };
	sierp_go(n - 1, points, start_pos, half_side_length);
	sierp_go(n - 1, points, c, half_side_length);
	sierp_go(n - 1, points, b, half_side_length);
    }
}

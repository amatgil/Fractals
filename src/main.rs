use std::f64::consts::TAU;

use ppmitzador::Coord;
use std::io::BufWriter;
use std::io::Write;

const THETA: f64       = TAU/6.0;
const GRID_SIDE: usize = 3;

const FLAKE_WIDTH: usize     = 1000;
const FLAKE_HEIGHT: usize    = FLAKE_WIDTH;

const WIDTH: usize     = FLAKE_WIDTH*GRID_SIDE*2;
const HEIGHT: usize    = FLAKE_HEIGHT*GRID_SIDE;

fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    let x = ((a.x as f64)*(1.0 - t) + (b.x as f64)*t).round() as usize;
    let y = ((a.y as f64)*(1.0 - t) + (b.y as f64)*t).round() as usize;

    Coord { x, y }
}

fn get_flake_points(n: usize, anti: bool) -> Vec<Coord> {
    println!("[INFO]: Initializing...");
    let lowest_y  = (0.5 - (3.0 as f64).sqrt()/12.0)*(FLAKE_HEIGHT as f64);
    let highest_y = lowest_y + (3.0 as f64).sqrt()*(FLAKE_HEIGHT as f64)/4.0;

    let mut points = vec![
        Coord { x: 1*FLAKE_WIDTH/4, y: lowest_y  as usize},
        Coord { x: 2*FLAKE_WIDTH/4, y: highest_y as usize},
        Coord { x: 3*FLAKE_WIDTH/4, y: lowest_y  as usize},
        Coord { x: 1*FLAKE_WIDTH/4, y: lowest_y  as usize}, // repetida pel cicle
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

fn main() {
    let n = 6;
    let anti = false;
    let file = std::fs::File::create("svg-test.svg").expect("Could not create file");

    let points = get_flake_points(n, anti);

    let mut buffer = BufWriter::new(file); // This will likely get changed to stdio
    buffer.write(format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" >\n").as_bytes()).unwrap();
    buffer.write(format!("<polygon points=\"").as_bytes()).unwrap();
    for Coord { x, y }  in points { buffer.write(format!("{x},{y} ").as_bytes()); }
    buffer.write(format!("\" fill=\"none\" stroke=\"black\"/>\n").as_bytes()).unwrap();
    buffer.write(format!("</svg>\n").as_bytes()).unwrap();

    println!("[INFO]: Finished, file should have been written");
}

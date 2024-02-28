use std::f64::consts::TAU;

use ppmitzador::*;

const WIDTH: usize     = 2000;
const HEIGHT: usize    = WIDTH;
const THETA: f64       = TAU/6.0;

fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    let x = ((a.x as f64)*(1.0 - t) + (b.x as f64)*t).round() as usize;
    let y = ((a.y as f64)*(1.0 - t) + (b.y as f64)*t).round() as usize;

    Coord { x, y }
}

fn save_flake(n: usize, anti: bool) {
    println!("[INFO]: Initializing...");
    let mut data = ImagePBM::new(WIDTH, HEIGHT, false);
    let lowest_y  = (0.5 - (3.0 as f64).sqrt()/12.0)*(HEIGHT as f64);
    let highest_y = lowest_y + (3.0 as f64).sqrt()*(HEIGHT as f64)/4.0;
    let mut points = vec![
        Coord { x: 1*WIDTH/4, y: lowest_y  as usize},
        Coord { x: 2*WIDTH/4, y: highest_y as usize},
        Coord { x: 3*WIDTH/4, y: lowest_y  as usize},
        Coord { x: 1*WIDTH/4, y: lowest_y  as usize}, // repetida pel cicle
    ];

    println!("[INFO]: Starting computations...");
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
    println!("[INFO]: Drawing lines...");

    for pair in points.windows(2) { data.draw_line_with_thickness(pair[0], pair[1], true, 2); }


    println!("[INFO]: Saving to file...");
    let filename = format!("koch-n{:0>2}-anti{anti}.ppm", n);
    data.save_to_file(&filename).unwrap();

    println!("When n = {n}, there were {} points", points.len());
    println!("[INFO]: Saved '{}'", filename);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let n: usize = args.get(1).expect("You forgor n")
        .parse().expect("n must be positive integer");

    let anti: bool = {
        let s = args.get(2).expect("You forgor to ask for anti or not (0 for normal, 1 for anti)");
        if &*s == "0" { false }
        else if &*s == "1" { true }
        else { panic!("anti must be either 0 (normal) or 1 (anti) ")}
    };

    save_flake(n, anti);
}

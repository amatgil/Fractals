use std::f64::consts::TAU;

use ppmitzador::*;

const THETA: f64       = TAU/6.0;
const GRID_SIDE: usize = 3;

const FLAKE_WIDTH: usize     = 4000;
const FLAKE_HEIGHT: usize    = FLAKE_WIDTH;

const WIDTH: usize     = FLAKE_WIDTH*GRID_SIDE*2;
const HEIGHT: usize    = FLAKE_HEIGHT*GRID_SIDE;

fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    let x = ((a.x as f64)*(1.0 - t) + (b.x as f64)*t).round() as usize;
    let y = ((a.y as f64)*(1.0 - t) + (b.y as f64)*t).round() as usize;

    Coord { x, y }
}

fn draw_flake(img: &mut ImagePBM, n: usize, anti: bool, origin: Coord) {
    println!("[INFO]: Initializing...");
    let lowest_y  = (0.5 - (3.0 as f64).sqrt()/12.0)*(FLAKE_HEIGHT as f64);
    let highest_y = lowest_y + (3.0 as f64).sqrt()*(FLAKE_HEIGHT as f64)/4.0;

    let mut points = vec![
        Coord { x: 1*FLAKE_WIDTH/4 + origin.x, y: lowest_y  as usize + origin.y},
        Coord { x: 2*FLAKE_WIDTH/4 + origin.x, y: highest_y as usize + origin.y},
        Coord { x: 3*FLAKE_WIDTH/4 + origin.x, y: lowest_y  as usize + origin.y},
        Coord { x: 1*FLAKE_WIDTH/4 + origin.x, y: lowest_y  as usize + origin.y}, // repetida pel cicle
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
    println!("[INFO]: Drawing lines for n = {n}...");

    for pair in points.windows(2) {
        let thickness = 
            if n < 8 { 4 }
            else { 1 };
        img.draw_line_with_thickness(pair[0], pair[1], true, thickness); 
    }

}

fn main() {
    let mut data = ImagePBM::new(WIDTH, HEIGHT, false);
    println!("w: {}, h: {}", WIDTH, HEIGHT);
    
    for y in 0..GRID_SIDE {
        for x in 0..GRID_SIDE {
            for anti in [true, false] {
                let mut origin = Coord { x: x * WIDTH / GRID_SIDE , y: y * HEIGHT / GRID_SIDE};
                origin.x += (anti as usize)*FLAKE_WIDTH;
                let n = 3*(GRID_SIDE - y - 1) + x;
                draw_flake(&mut data, n, anti, origin);
            }
        }
    }

    println!("[INFO]: Mathematics finished, adding finishing touches");
    let mut y = 0;
    while y < HEIGHT {
        data.draw_line_with_thickness(
            Coord::new(0, y),
            Coord::new(WIDTH - 1, y),
            true, 3);

        y += FLAKE_HEIGHT;
    }

    let mut x = 0;
    while x < WIDTH {
        data.draw_line_with_thickness(
            Coord::new(x, 0),
            Coord::new(x, HEIGHT - 1),
            true, 3);

        x += FLAKE_WIDTH * 2;
    }

    println!("[INFO]: Drawing finished, saving file...");
    data.save_to_file("atlas.pbm").unwrap();
    println!("[INFO]: File saved, enjoy! :D");
}

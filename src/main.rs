use std::f64::consts::TAU;

use ppmitzador::*;

const BG: Pixel     = Pixel::BLACK;
const WIDTH: usize  = 10000;
const HEIGHT: usize = WIDTH;
const ANTI: bool    = false;

fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    // x
    let x = ((a.x as f64)*(1.0 - t) + (b.x as f64)*t).round() as usize;
    let y = ((a.y as f64)*(1.0 - t) + (b.y as f64)*t).round() as usize;

    Coord { x, y }
}

fn main() {
    let n = 7;
    let mut data = ImagePPM::new(WIDTH, HEIGHT, BG);

    let mut points = vec![
        Coord { x: 1*WIDTH/4, y: 1*HEIGHT / 4 },
        Coord { x: 2*WIDTH/4, y: 3*HEIGHT / 4 },
        Coord { x: 3*WIDTH/4, y: 1*HEIGHT / 4 },
        Coord { x: 1*WIDTH/4, y: 1*HEIGHT / 4 }, // repetida pel cicle
    ];

    for _ in 0..n {
        let mut new_points = vec![];
        
        for pair in points.windows(2) {
            //        d
            //       / \            <- Koch pattern
            // a -- c   e -- b
            let a = pair[0];
            let b = pair[1];

            let c = lerp_c(a, b, 1.0/3.0);
            let e = lerp_c(a, b, 2.0/3.0);

            // rotate e around c by 60º
            let d = { 
                let x1 = c.x as f64;
                let x2 = e.x as f64;
                let y1 = c.y as f64;
                let y2 = e.y as f64;

                let dx = x2 - x1;
                let dy = y2 - y1;

                let theta = TAU/6.0 * if ANTI { 1.0 } else { -1.0 }; // 60ª

                let rx = x1 +    theta.cos()*dx + theta.sin()*dy;
                let ry = y1 + (-theta).sin()*dx + theta.cos()*dy;
                
                Coord { x: rx as usize, y: ry as usize}
            };

            new_points.push(a);
            new_points.push(c);
            new_points.push(d);
            new_points.push(e);
            new_points.push(b);

    
        }
        points = new_points
    }



    for p in points {
        *data.get_mut(p.x, p.y).unwrap() = Pixel::WHITE;
    }

    data.save_to_file("testing.ppm").unwrap();

    println!("Finished");
}

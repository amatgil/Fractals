use std::f64::consts::TAU;

use ppmitzador::*;

const BG: Pixel = Pixel::BLACK;

//fn draw_koch_line(img: &mut ImagePPM, n: usize, start: Coord, end: Coord) {
//    if n == 1 { img.draw_line(start, end) }
//    else {
//        let node1 = lerp_c(start, end, 0.3);
//        let node2 = lerp_c(start, end, 0.6);
//
//        img.draw_line(img, start, node1);
//
//        img.draw_line(img, node2, end);
//    }
//}
//
//fn lerp(a: usize, b: usize, t: f64) -> usize {
//    a*(1 - t) + b*t
//}
//

fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    // x
    let x = ((a.x as f64)*(1.0 - t) + (b.x as f64)*t).round() as usize;
    let y = ((a.y as f64)*(1.0 - t) + (b.y as f64)*t).round() as usize;

    Coord { x, y }
}

fn main() {
    let n = 2;
    let width = 100;
    let height = width;
    let mut data = ImagePPM::new(width, height, BG);

    let mut points = vec![
        Coord { x: 1*width/4, y: 1*height / 4 },
        Coord { x: 2*width/4, y: 3*height / 4 },
        Coord { x: 3*width/4, y: 1*height / 4 },
        Coord { x: 1*width/4, y: 1*height / 4 }, // repetida pel cicle
    ];

    for _ in 0..n {
        let mut new_points = vec![];
        
        for pair in points.windows(2) {
            //        d
            //       / \
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

                let theta = -TAU/6.0; // 60ª

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
        dbg!(points);
        points = new_points
    }



    for p in points {
        *data.get_mut(p.x, p.y).unwrap() = Pixel::WHITE;
    }

    data.save_to_file("testing.ppm").unwrap();

    println!("Finished");
}

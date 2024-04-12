use crate::*;


const WIDTH: usize = 1500;
const HEIGHT: usize = WIDTH;

#[wasm_bindgen]
pub fn pythagorean_tree(n: i32, theta: f64, side_length: usize) -> String {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    let base_line = {
        let base_line_h = HEIGHT * 30 / 100;
        let a = Coord { x: WIDTH / 2 - side_length / 2, y: base_line_h };
        let b = Coord { x: WIDTH / 2 + side_length / 2, y: base_line_h };
        Line { a, b }
    };

    if n >= 0 {
        pythagorean_tree_go(n as usize, &mut shapes, theta, base_line);
    }

    let mut buffer = String::new();

    buffer.push_str(&format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"pyth-tree-holder\">\n"));
    for shape in shapes {
        buffer.push_str(&format!("<polygon points=\""));
        for Coord { x, y } in shape.points() {
            let y = HEIGHT - y - 1;
            buffer.push_str(&format!("{x},{y} "));
        }
        buffer.push_str(&format!("\" fill=\"none\" stroke=\"{COLOR}\"/>\n"));
    }
    buffer.push_str(&format!("</svg>\n"));

    return buffer;
}

fn pythagorean_tree_go(n: usize, shapes: &mut Shapes, theta: f64, base_line: Line) {
    if n == 0 { return; }

    let q: Square = {
        // d ---- c
        // |      |
        // |      |
        // a ---- b
        let a = base_line.a;
        let b = base_line.b;
        let d = rotate_point_around_pivot(b, a, TAU/4.0);
        let c = rotate_point_around_pivot(a, b, -TAU/4.0);
        
        Square { a, b, c, d}
    };

    let tri: Triangle = {
        //     c
        //   /    \
        //  /      \
        // a ------- b
        let l = base_line.b.distance(base_line.a).round() as usize;
        let a = q.d;
        let b = q.c;
        let c = {
            //let dx = (l as f64 * theta.cos().powi(2)).round() as usize;
            //let dy = (l as f64 * theta.sin().powi(2)).round() as usize;
            //Coord {
            //    x: base_line.a.x + dx,
            //    y: base_line.a.y - dy,
            //}
            rotate_point_around_pivot(a, b, -theta)
            //rotate_point_around_pivot(b, a, theta)
        };

        Triangle { a, b, c }
    };
    let l1 = Line { a: tri.a, b: tri.c };
    let l2 = Line { a: tri.c, b: tri.b };

    shapes.push(Box::new(q));
    shapes.push(Box::new(tri));

    pythagorean_tree_go(n - 1, shapes, theta, l1);
    pythagorean_tree_go(n - 1, shapes, theta, l2);
}
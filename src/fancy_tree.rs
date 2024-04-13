use crate::*;


const WIDTH: usize = 1500;
const HEIGHT: usize = WIDTH;

const TRI_COLOR: &str = "none";
const QUA_COLOR: &str = COLOR;


// Places in bounds
fn normalize(c: Coord) -> Coord {
    fn n(v: usize, w: usize) -> usize {
        if v > 100000 { return 0; }
        if v > w { return w - 1; }
        return v;
    }

    Coord {
        x: n(c.x, WIDTH),
        y: n(c.y, HEIGHT),
    }
}

fn mirror_y_properly(y: usize) -> usize {
    let maybe = HEIGHT - y - 1;
    if maybe > HEIGHT { 0 }
    else { maybe } 
}

#[wasm_bindgen]
pub fn pythagorean_tree(n: i32, theta: f64, side_length: usize) -> String {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    let base_line = {
        let base_line_h = HEIGHT * 15 / 100;
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
        for Coord { x, y } in shape.points().into_iter().map(normalize) { buffer.push_str(&format!("{x},{} ", mirror_y_properly(y))); }
        let col = if shape.points().len() == 3 { TRI_COLOR } else { QUA_COLOR };
        buffer.push_str(&format!("\" fill=\"{col}\" stroke-width=\"5px\" stroke=\"{col}\"/>\n"));
    }

    buffer.push_str(&format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke-width=\"10px\" stroke=\"#000000\" />",
     base_line.a.x,
     mirror_y_properly(base_line.a.y),
     base_line.b.x,
     mirror_y_properly(base_line.b.y)));
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
        let a = q.d;
        let b = q.c;
        let c = {
            let extended = lerp_c(a, b, theta.cos());
            rotate_point_around_pivot(extended, a, -theta)
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

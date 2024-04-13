use crate::*;

#[wasm_bindgen]
pub fn tree(n: i32, theta: f64, branch_length: usize, branch_multiplier: f64) -> String {
    let mut lines = Vec::new();
    let first = Coord {
        x: WIDTH / 2,
        y: HEIGHT * 80 / 100,
    };
    let second = Coord {
        x: first.x,
        y: first.y - branch_length,
    };
    lines.push(Line {
        a: first,
        b: second,
    });
    let first_line = Line { a: first, b: second };
    lines.push(first_line);

    if n > 0 { tree_go(n as usize, &mut lines, theta, branch_multiplier, first_line); }

    let mut buffer = String::new();

    buffer.push_str(&format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"tree-holder\">\n"));
    for line in lines {
        buffer.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{COLOR}\" stroke-width=\"5\"/>",
            line.a.x, line.a.y, line.b.x, line.b.y
        ));
    }
    buffer.push_str(&format!("</svg>\n"));

    return buffer;
}

fn tree_go(n: usize, v: &mut Vec<Line>, theta: f64, branch_multiplier: f64, r: Line) {
    if n <= 0 { return; }

    let e = Coord { // r.b + (r.b - r.a)*bmult
	    x: (r.b.x as f64 + (r.b.x as f64 * branch_multiplier) - (r.a.x as f64 * branch_multiplier)).round() as usize,
	    y: (r.b.y as f64 + (r.b.y as f64 * branch_multiplier) - (r.a.y as f64 * branch_multiplier)).round() as usize,
    };

    let p1 = rotate_point_around_pivot(e, r.b, -theta);
    let p2 = rotate_point_around_pivot(e, r.b, theta);

    let l1 = Line { a: r.b.clone(), b: p1 };
    let l2 = Line { a: r.b, b: p2 };
    v.push(l1.clone());
    v.push(l2.clone());

    tree_go(n - 1, v, theta, branch_multiplier, l1);
    tree_go(n - 1, v, theta, branch_multiplier, l2);
}

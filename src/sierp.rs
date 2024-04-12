use crate::*;

#[wasm_bindgen]
pub fn sierp(n: usize) -> String {
    let lowest_y = (0.5 - (3.0 as f64).sqrt() / 12.0) * (HEIGHT as f64);
    let highest_y = lowest_y + (3.0 as f64).sqrt() * (HEIGHT as f64) / 4.0;
    let lowest_y = lowest_y as usize;
    let highest_y = highest_y as usize;

    let base_side_length = Coord {
        x: WIDTH / 4,
        y: highest_y - lowest_y,
    };

    let mut points = Vec::new();
    sierp_go(
        n,
        &mut points,
        Coord {
            x: WIDTH / 4,
            y: lowest_y as usize,
        },
        base_side_length,
    );
    points = points
        .into_iter()
        .map(|triplet| {
            triplet.map(|p| Coord {
                x: p.x,
                y: HEIGHT - p.y - 1,
            })
        })
        .collect(); // Flip vertically

    let mut buffer = String::new();

    buffer.push_str(&format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"sierp-holder\">\n"));
    for triplet in points {
        buffer.push_str(&format!("<polygon points=\""));
        for Coord { x, y } in triplet {
            buffer.push_str(&format!("{x},{y} "));
        }
        buffer.push_str(&format!(
            "\" fill=\"{COLOR}\" stroke=\"#000000\" stroke-width=\"0.1%\"/>\n"
        ));
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
    let b = start_pos
        + Coord {
            x: side_length.x * 2,
            y: 0,
        };

    if n <= 0 {
        points.push([start_pos, c, b]);
	return;
    } 

    let half_side_length = Coord {
	x: side_length.x / 2,
	y: side_length.y / 2,
    };

    let c = start_pos + half_side_length;
    let b = start_pos
        + Coord {
            x: half_side_length.x * 2,
            y: 0,
        };
    sierp_go(n - 1, points, start_pos, half_side_length);
    sierp_go(n - 1, points, c, half_side_length);
    sierp_go(n - 1, points, b, half_side_length);
}

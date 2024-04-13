use std::collections::HashMap;

use crate::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[wasm_bindgen]
pub enum LSystemTurtle {
    DoNothing,
    DrawForward,
    Rotate45CW,
    Rotate45CCW,
    Rotate90CW,
    Rotate90CCW
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TurtleMapping {
    val: HashMap<char, LSystemTurtle>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RulesMap {
    val: HashMap<String, String>
}

  
#[wasm_bindgen]
pub fn generate_l_fractal(n: i32, axiom: String, rules: JsValue, turtle_mapping: JsValue, start_x: i32, start_y: i32, start_direction: f64, line_length: i32) -> Result<String, String> {
    let error_svg = |e: &str| {
        format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"l-system-holder\">\n <text x=\"20\" y=\"20\">{e}'</text></svg>\n")
    };

    let RulesMap { val: rules } = serde_wasm_bindgen::from_value(rules)
	.map_err(|in_e| error_svg(&format!("The Rules that advance the axiom are invalid: '{}'", in_e.to_string())))?;
    let TurtleMapping { val: turtle_map } = serde_wasm_bindgen::from_value(turtle_mapping)
	.map_err(|in_e| error_svg(&format!("The Turle Mapping is invalid: '{}'", in_e.to_string())))?; // If the JS input sanitization works, this should never error

    if start_x < 0 || start_y < 0 { return Err(error_svg("Start coordinate components must not be negative")); }
    let start_pos = Coord { x: start_x as usize, y: start_y as usize};

    // IDEA: Displace the root so that everybody fits? We don't know the final shape
    let lines: Vec<Line> = get_system_l_lines(n, &axiom, &rules, &turtle_map, start_pos, start_direction, line_length)
	.map_err(|e| error_svg(&e))?;

    let mut buffer = String::new();

    buffer.push_str(&format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"l-system-holder\">\n"));
    for Line { a: Coord { x: x1, y: y1 }, b: Coord { x: x2, y: y2 } } in lines {
        buffer.push_str(&format!(
            "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"{COLOR}\" stroke-width=\"5\"/>",
        ));
    }
    buffer.push_str(&format!("</svg>\n"));

    return Ok(buffer);
}

pub fn get_system_l_lines(
    n: i32,
    axiom: &str,
    rules: &HashMap<String, String>,
    turtle: &HashMap<char, LSystemTurtle>,
    start_pos: Coord,
    start_direction: f64,
    line_length: i32
) -> Result<Vec<Line>, String> {
    let mut lines = Vec::new();

    let mut state = axiom.to_owned();
    for _ in 0..n {
	state = advance_l_system(&state, rules);
    }

    let mut pos = start_pos;
    let mut direction = start_direction;
    for c in state.chars() {
	if let Some(turtl) = turtle.get(&c) {
	    const DEG90: f64 = TAU/4.0;
	    const DEG45: f64 = DEG90 / 2.0;
	    match turtl {
		LSystemTurtle::DoNothing => { },
		LSystemTurtle::DrawForward => {
		    let end = Coord {
			x: pos.x + (line_length as f64 * direction.cos()).round() as usize,
			y: pos.x + (line_length as f64 * direction.sin()).round() as usize,
		    };
		    lines.push(Line { a: pos, b: end });
		    pos = end;
		},
		LSystemTurtle::Rotate45CW => direction  -= DEG45,
		LSystemTurtle::Rotate90CW => direction  -= DEG90,
		LSystemTurtle::Rotate45CCW => direction += DEG45,
		LSystemTurtle::Rotate90CCW => direction += DEG90,
	    }
	} else {
	    return Err(format!("Variable/constant {c} has no assigned turtle action"));
	}
    }


    Ok(lines)
} 
    
/// Advance the system one iteration by applying all rules at once.
///
/// The variables/constants are implied by the current state, their distinction meaningless as if one does not exist in the rules map then it passes through unchanged
fn advance_l_system(axiom: &str, rules: &HashMap<String, String>) -> String {
    axiom.chars()
	.map(String::from)
	.map(|s| if let Some(r) = rules.get(&*s) { r.to_string() } else { s })
	.collect()
}


// Testsss
fn check_correctness(axiom: &str, stats: Vec<&str>, rules: &HashMap<String, String>) {
    let mut status = axiom.to_owned();
    for stat in stats {
	status = advance_l_system(&status, &rules);
	assert_eq!(status, stat);
    }
}

#[test]
fn algues() {
    let axiom = "A";
    let rules = HashMap::from([
	("A".into(), "AB".into()),
	("B".into(), "A".into())
    ]);
    let stats = ["AB", "ABA", "ABAAB", "ABAABABA", "ABAABABAABAAB", "ABAABABAABAABABAABABA", "ABAABABAABAABABAABABAABAABABAABAAB",];

    check_correctness(axiom, stats.to_vec(), &rules);
}

#[test]
fn bintree() {
    let axiom = "0";
    let rules = HashMap::from([
	("1".into(), "11".into()),
	("0".into(), "1[0]0".into())
    ]);
    let stats = ["1[0]0", "11[1[0]0]1[0]0", "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0"];

    check_correctness(axiom, stats.to_vec(), &rules);
}

// Turtl for the dragon:
// - 'F' means go forward
// - '-' means turn left 90 degrees
// - '+' means turn right 90 degrees
// - 'X' or 'Y' means do nothing

#[test]
fn dragon() {
    //let axiom = "ummm";
    //let rules = HashMap::from([
    //	("X", "X+YF+"),
    //	("Y", "-FX-Y")
    //]);

    //check_correctness(axiom, stats.to_vec(), &rules);
}

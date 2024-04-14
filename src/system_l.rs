use std::collections::HashMap;

use crate::*;

const WIDTH: usize = 2000;
const HEIGHT: usize = WIDTH;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum LSystemTurtle {
    DoNothing,
    DrawForward(f64),
    Rotate(f64),  // Rotate in degrees
    Advance(f64), // Multiplier of side length
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RulesMap {
    val: HashMap<String, String>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TurtleMapping {
    val: HashMap<char, LSystemTurtle>
}

#[wasm_bindgen]
pub fn get_empty_rules_map() -> JsValue {
    set_panic_hook();
    let r = RulesMap { val: HashMap::new() };
    serde_wasm_bindgen::to_value(&r).unwrap()
}

#[wasm_bindgen]
pub fn get_empty_turtle_map() -> JsValue {
    set_panic_hook();
    let r = TurtleMapping { val: HashMap::new() };
    serde_wasm_bindgen::to_value(&r).unwrap()
}

#[wasm_bindgen]
pub fn add_to_turtle_map(turtle_mapping: JsValue, c: char, s: String, amount: f64) -> Result<JsValue, String> {
    set_panic_hook();
    let TurtleMapping { val: mut turtle_map } = serde_wasm_bindgen::from_value(turtle_mapping).unwrap();

    let v: LSystemTurtle = match &*s {
	"rotate" => LSystemTurtle::Rotate(amount),
	"advance" => LSystemTurtle::Advance(amount),
	"draw" => LSystemTurtle::DrawForward(amount),
	_ => return Err(format!("{s} is not a valid enum variant")),
    };
    turtle_map.insert(c, v);

    let new_turtle_mapping = TurtleMapping{ val: turtle_map };

    let new_map: JsValue = serde_wasm_bindgen::to_value(&new_turtle_mapping).expect("What the fuck? How did this crash?");
    Ok(new_map)
}

#[wasm_bindgen]
pub fn add_to_rules_map(rules_mapping: JsValue, lhs: String, rhs: String) -> Result<JsValue, String> {
    set_panic_hook();
    let RulesMap { val: mut rules_map } = serde_wasm_bindgen::from_value(rules_mapping).unwrap();
    rules_map.insert(lhs, rhs);
    let new_rules_mapping = RulesMap { val: rules_map };

    let new_map: JsValue = serde_wasm_bindgen::to_value(&new_rules_mapping).expect("What the fuck? How did this crash?");
    Ok(new_map)
}
  
#[wasm_bindgen]
pub fn generate_l_fractal(
    n: i32,
    axiom: String,
    rules: JsValue,
    turtle_mapping: JsValue,
    start_x: i32,
    start_y: i32,
    start_direction: f64,
    line_length: i32
) -> Result<String, String> {
    set_panic_hook();
    let error_svg = |e: &str| {
        format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"l-system-holder\">\n <text x=\"20\" y=\"20\">{e}'</text></svg>\n")
    };

    let RulesMap { val: rules } = serde_wasm_bindgen::from_value(rules)
	.map_err(|in_e| error_svg(&format!("The Rules that advance the axiom are invalid: '{}'", in_e.to_string())))?;
    let TurtleMapping { val: turtle_map } = serde_wasm_bindgen::from_value(turtle_mapping)
	.map_err(|in_e| error_svg(&format!("The Turle Mapping is invalid: '{}'", in_e.to_string())))?; // If the JS input sanitization works, this should never error

    if start_x < 0 || start_y < 0 { return Err(error_svg("Start coordinate components must not be negative")); }
    let start_pos = Coord { x: start_x as usize, y: start_y as usize};

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
    for _ in 0..n { state = advance_l_system(&state, rules); } // Get final state

    let mut pos = start_pos;
    let mut direction = start_direction;
    for c in state.chars() {
	if let Some(turtl) = turtle.get(&c) {
	    match turtl {
		LSystemTurtle::DrawForward(lambda) => {
		    let end = Coord {
			x: (pos.x as f64 + line_length as f64 * lambda * direction.cos()).round() as usize,
			y: (pos.y as f64 + line_length as f64 * lambda * direction.sin()).round() as usize,
		    };
		    lines.push(Line { a: pos, b: end });
		    pos = end;
		},
		LSystemTurtle::Rotate(theta) => direction += theta * TAU / 360.0,
		LSystemTurtle::Advance(lambda) => { // Same code as draw forwards but without pushing the line
		    let end = Coord {
			x: (pos.x as f64 + line_length as f64 * lambda * direction.cos()).round() as usize,
			y: (pos.y as f64 + line_length as f64 * lambda * direction.sin()).round() as usize,
		    };
		    lines.push(Line { a: pos, b: end });
		    pos = end;
		}
		LSystemTurtle::DoNothing => { },
	    }
	} else {
	    //return Err(format!("Variable/constant {c} has no assigned turtle action"));
	     { } // If it has nothing, we assume they meant to do nothing
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

use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LayoutCode {
    Dvorak,
    Qwerty,
    Colemak,
}

impl FromStr for LayoutCode {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "dvorak" => Ok(LayoutCode::Dvorak),
            "qwerty" => Ok(LayoutCode::Qwerty),
            "colemak" => Ok(LayoutCode::Colemak),
            _ => Err(()),
        }
    }
}

fn create_keymaps() -> HashMap<(LayoutCode, LayoutCode), HashMap<char, char>> {
    let mut keymaps = HashMap::new();

    let qwerty_to_dvorak = qwerty_to_dvorak();
    let qwerty_to_colemak = qwerty_to_colemak();

    keymaps.insert((LayoutCode::Qwerty, LayoutCode::Dvorak), qwerty_to_dvorak);
    keymaps.insert((LayoutCode::Qwerty, LayoutCode::Colemak), qwerty_to_colemak);

    generate_inverse_maps(&mut keymaps);

    generate_composite_maps(&mut keymaps);

    keymaps
}

// Generates inverse maps for all direct maps to/from Qwerty
fn generate_inverse_maps(keymaps: &mut HashMap<(LayoutCode, LayoutCode), HashMap<char, char>>) {
    let layouts = vec![LayoutCode::Dvorak, LayoutCode::Colemak];
    for &layout in &layouts {
        if let Some(map) = keymaps.get(&(LayoutCode::Qwerty, layout)) {
            let inverse_map = invert_map(map);
            keymaps.insert((layout, LayoutCode::Qwerty), inverse_map);
        }
    }
}

// Function to generate composite maps between all layouts via Qwerty
fn generate_composite_maps(keymaps: &mut HashMap<(LayoutCode, LayoutCode), HashMap<char, char>>) {
    let layouts = vec![LayoutCode::Dvorak, LayoutCode::Colemak];
    for &from in &layouts {
        for &to in &layouts {
            if from != to {
                if let Some(map_to_qwerty) = keymaps.get(&(from, LayoutCode::Qwerty)) {
                    if let Some(map_from_qwerty) = keymaps.get(&(LayoutCode::Qwerty, to)) {
                        let combined_map = combine_maps(map_to_qwerty, map_from_qwerty);
                        keymaps.insert((from, to), combined_map);
                    }
                }
            }
        }
    }
}

fn invert_map(map: &HashMap<char, char>) -> HashMap<char, char> {
    map.iter().map(|(k, v)| (*v, *k)).collect()
}

fn combine_maps(first: &HashMap<char, char>, second: &HashMap<char, char>) -> HashMap<char, char> {
    first
        .iter()
        .map(|(&k, &v)| (k, second.get(&v).copied().unwrap_or(v)))
        .collect()
}

fn convertion_map(from: LayoutCode, to: LayoutCode) -> HashMap<char, char> {
    let keymaps = create_keymaps();
    keymaps.get(&(from, to)).unwrap().clone()
}

pub fn convert_text(text: String, from: LayoutCode, to: LayoutCode) -> String {
    let map = convertion_map(from, to);
    text.chars()
        .map(|c| map.get(&c).copied().unwrap_or(c))
        .collect()
}

fn qwerty_to_dvorak() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('q', '\'');
    map.insert('w', ',');
    map.insert('e', '.');
    map.insert('r', 'p');
    map.insert('t', 'y');
    map.insert('y', 'f');
    map.insert('u', 'g');
    map.insert('i', 'c');
    map.insert('o', 'r');
    map.insert('p', 'l');
    map.insert('s', 'o');
    map.insert('d', 'e');
    map.insert('f', 'u');
    map.insert('g', 'i');
    map.insert('h', 'd');
    map.insert('j', 'h');
    map.insert('k', 't');
    map.insert('l', 'n');
    map.insert('z', ';');
    map.insert('x', 'q');
    map.insert('c', 'j');
    map.insert('v', 'k');
    map.insert('b', 'x');
    map.insert('n', 'b');
    map.insert(',', 'w');
    map.insert('.', 'v');
    map.insert(';', 's');
    map.insert('/', 'z');
    map.insert('\'', '-');
    map.insert('[', '/');
    map.insert(']', '=');
    map.insert('-', '[');
    map.insert('=', ']');
    // capital letters
    map.insert('Q', '"');
    map.insert('W', '<');
    map.insert('E', '>');
    map.insert('R', 'P');
    map.insert('T', 'Y');
    map.insert('Y', 'F');
    map.insert('U', 'G');
    map.insert('I', 'C');
    map.insert('O', 'R');
    map.insert('P', 'L');
    map.insert('S', 'O');
    map.insert('D', 'E');
    map.insert('F', 'U');
    map.insert('G', 'I');
    map.insert('H', 'D');
    map.insert('J', 'H');
    map.insert('K', 'T');
    map.insert('L', 'N');
    map.insert('Z', ':');
    map.insert('X', 'Q');
    map.insert('C', 'J');
    map.insert('V', 'K');
    map.insert('B', 'X');
    map.insert('N', 'B');
    map.insert('<', 'W');
    map.insert('>', 'V');
    map.insert(':', 'S');
    map.insert('?', 'Z');
    map.insert('"', '_');
    map.insert('{', '?');
    map.insert('}', '+');
    map.insert('_', '{');
    map.insert('+', '}');
    map
}

fn qwerty_to_colemak() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('e', 'f');
    map.insert('r', 'p');
    map.insert('t', 'g');
    map.insert('y', 'j');
    map.insert('u', 'l');
    map.insert('i', 'u');
    map.insert('o', 'y');
    map.insert('p', ';');
    map.insert('s', 'r');
    map.insert('d', 's');
    map.insert('f', 't');
    map.insert('g', 'd');
    map.insert('h', 'h');
    map.insert('j', 'n');
    map.insert('k', 'e');
    map.insert('l', 'i');
    map.insert(';', 'p');
    map.insert('\'', '-');
    map.insert('-', '\'');
    // capital letters
    map.insert('E', 'F');
    map.insert('R', 'P');
    map.insert('T', 'G');
    map.insert('Y', 'J');
    map.insert('U', 'L');
    map.insert('I', 'U');
    map.insert('O', 'Y');
    map.insert('P', ':');
    map.insert('S', 'R');
    map.insert('D', 'S');
    map.insert('F', 'T');
    map.insert('G', 'D');
    map.insert('H', 'H');
    map.insert('J', 'N');
    map.insert('K', 'E');
    map.insert('L', 'I');
    map.insert(':', 'P');
    map.insert('"', '_');
    map.insert('_', '"');
    map
}

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LayoutCode {
    Dvorak,
    Qwerty,
    Colemak,
    Russian,
}

impl FromStr for LayoutCode {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "dvorak" => Ok(LayoutCode::Dvorak),
            "qwerty" => Ok(LayoutCode::Qwerty),
            "colemak" => Ok(LayoutCode::Colemak),
            "russian" => Ok(LayoutCode::Russian),
            _ => Err(()),
        }
    }
}

lazy_static! {
    static ref KEYMAPS: HashMap<(LayoutCode, LayoutCode), HashMap<char, char>> = create_keymaps();
}

fn create_keymaps() -> HashMap<(LayoutCode, LayoutCode), HashMap<char, char>> {
    let mut keymaps = HashMap::new();

    keymaps.insert((LayoutCode::Qwerty, LayoutCode::Dvorak), qwerty_to_dvorak());
    keymaps.insert(
        (LayoutCode::Qwerty, LayoutCode::Colemak),
        qwerty_to_colemak(),
    );
    keymaps.insert(
        (LayoutCode::Qwerty, LayoutCode::Russian),
        qwerty_to_russian(),
    );

    generate_inverse_maps(&mut keymaps);

    generate_composite_maps(&mut keymaps);

    keymaps
}

// Generates inverse maps for all direct maps to/from Qwerty
fn generate_inverse_maps(keymaps: &mut HashMap<(LayoutCode, LayoutCode), HashMap<char, char>>) {
    let layouts = vec![LayoutCode::Dvorak, LayoutCode::Colemak, LayoutCode::Russian];

    for &layout in &layouts {
        if let Some(map) = keymaps.get(&(LayoutCode::Qwerty, layout)) {
            let inverse_map = invert_map(map);
            keymaps.insert((layout, LayoutCode::Qwerty), inverse_map);
        }
    }
}

// Function to generate composite maps between all layouts via Qwerty
fn generate_composite_maps(keymaps: &mut HashMap<(LayoutCode, LayoutCode), HashMap<char, char>>) {
    let layouts = vec![LayoutCode::Dvorak, LayoutCode::Colemak, LayoutCode::Russian];
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

pub fn convert_text(text: String, from: LayoutCode, to: LayoutCode) -> String {
    if let Some(map) = KEYMAPS.get(&(from, to)) {
        text.chars()
            .map(|c| map.get(&c).copied().unwrap_or(c)) // Safe because `unwrap_or` provides a default
            .collect()
    } else {
        // Log the error or handle the case when map is not found
        eprintln!("Error: No conversion map found for {:?} to {:?}", from, to);
        text // Optionally, return the original text or a specific error message
    }
}

pub fn parallel_convert_text(text: String, from: LayoutCode, to: LayoutCode) -> String {
    const THRESHOLD: usize = 1000;
    const MAX_THREADS: usize = 4;
    if text.len() > THRESHOLD {
        let chunk_size = text.len() / MAX_THREADS;
        let chunks: Vec<String> = text
            .chars()
            .collect::<Vec<char>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect();
        let mut converted_chunks = Vec::new();
        for chunk in chunks {
            let from = from;
            let to = to;
            let handle = std::thread::spawn(move || convert_text(chunk, from, to));
            converted_chunks.push(handle);
        }
        converted_chunks
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    } else {
        convert_text(text, from, to)
    }
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

fn qwerty_to_russian() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('q', 'й');
    map.insert('w', 'ц');
    map.insert('e', 'у');
    map.insert('r', 'к');
    map.insert('t', 'е');
    map.insert('y', 'н');
    map.insert('u', 'г');
    map.insert('i', 'ш');
    map.insert('o', 'щ');
    map.insert('p', 'з');
    map.insert('[', 'х');
    map.insert(']', 'ъ');
    map.insert('a', 'ф');
    map.insert('s', 'ы');
    map.insert('d', 'в');
    map.insert('f', 'а');
    map.insert('g', 'п');
    map.insert('h', 'р');
    map.insert('j', 'о');
    map.insert('k', 'л');
    map.insert('l', 'д');
    map.insert(';', 'ж');
    map.insert('\'', 'э');
    map.insert('z', 'я');
    map.insert('x', 'ч');
    map.insert('c', 'с');
    map.insert('v', 'м');
    map.insert('b', 'и');
    map.insert('n', 'т');
    map.insert('m', 'ь');
    map.insert(',', 'б');
    map.insert('.', 'ю');
    map.insert('/', '.');
    // capital letters
    map.insert('Q', 'Й');
    map.insert('W', 'Ц');
    map.insert('E', 'У');
    map.insert('R', 'К');
    map.insert('T', 'Е');
    map.insert('Y', 'Н');
    map.insert('U', 'Г');
    map.insert('I', 'Ш');
    map.insert('O', 'Щ');
    map.insert('P', 'З');
    map.insert('{', 'Х');
    map.insert('}', 'Ъ');
    map.insert('A', 'Ф');
    map.insert('S', 'Ы');
    map.insert('D', 'В');
    map.insert('F', 'А');
    map.insert('G', 'П');
    map.insert('H', 'Р');
    map.insert('J', 'О');
    map.insert('K', 'Л');
    map.insert('L', 'Д');
    map.insert(':', 'Ж');
    map.insert('"', 'Э');
    map.insert('Z', 'Я');
    map.insert('X', 'Ч');
    map.insert('C', 'С');
    map.insert('V', 'М');
    map.insert('B', 'И');
    map.insert('N', 'Т');
    map.insert('M', 'Ь');
    map.insert('<', 'Б');
    map.insert('>', 'Ю');
    map.insert('?', ',');
    map
}

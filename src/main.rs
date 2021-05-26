use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let _args = std::env::args();
    let keyboard = KeyboardBuilder::build([
        ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
        ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
        ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
    ]);

    let mut key_logger = KeyLogger::new(keyboard);
    KeyLogger::log(&mut key_logger, &'h');
    KeyLogger::log(&mut key_logger, &'h');
    KeyLogger::log(&mut key_logger, &'g');

    key_logger
        .finger_movements_map
        .iter()
        .for_each(|val| println!("{:?}", val))
}

struct Key {
    value: char,
    finger: u8,
    pos: (i8, i8),
}

impl Key {
    fn new(value: char, finger: u8, pos: (i8, i8)) -> Self {
        Key { value, finger, pos }
    }
}

struct KeyboardBuilder {}

impl KeyboardBuilder {
    fn build(char_layout: [[char; 10]; 3]) -> HashMap<char, Key> {
        char_layout
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().enumerate().map(move |(x, char)| {
                    (
                        char.to_owned(),
                        Key::new(
                            char.to_owned(),
                            KeyboardBuilder::get_finger(x as u8),
                            KeyboardBuilder::get_pos(x as u8, y as u8),
                        ),
                    )
                })
            })
            .flatten()
            .collect()
    }

    fn get_finger(x: u8) -> u8 {
        if x < 4 {
            x
        } else if x < 6 {
            x - 1
        } else {
            x - 2
        }
    }

    fn get_pos(x: u8, y: u8) -> (i8, i8) {
        let iy = y as i8;
        let pos_y: i8 = -(iy - 1);

        let ix = x as i8;
        let mut pos_x: i8 = -((ix * 2) - 9);

        if pos_x > 1 || pos_x < -1 {
            pos_x = 0;
        }

        return (pos_x, pos_y);
    }
}

struct KeyLogger {
    keyboard: HashMap<char, Key>,

    finger_movements_map: HashMap<(i8, i8), u8>,
    finger_usage_map: HashMap<u8, u8>,

    prev_finger: i8,
    prev_char: char,
    same_finger_usage: u8,
}

impl KeyLogger {
    fn new(keyboard: HashMap<char, Key>) -> Self {
        KeyLogger {
            keyboard,

            finger_movements_map: HashMap::new(),
            finger_usage_map: HashMap::new(),

            prev_finger: -1,
            prev_char: '\0',
            same_finger_usage: 0,
        }
    }

    fn log(&mut self, char: &char) {
        if let Some(key) = self.keyboard.get(char) {
            match self.finger_movements_map.entry(key.pos) {
                Entry::Occupied(mut o) => o.insert(o.get() + 1),
                Entry::Vacant(v) => *v.insert(1u8),
            };

            match self.finger_usage_map.entry(key.finger) {
                Entry::Occupied(mut o) => o.insert(o.get() + 1),
                Entry::Vacant(v) => *v.insert(1u8),
            };

            if self.prev_finger == key.finger as i8 && self.prev_char != key.value {
                self.same_finger_usage += 1;
            }
        };
    }
}

#[cfg(test)]
mod keyboard_tests {

    use super::KeyLogger;
    use super::KeyboardBuilder;
    use std::collections::HashMap;

    fn get_key_logger() -> KeyLogger {
        let keyboard = KeyboardBuilder::build([
            ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
            ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
        ]);

        KeyLogger::new(keyboard)
    }

    #[test]
    fn qwerty_finger_movement_validation_test() {
        let mut key_logger = get_key_logger();

        let mut key_sequence: HashMap<(i8, i8), Vec<char>> = HashMap::new();

        key_sequence.insert((0, 1), vec!['q', 'w', 'e', 'r', 'u', 'i', 'o', 'p']);
        key_sequence.insert((0, 0), vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';']);
        key_sequence.insert((0, -1), vec!['z', 'x', 'c', 'v', 'm', ',', '.', '/']);

        key_sequence.insert((1, 0), vec!['g']);
        key_sequence.insert((-1, 0), vec!['h']);

        key_sequence.insert((1, 1), vec!['t']);
        key_sequence.insert((1, -1), vec!['b']);

        key_sequence.insert((-1, 1), vec!['y']);
        key_sequence.insert((-1, -1), vec!['n']);

        key_sequence.iter().for_each(|entry| {
            entry.1.iter().enumerate().for_each(|(idx, char)| {
                key_logger.log(char);

                match key_logger.finger_movements_map.get(entry.0) {
                    Some(i) => assert_eq!(*i, idx as u8 + 1),
                    None => panic!("movement {:?} not found", entry.0),
                }
            })
        });
    }

    #[test]
    fn qwerty_finger_usage_validation_test() {
        let mut key_logger = get_key_logger();
        let str: &str = "qwertyuiop asdfghjkl; zxcvbnm,./";
        let expected_result = [3, 3, 3, 6, 6, 3, 3, 3];

        str.chars().for_each(|char| {
            key_logger.log(&char);
        });

        key_logger.finger_usage_map.iter().for_each(|( finger, count)| {
            let expected_count = expected_result.get(*finger as usize).unwrap();
            assert_eq!(expected_count, count);
        })
    }
}

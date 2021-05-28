use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;

fn main() {
    let qwerty = KeyboardBuilder::build([
        ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
        ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
        ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
    ]);

    let dvorak = KeyboardBuilder::build([
        ['\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l'],
        ['a', 'o', 'e', 'u', 'i', 'd', 'h', 't', 'n', 's'],
        [';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z'],
    ]);

    let mut loggers: [(&str, KeyLogger); 2] = [
        ("QWERTY", KeyLogger::new(qwerty)),
        ("DVORAK", KeyLogger::new(dvorak)),
    ];

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    break;
                } else {
                    for char in input.chars() {
                        for i in 0..loggers.len() {
                            loggers[i].1.log(&char);
                        }
                    }
                }
            }

            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }

    let mut report = LogReport::new();
    loggers
        .iter()
        .for_each(|(name, logger)| report.add_logger(name.to_owned().to_owned(), logger));
    report.print();
}

#[derive(Debug)]
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

#[derive(Debug)]
struct KeyLogger {
    keyboard: HashMap<char, Key>,

    finger_movements_map: HashMap<(i8, i8), u8>,
    finger_usage_map: HashMap<u8, u8>,

    prev_finger: Option<u8>,
    prev_char: char,
    same_finger_usage: u8,
}

impl KeyLogger {
    fn new(keyboard: HashMap<char, Key>) -> Self {
        KeyLogger {
            keyboard,

            finger_movements_map: HashMap::new(),
            finger_usage_map: HashMap::new(),

            prev_finger: Option::None,
            prev_char: '\0',
            same_finger_usage: 0,
        }
    }

    fn log(&mut self, char: &char) -> () {
        if let Some(key) = self.keyboard.get(char) {
            match self.finger_movements_map.entry(key.pos) {
                Entry::Occupied(mut o) => o.insert(o.get() + 1),
                Entry::Vacant(v) => *v.insert(1u8),
            };

            match self.finger_usage_map.entry(key.finger) {
                Entry::Occupied(mut o) => o.insert(o.get() + 1),
                Entry::Vacant(v) => *v.insert(1u8),
            };

            if !self.prev_finger.is_none()
                && self.prev_finger.unwrap() == key.finger
                && self.prev_char != key.value
            {
                self.same_finger_usage += 1;
            }

            self.prev_finger = Option::Some(key.finger);
            self.prev_char = key.value;
        };
    }
}

struct LogReport<'a> {
    key_loggers: HashMap<String, &'a KeyLogger>,
    row_headers: Vec<String>,
    movement_header_map: Vec<((i8, i8), String)>,
}

impl<'a> LogReport<'a> {
    fn new() -> Self {
        let movement_header_map: Vec<((i8, i8), String)> = vec![
            ((0, 0), "No Movement".to_string()),
            ((0, 1), "Up Movement".to_string()),
            ((0, -1), "Down Movement".to_string()),
            ((1, 0), "Right Movement".to_string()),
            ((-1, 0), "Left Movement".to_string()),
            ((1, 1), "Top Right Movement".to_string()),
            ((-1, 1), "Top Left Movement".to_string()),
            ((1, -1), "Bottom Right Movement".to_string()),
            ((-1, -1), "Bottom Left Movement".to_string()),
        ];

        let mut row_headers: Vec<String> = vec![
            "Finger Movements".to_string(),
            "Same Finger Usage".to_string(),
        ];

        row_headers.append(
            &mut movement_header_map
                .iter()
                .map(|(_, header)| header.to_owned())
                .collect(),
        );

        LogReport {
            key_loggers: HashMap::new(),
            row_headers,
            movement_header_map,
        }
    }

    fn add_logger(&mut self, name: String, logger: &'a KeyLogger) {
        self.key_loggers.insert(name, logger);
    }

    fn print(&self) {
        let table_data = self.get_table_body_data();
        self.print_table_headers(&self.get_table_header_data());
        self.print_table_body(&self.row_headers, &table_data);
    }

    fn print_table_body(&self, row_headers: &Vec<String>, data_table: &Vec<Vec<u8>>) {
        data_table.iter().enumerate().for_each(|(idx, row)| {
            let mut str: String = String::from("");

            str.push_str(&format!("{:<30}", row_headers[idx]));

            row.iter()
                .for_each(|row| str.push_str(&format!("{:<15}", row)));

            println!("{}", str);
        });
    }

    fn print_table_headers(&self, names: &Vec<String>) {
        let mut str: String = String::new();

        str.push_str(&format!("{:<30}", "CATEGORY"));

        for name in names {
            str.push_str(&format!("{:<15}", name));
        }

        println!("{}", str);
    }

    fn get_table_header_data(&self) -> Vec<String> {
        self.key_loggers.keys().map(|key| key.to_owned()).collect()
    }

    fn get_table_body_data(&self) -> Vec<Vec<u8>> {
        // rotate the matrix
        let mut data_table: Vec<Vec<u8>> = Vec::new();
        let logger_count = self.key_loggers.len();
        let log_data = self.get_log_data_from_key_logger();

        for cell_idx in 0..11 {
            let mut row: Vec<u8> = Vec::with_capacity(logger_count);

            for row_idx in 0..logger_count {
                row.insert(row_idx, log_data[row_idx][cell_idx]);
            }

            data_table.push(row.to_vec());
        }

        return data_table;
    }

    fn get_log_data_from_key_logger(&self) -> Vec<Vec<u8>> {
        self.key_loggers
            .values()
            .map(|logger| -> Vec<u8> {
                let mut row: Vec<u8> = Vec::new();

                // total finger movements
                row.push(
                    logger
                        .finger_movements_map
                        .iter()
                        .filter(|(movement, _)| *movement != &(0i8, 0i8))
                        .map(|(_, count)| count)
                        .sum(),
                );

                // same finger usage
                row.push(logger.same_finger_usage);

                // individual finger movements
                self.movement_header_map.iter().for_each(|(key, _)| {
                    row.push(*logger.finger_movements_map.get(key).unwrap_or(&0u8))
                });

                return row;
            })
            .collect()
    }
}

#[cfg(test)]
mod keyboard_tests {

    use super::KeyLogger;
    use super::KeyboardBuilder;
    use super::LogReport;
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
    fn qwerty_finger_movement_validation() {
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
    fn qwerty_finger_usage_alphabet_validation() {
        let mut key_logger = get_key_logger();
        let str: &str = "qwertyuiop asdfghjkl; zxcvbnm,./";
        let expected_result = [3, 3, 3, 6, 6, 3, 3, 3];

        str.chars().for_each(|char| {
            key_logger.log(&char);
        });

        key_logger
            .finger_usage_map
            .iter()
            .for_each(|(finger, count)| {
                let expected_count = expected_result.get(*finger as usize).unwrap();
                assert_eq!(expected_count, count);
            })
    }

    #[test]
    fn qwerty_finger_usage_word_validation() {
        let word_expected_result_map = [
            ("hello", [3, 1, 2, 2, 0, 0, 1, 0, 0, 0, 0]),
            ("world", [3, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0]),
            ("back", [2, 0, 2, 0, 1, 0, 0, 0, 0, 1, 0]),
            ("value", [3, 0, 2, 2, 1, 0, 0, 0, 0, 0, 0]),
        ];

        word_expected_result_map
            .iter()
            .for_each(|(word, expected_res)| {
                println!("Testing the word::{}", word);

                let mut key_logger = get_key_logger();

                word.chars().for_each(|char| {
                    &key_logger.log(&char);
                });

                let mut log_report = LogReport::new();
                log_report.add_logger("QWERTY".to_string(), &key_logger);

                let actual_res = &log_report.get_log_data_from_key_logger()[0];

                println!("{:?}\n{:?}", actual_res, expected_res);

                assert!(Iterator::eq(actual_res.iter(), expected_res.iter()));
            })
    }
}

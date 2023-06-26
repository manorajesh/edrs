use crossterm::{self, cursor, event::KeyCode, queue, style::Stylize};
use std::{
    cmp::min,
    io::{ErrorKind, Stdout, Write},
};

use crate::{TextBuf, TABLENGTH};

pub fn get_key() -> KeyCode {
    loop {
        if let crossterm::event::Event::Key(event) = crossterm::event::read().unwrap() {
            if event.kind == crossterm::event::KeyEventKind::Press {
                return event.code;
            }
        }
    }
}

pub fn process_key_code(key: KeyCode, textbuf: &mut TextBuf) {
    match key {
        KeyCode::Char(c) => {
            if textbuf.cursor.1 >= textbuf.row_buffer.len() {
                textbuf.row_buffer.push(Vec::new());
            }

            textbuf.row_buffer[textbuf.cursor.1].insert(textbuf.cursor.0, c);

            textbuf.cursor.0 += 1;
        }

        KeyCode::Backspace => {
            if textbuf.cursor.0 > 0 {
                textbuf.cursor.0 -= 1;
                textbuf.row_buffer[textbuf.cursor.1].remove(textbuf.cursor.0);
            } else if textbuf.cursor.1 > 0 {
                textbuf.row_buffer.remove(textbuf.cursor.1);
                textbuf.cursor.1 -= 1;
                textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
            }
        }

        KeyCode::Up => {
            if textbuf.cursor.1 > 0 {
                textbuf.cursor.1 -= 1;
                if textbuf.cursor.0 > textbuf.row_buffer[textbuf.cursor.1].len() {
                    textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
                }
            } else {
                textbuf.cursor.0 = 0;
            }
        }

        KeyCode::Down => {
            if textbuf.cursor.1 < textbuf.row_buffer.len() - 1 {
                textbuf.cursor.1 += 1;
                if textbuf.cursor.0 > textbuf.row_buffer[textbuf.cursor.1].len() {
                    textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
                }
            } else {
                textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
            }
        }

        KeyCode::Left => {
            if textbuf.cursor.0 > 0 {
                textbuf.cursor.0 -= 1;
            } else if textbuf.cursor.1 > 0 {
                textbuf.cursor.1 -= 1;
                textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
            }
        }

        KeyCode::Right => {
            if textbuf.cursor.0 < textbuf.row_buffer[textbuf.cursor.1].len() {
                textbuf.cursor.0 += 1;
            } else if textbuf.cursor.1 < textbuf.row_buffer.len() - 1 {
                textbuf.cursor.1 += 1;
                textbuf.cursor.0 = 0;
            }
        }

        KeyCode::Enter => {
            if textbuf.row_buffer.len() > textbuf.cursor.1 {
                let element = textbuf.row_buffer[textbuf.cursor.1].split_off(textbuf.cursor.0);
                textbuf.cursor.0 = 0;
                textbuf.cursor.1 += 1;
                textbuf.row_buffer.insert(textbuf.cursor.1, element);
            } else {
                textbuf.row_buffer.push(Vec::new());
                textbuf.cursor.0 = 0;
                textbuf.cursor.1 += 1;
            }
        }

        KeyCode::Tab => {
            if textbuf.cursor.1 >= textbuf.row_buffer.len() {
                textbuf.row_buffer.push(Vec::new());
            }

            for _ in 0..TABLENGTH {
                textbuf.row_buffer[textbuf.cursor.1].insert(textbuf.cursor.0, ' ');
                textbuf.cursor.0 += 1;
            }
        }
        _ => {}
    }
}

fn viewport_bounding(textbuf: &mut TextBuf) {
    // vertical
    if textbuf.cursor.1 < textbuf.viewport_v_offset {
        textbuf.viewport_v_offset = textbuf.cursor.1;
    } else if textbuf.cursor.1 >= textbuf.viewport_v_offset + textbuf.dimensions.1 as usize {
        textbuf.viewport_v_offset = textbuf.cursor.1 - textbuf.dimensions.1 as usize + 1;
    }

    // horizontal
    if textbuf.cursor.0 < textbuf.viewport_h_offset {
        textbuf.viewport_h_offset = textbuf.cursor.0;
    } else if textbuf.cursor.0 >= textbuf.viewport_h_offset + textbuf.dimensions.0 as usize {
        textbuf.viewport_h_offset = textbuf.cursor.0 - textbuf.dimensions.0 as usize + 1;
    }
}

pub fn render_textbuf(textbuf: &mut TextBuf, stdout: &mut Stdout) {
    queue!(stdout, cursor::Hide).unwrap();
    queue!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();

    viewport_bounding(textbuf);

    let vstart = textbuf.viewport_v_offset;
    let vend = min(
        textbuf.row_buffer.len(),
        textbuf.viewport_v_offset + textbuf.dimensions.1 as usize,
    );

    for (idx, row) in textbuf.row_buffer[vstart..vend].iter().enumerate() {
        queue!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();

        let hstart = textbuf.viewport_h_offset;
        let hend = min(
            row.len(),
            textbuf.viewport_h_offset + textbuf.dimensions.0 as usize,
        );

        if hend > hstart {
            for c in &row[hstart..hend] {
                print!("{}", c);
            }
        }

        queue!(stdout, crossterm::cursor::MoveTo(0, idx as u16 + 1)).unwrap();
    }

    // draw tildes
    let empty_line_char = if textbuf.viewport_h_offset == 0 {
        '~'
    } else {
        ' '
    };
    for idx in vend..=textbuf.dimensions.1 as usize {
        queue!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();
        print!("{empty_line_char}");
        queue!(stdout, crossterm::cursor::MoveTo(0, idx as u16 + 1)).unwrap();
    }

    queue!(
        stdout,
        crossterm::cursor::MoveTo(
            (textbuf.cursor.0 - textbuf.viewport_h_offset) as u16,
            (textbuf.cursor.1 - textbuf.viewport_v_offset) as u16
        )
    )
    .unwrap();
    queue!(stdout, cursor::Show).unwrap();

    stdout.flush().unwrap();
}

pub fn popup(message: &str, stdout: &mut Stdout) {
    let (_, _) = crossterm::terminal::size().unwrap();

    // Move to top left corner and print message
    queue!(stdout, cursor::MoveTo(0, 0)).unwrap();
    queue!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)
    )
    .unwrap();
    print!("{}", message.negative());

    stdout.flush().unwrap();
}

pub fn save_prompt(textbuf: &mut TextBuf, stdout: &mut Stdout) {
    popup("Save file? (y/n)", stdout);

    'top: loop {
        let key = get_key();
        match key {
            KeyCode::Char('y') => match textbuf.save() {
                Ok(_) => {
                    popup("File saved!", stdout);
                    break;
                }
                Err(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        let mut filename = String::new();
                        loop {
                            popup(format!("Enter filename: {filename}").as_str(), stdout);
                            let key = get_key();
                            match key {
                                KeyCode::Char(c) => filename.push(c),
                                KeyCode::Backspace => { filename.pop(); }
                                KeyCode::Enter => break,
                                KeyCode::Esc => break 'top,
                                _ => continue,
                            };
                        }
                        textbuf.filename = Some(filename.clone());
                        match textbuf.save() {
                            Ok(_) => {
                                popup("File saved!", stdout);
                                break;
                            }
                            Err(_) => {
                                popup((format!("Error with filename: {filename} ({e})!")).as_str(), stdout);
                                break;
                            }
                        }
                    } else {
                        popup("Error saving file!", stdout);
                        break;
                    }
                }
            },
            KeyCode::Char('n') => {
                break;
            }
            _ => {}
        }
    }
}

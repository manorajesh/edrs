use crossterm::{self, cursor, event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind}, queue, style::Stylize};
use std::{
    cmp::min,
    io::{ErrorKind, Stdout, Write},
};

use crate::{TextBuf, TABLENGTH};

pub struct KeyStroke(pub KeyCode, KeyModifiers);

#[derive(PartialEq)]
pub enum InputEvent {
    KeyStroke(KeyCode, KeyModifiers),
    Mouse(MouseEvent),
    Resize(u16, u16),
}


pub fn get_key() -> KeyStroke {
    loop {
        if let crossterm::event::Event::Key(event) = crossterm::event::read().unwrap() {
            if event.kind == crossterm::event::KeyEventKind::Press {
                return KeyStroke(event.code, event.modifiers);
            }
        }
    }
}

pub fn get_event() -> InputEvent {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => {
                if event.kind == crossterm::event::KeyEventKind::Press {
                    return InputEvent::KeyStroke(event.code, event.modifiers);
                }
            },
            crossterm::event::Event::Mouse(event) => {
                return InputEvent::Mouse(event);
            },
            crossterm::event::Event::Resize(width, height) => {
                return InputEvent::Resize(width, height);
            }
            _ => {}
        }
    }
}

pub fn process_event(event: InputEvent, textbuf: &mut TextBuf) {
    match event {
        InputEvent::KeyStroke(key, modifiers) => {
            process_key_code(KeyStroke(key, modifiers), textbuf);
        },
        InputEvent::Mouse(mouse_event) => {
            process_mouse_code(mouse_event, textbuf);
        },
        InputEvent::Resize(width, height) => {
            textbuf.dimensions = (width, height);
        },
    }
}

fn process_mouse_code(event: MouseEvent, textbuf: &mut TextBuf) {
    match event.kind {
        MouseEventKind::ScrollDown => {
            if textbuf.row_buffer.len() > textbuf.dimensions.1 as usize && textbuf.viewport_v_offset < textbuf.row_buffer.len() - textbuf.dimensions.1 as usize {
                textbuf.viewport_v_offset += 1;
            }

            if textbuf.cursor.1 <= textbuf.viewport_v_offset {
                textbuf.cursor.1 = textbuf.viewport_v_offset;
            }
        },

        MouseEventKind::ScrollUp => {
            if textbuf.viewport_v_offset > 0 {
                textbuf.viewport_v_offset -= 1;
            }

            if textbuf.cursor.1 >= textbuf.viewport_v_offset {
                textbuf.cursor.1 = textbuf.viewport_v_offset + textbuf.dimensions.1 as usize - 1;
            }
        },

        MouseEventKind::Down(_) => {
            let mut x = event.column as usize + textbuf.viewport_h_offset;
            let mut y = event.row as usize + textbuf.viewport_v_offset;

            if textbuf.row_buffer.len() > 0 {
                if y > textbuf.row_buffer.len()-1 {
                    y = textbuf.row_buffer.len()-1;
                }
    
                if x > textbuf.row_buffer[y].len() {
                    x = textbuf.row_buffer[y].len();
                }
    
                textbuf.cursor = (x, y);
            }
        },

        _ => {}
    }
}

fn process_key_code(key: KeyStroke, textbuf: &mut TextBuf) {
    match key {
        KeyStroke(KeyCode::Char(c), KeyModifiers::NONE) => {
            if textbuf.cursor.1 >= textbuf.row_buffer.len() {
                textbuf.row_buffer.push(Vec::new());
            }

            textbuf.row_buffer[textbuf.cursor.1].insert(textbuf.cursor.0, c);

            textbuf.cursor.0 += 1;
        }

        KeyStroke(KeyCode::Backspace, _) => {
            if textbuf.cursor.0 > 0 {
                textbuf.cursor.0 -= 1;
                textbuf.row_buffer[textbuf.cursor.1].remove(textbuf.cursor.0);
            } else if textbuf.cursor.1 > 0 {
                textbuf.row_buffer.remove(textbuf.cursor.1);
                textbuf.cursor.1 -= 1;
                textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
            }
        }

        KeyStroke(KeyCode::Up, _) => {
            if textbuf.cursor.1 > 0 {
                textbuf.cursor.1 -= 1;
                if textbuf.cursor.0 > textbuf.row_buffer[textbuf.cursor.1].len() {
                    textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
                }
            } else {
                textbuf.cursor.0 = 0;
            }
        }

        KeyStroke(KeyCode::Down, _) => {
            if textbuf.cursor.1 < textbuf.row_buffer.len() - 1 {
                textbuf.cursor.1 += 1;
                if textbuf.cursor.0 > textbuf.row_buffer[textbuf.cursor.1].len() {
                    textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
                }
            } else {
                textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
            }
        }

        KeyStroke(KeyCode::Left, _) => {
            if textbuf.cursor.0 > 0 {
                textbuf.cursor.0 -= 1;
            } else if textbuf.cursor.1 > 0 {
                textbuf.cursor.1 -= 1;
                textbuf.cursor.0 = textbuf.row_buffer[textbuf.cursor.1].len();
            }
        }

        KeyStroke(KeyCode::Right, _) => {
            if textbuf.cursor.0 < textbuf.row_buffer[textbuf.cursor.1].len() {
                textbuf.cursor.0 += 1;
            } else if textbuf.cursor.1 < textbuf.row_buffer.len() - 1 {
                textbuf.cursor.1 += 1;
                textbuf.cursor.0 = 0;
            }
        }

        KeyStroke(KeyCode::Enter, _) => {
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

        KeyStroke(KeyCode::Tab, _) => {
            if textbuf.cursor.1 >= textbuf.row_buffer.len() {
                textbuf.row_buffer.push(Vec::new());
            }

            for _ in 0..TABLENGTH {
                textbuf.row_buffer[textbuf.cursor.1].insert(textbuf.cursor.0, ' ');
                textbuf.cursor.0 += 1;
            }
        }

        KeyStroke(KeyCode::PageUp, _) => {
            if textbuf.cursor.1 > textbuf.dimensions.1 as usize {
                textbuf.cursor.1 -= textbuf.dimensions.1 as usize-1;
            } else {
                textbuf.cursor.1 = 0;
            }
        }

        KeyStroke(KeyCode::PageDown, _) => {
            if (textbuf.cursor.1 + textbuf.dimensions.1 as usize) < textbuf.row_buffer.len() {
                textbuf.cursor.1 += textbuf.dimensions.1 as usize-1;
            } else {
                textbuf.cursor.1 = textbuf.row_buffer.len() - 1;
            }
        }

        KeyStroke(KeyCode::Char(c), KeyModifiers::CONTROL) => {
            match c {
                's' => {
                    match textbuf.save() {
                        Ok(_) => {}
                        Err(_) => {
                            match save_prompt(textbuf, &mut std::io::stdout()) {
                                Ok(_) => {}
                                Err(e) => {
                                    popup(&format!("Error: {}", e), &mut std::io::stdout());
                                }
                            }
                        }
                    }
                }
                'q' => {
                    std::process::exit(0);
                }
                _ => {}
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
        textbuf.viewport_v_offset + textbuf.dimensions.1 as usize + 1,
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
    if textbuf.viewport_h_offset == 0 {
        for idx in vend..=textbuf.dimensions.1 as usize {
            queue!(
                stdout,
                crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)
            )
            .unwrap();
            print!("~");
            queue!(stdout, crossterm::cursor::MoveTo(0, idx as u16)).unwrap();
        }
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

pub fn save_prompt(textbuf: &mut TextBuf, stdout: &mut Stdout) -> Result<(), std::io::Error> {
    popup("Save file? (y/n)", stdout);

    loop {
        let key = get_key();
        match key.0 {
            KeyCode::Char('y') => match textbuf.save() {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        let mut filename = String::new();
                        loop {
                            popup(format!("Enter filename: {filename}").as_str(), stdout);
                            let key = get_key();
                            match key.0 {
                                KeyCode::Char(c) => filename.push(c),
                                KeyCode::Backspace => {
                                    filename.pop();
                                }
                                KeyCode::Enter => break,
                                KeyCode::Esc => {
                                    return Err(std::io::Error::new(
                                        ErrorKind::Other,
                                        "User cancelled!",
                                    ))
                                }
                                _ => continue,
                            };
                        }
                        textbuf.filename = Some(filename.clone());
                        match textbuf.save() {
                            Ok(_) => {
                                return Ok(());
                            }
                            Err(_) => {
                                popup(
                                    (format!("Error with filename: '{filename}' ({e})!")).as_str(),
                                    stdout,
                                );
                                get_key();
                                return Err(std::io::Error::new(
                                    ErrorKind::Other,
                                    "Error saving file!",
                                ));
                            }
                        }
                    } else {
                        popup("Error saving file!", stdout);
                        get_key();
                        return Err(std::io::Error::new(ErrorKind::Other, "Error saving file!"));
                    }
                }
            },
            KeyCode::Char('n') => {
                return Ok(());
            }
            _ => {}
        }
    }
}

use crossterm::{self, event::KeyCode, queue, cursor};
use std::io::Write;

use crate::{TextBuf, EMP};

pub fn get_key() -> KeyCode {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => {
                if event.kind == crossterm::event::KeyEventKind::Press {
                    return event.code;
                }
            }
            _ => {}
        }
    }
}

pub fn process_key_code(key: KeyCode, textbuf: &mut TextBuf) {
    match key {
        KeyCode::Char(c) => {
            if textbuf.cursor.1 >= textbuf.row_bufs.len() {
                textbuf.row_bufs.push(vec![EMP; 1]);
            }

            if textbuf.cursor.0 >= textbuf.row_bufs[textbuf.cursor.1].len() {
                textbuf.row_bufs[textbuf.cursor.1].push(EMP);
            }

            textbuf.row_bufs[textbuf.cursor.1].insert(textbuf.cursor.0, c);

            if textbuf.cursor.0 < textbuf.dimensions.0 as usize - 1 {
                textbuf.cursor.0 += 1;
            } else {
                textbuf.cursor.1 += 1;
                textbuf.cursor.0 = 0;
            }
        }

        KeyCode::Backspace => {
            if textbuf.cursor.0 > 0 {
                textbuf.cursor.0 -= 1;
                textbuf.row_bufs[textbuf.cursor.1].remove(textbuf.cursor.0);
            } else if textbuf.cursor.1 > 0 {
                textbuf.cursor.1 -= 1;
                textbuf.cursor.0 = textbuf.dimensions.0 as usize - 1;
                textbuf.row_bufs[textbuf.cursor.1].remove(textbuf.cursor.0);
            }
        }

        KeyCode::Up => {
            if textbuf.cursor.1 > 0 {
                textbuf.cursor.1 -= 1;
                if textbuf.cursor.0 > textbuf.row_bufs[textbuf.cursor.1].len()-1 {
                    textbuf.cursor.0 = textbuf.row_bufs[textbuf.cursor.1].len()-1;
                }
            }
        }

        KeyCode::Down => {
            if textbuf.cursor.1 < textbuf.row_bufs.len() - 1 {
                textbuf.cursor.1 += 1;
                if textbuf.cursor.0 > textbuf.row_bufs[textbuf.cursor.1].len() {
                    textbuf.cursor.0 = textbuf.row_bufs[textbuf.cursor.1].len()-1;
                }
            }
        }

        KeyCode::Left => {
            if textbuf.cursor.0 > 0 {
                textbuf.cursor.0 -= 1;
            } else if textbuf.cursor.1 > 0 {
                textbuf.cursor.1 -= 1;
                textbuf.cursor.0 = textbuf.row_bufs[textbuf.cursor.1].len()-1;
            }
        }

        KeyCode::Right => {
            if textbuf.cursor.0 < textbuf.row_bufs[textbuf.cursor.1].len()-1 {
                textbuf.cursor.0 += 1;
            }
        }

        KeyCode::Enter => {
            textbuf.row_bufs[textbuf.cursor.1].insert(textbuf.cursor.0, '\n');
            textbuf.cursor.0 = 0;
            textbuf.cursor.1 += 1;
        }
        _ => {}
    }
}

pub fn draw_tildes(dimensions: (u16, u16)) -> Vec<Vec<char>> {
    let vec: Vec<Vec<char>> = Vec::new();
    for _ in 0..dimensions.1 {
        println!("~");
    }
    return vec;
}

pub fn render_textbuf(textbuf: &TextBuf, stdout: &mut std::io::Stdout) {
    queue!(stdout, cursor::Hide).unwrap();
    queue!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
    for (idx, row) in textbuf.row_bufs.iter().enumerate() {
        queue!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)).unwrap();
        for c in row {
            print!("{}", c);
        }
        queue!(stdout, crossterm::cursor::MoveTo(0, idx as u16+1)).unwrap();
    }

    queue!(stdout, crossterm::cursor::MoveTo(textbuf.cursor.0 as u16, textbuf.cursor.1 as u16)).unwrap();
    queue!(stdout, cursor::Show).unwrap();

    stdout.flush().unwrap();
}

mod io;
mod textbuf;

use crossterm::{
    cursor::SetCursorStyle,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};
use std::io::Write;

use crate::{io::{save_prompt, get_key}, textbuf::TextBuf};

pub const TABLENGTH: usize = 4;

fn main() {
    // terminal setup
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, Clear(crossterm::terminal::ClearType::All)).unwrap();
    execute!(stdout, SetCursorStyle::BlinkingBlock).unwrap();

    // initialize textbuf
    let mut textbuf = TextBuf::new();
    execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();

    // main loop
    loop {
        // draw textbuf
        io::render_textbuf(&mut textbuf, &mut stdout);

        // wait for keypress
        let key = io::get_key();
        if key == crossterm::event::KeyCode::Esc {
            break;
        }

        // process keypress
        io::process_key_code(key, &mut textbuf);
    }

    save_prompt(&mut textbuf, &mut stdout);

    // terminal cleanup
    disable_raw_mode().unwrap();

    // clear screen
    crossterm::queue!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
    execute!(stdout, SetCursorStyle::DefaultUserShape).unwrap();
    print!("{:#?}", textbuf);
}

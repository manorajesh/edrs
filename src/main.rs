mod io;
mod textbuf;
mod args;

use crossterm::{
    cursor::SetCursorStyle,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen},
};
use std::io::Write;
use clap::Parser;

use crate::{
    io::{get_key, process_key_code, render_textbuf, save_prompt, popup},
    textbuf::TextBuf,
};

pub const TABLENGTH: usize = 4;

fn main() {
    // parse args
    let args = args::Args::parse();

    // terminal setup
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, Clear(crossterm::terminal::ClearType::All)).unwrap();
    execute!(stdout, SetCursorStyle::BlinkingBlock).unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    // initialize textbuf
    let mut textbuf = if let Some(file) = args.file {
        TextBuf::load(&file).unwrap_or_else(|e| {
            popup(format!("Error loading file: {}", e).as_str(), &mut stdout);
            get_key();
            TextBuf::new()
        })
    } else {
        TextBuf::new()
    };
    execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();

    // main loop
    loop {
        // draw textbuf
        render_textbuf(&mut textbuf, &mut stdout);

        // wait for keypress
        let key = get_key();
        if key.0 == crossterm::event::KeyCode::Esc {
            match   save_prompt(&mut textbuf, &mut stdout) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }

        // process keypress
        process_key_code(key, &mut textbuf);
    }

    // terminal cleanup
    disable_raw_mode().unwrap();

    // clear screen
    crossterm::queue!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
    execute!(stdout, SetCursorStyle::DefaultUserShape).unwrap();
    // print!("{:#?}", textbuf);
}

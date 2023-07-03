mod args;
mod io;
mod textbuf;

use clap::Parser;
use crossterm::{
    cursor::SetCursorStyle,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use io::{nonblocking_get_event, process_event, InputEvent};
use std::{
    io::Write,
    sync::{Arc, Mutex},
    time::Duration,
};

use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

use crate::{
    io::{popup, render_textbuf, save_prompt},
    textbuf::TextBuf,
};

pub const TABLENGTH: usize = 4;

pub struct SynHighlighter {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub theme: String,
    pub use_colors: bool,
}

impl SynHighlighter {
    fn from(theme: String, use_colors: bool) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();

        SynHighlighter {
            syntax_set,
            theme_set,
            theme,
            use_colors,
        }
    }
}

fn main() {
    // parse args
    let args = args::Args::parse();

    // terminal setup
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::event::EnableMouseCapture).unwrap();
    execute!(stdout, Clear(crossterm::terminal::ClearType::All)).unwrap();
    execute!(stdout, SetCursorStyle::BlinkingBlock).unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    // SynHighlighter setup
    let syn_highlighter = SynHighlighter::from(args.theme, args.syntax);

    // initialize textbuf
    let textbuf = Arc::new(Mutex::new(TextBuf::new()));
    let textbuf_clone = Arc::clone(&textbuf);

    let thread_load = if let Some(file) = args.file {
        std::thread::spawn(move || {
            if let Err(e) = TextBuf::async_load(&file, &textbuf_clone) {
                popup(
                    format!("Error loading file: {}", e).as_str(),
                    &mut std::io::stdout(),
                );
            }
        })
    } else {
        std::thread::spawn(move || {})
    };

    execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();

    // main loop
    loop {
        // draw textbuf
        let mut textbuf_guard = textbuf.lock().unwrap();
        if textbuf_guard.dirty {
            render_textbuf(&mut textbuf_guard, &mut stdout, &syn_highlighter);
            textbuf_guard.dirty = true;
        }
        stdout.flush().unwrap();
        drop(textbuf_guard);

        // wait for keypress
        if let Some(key) = nonblocking_get_event() {
            if key
                == InputEvent::KeyStroke(
                    crossterm::event::KeyCode::Esc,
                    crossterm::event::KeyModifiers::NONE,
                )
            {
                let mut textbuf_guard = textbuf.lock().unwrap();
                match save_prompt(&mut textbuf_guard, &mut stdout) {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }

            // process keypress
            let mut textbuf_guard = textbuf.lock().unwrap();
            process_event(key, &mut textbuf_guard);
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    thread_load.join().unwrap();

    // terminal cleanup
    disable_raw_mode().unwrap();

    // clear screen
    crossterm::queue!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap();
    execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, SetCursorStyle::DefaultUserShape).unwrap();
    // print!("{:#?}", textbuf);
}

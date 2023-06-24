mod io;

use crossterm::{
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear}, execute, cursor::SetCursorStyle,
};
use io::draw_tildes;
use std::io::Write;

pub const EMP: char = '\0';

#[derive(Debug)]
pub struct TextBuf {
    pub row_bufs: Vec<Vec<char>>,
    pub cursor: (usize, usize),
    pub dimensions: (u16, u16),
}

impl TextBuf {
    fn new() -> Self {
        let dimensions = terminal::size().unwrap(); // (columns, rows)

        let vec = draw_tildes(dimensions);

        TextBuf {
            row_bufs: vec,
            cursor: (0, 0),
            dimensions: dimensions,
        }
    }
}

fn main() {
    // terminal setup
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, Clear(crossterm::terminal::ClearType::All)).unwrap();
    execute!(stdout, SetCursorStyle::BlinkingBlock).unwrap();

    // initialize textbuf
    let mut textbuf = TextBuf::new();
    stdout.flush().unwrap();

    // main loop
    loop {
        // wait for keypress
        let key = io::get_key();
        if key == crossterm::event::KeyCode::Esc {
            break;
        }

        // process keypress
        io::process_key_code(key, &mut textbuf);
        // draw textbuf
        io::render_textbuf(&textbuf, &mut stdout);
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
    print!("{:#?}", textbuf);
}

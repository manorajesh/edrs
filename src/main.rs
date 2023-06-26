mod io;

use crossterm::{
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear}, execute, cursor::SetCursorStyle,
};
use std::io::Write;

pub const TABLENGTH: usize = 4;

#[derive(Debug)]
pub struct TextBuf {
    pub row_buffer: Vec<Vec<char>>,
    pub cursor: (usize, usize),
    pub dimensions: (u16, u16),
    pub viewport_v_offset: usize, // offset to (start row, end row) of viewport 
    pub viewport_h_offset: usize, 
}

impl TextBuf {
    fn new() -> Self {
        let dimensions = terminal::size().unwrap(); // (columns, rows)

        let vec: Vec<Vec<char>> = Vec::new();

        TextBuf {
            row_buffer: vec,
            cursor: (0, 0),
            dimensions: dimensions,
            viewport_v_offset: 0,
            viewport_h_offset: 0,
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

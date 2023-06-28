use std::{
    fs::OpenOptions,
    io::{Read, Write}, sync::{Arc, Mutex},
};

use crossterm::terminal;

use crate::TABLENGTH;

#[derive(Debug, Clone, PartialEq)]
pub struct TextBuf {
    pub row_buffer: Vec<Vec<char>>,
    pub cursor: (usize, usize),
    pub dimensions: (u16, u16),
    pub viewport_v_offset: usize, // offset to (start row, end row) of viewport
    pub viewport_h_offset: usize,
    pub filename: Option<String>,
    pub save_changed: bool,
    pub dirty: bool,
}

impl TextBuf {
    pub fn new() -> Self {
        let dimensions = terminal::size().unwrap(); // (columns, rows)

        let vec: Vec<Vec<char>> = Vec::new();

        TextBuf {
            row_buffer: vec,
            cursor: (0, 0),
            dimensions,
            viewport_v_offset: 0,
            viewport_h_offset: 0,
            filename: None,
            save_changed: false,
            dirty: true, // force draw at start
        }
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(filename) = &self.filename {
            let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

            for row in &self.row_buffer {
                for c in row {
                    file.write_all(c.to_string().as_bytes())?;
                }
                file.write_all("\n".as_bytes())?;
            }

            self.save_changed = false;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No valid filename",
            ))
        }
    }

    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = OpenOptions::new()
            .write(false)
            .create(false)
            .read(true)
            .open(filename)?;

        let mut textbuf = TextBuf::new();

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        buf = buf.replace('\t', " ".repeat(TABLENGTH).as_str());

        for line in buf.lines() {
            let row = line.chars().collect();
            textbuf.row_buffer.push(row);
        }

        textbuf.filename = Some(filename.to_string());

        Ok(textbuf)
    }

    pub fn async_load(filename: &str, textbuf: &Arc<Mutex<TextBuf>>) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .write(false)
            .create(false)
            .read(true)
            .open(filename)?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        for line in buf.lines() {
            let row = line.chars().collect();
            textbuf.lock().unwrap().row_buffer.push(row);
            textbuf.lock().unwrap().dirty = true;
        }

        Ok(())
    }
}

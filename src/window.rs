use std::io::{Stdout, StdoutLock, Write};

use crossterm::event::{Event, KeyCode as Key, KeyEvent, KeyModifiers as Mod};
use crossterm::style::Print;
use crossterm::{cursor, queue};

use super::editor::Position;
use super::rows::Row;

#[derive(Clone, Default, Debug)]
pub struct Window {
    pub name: String,
    pub x1: u16,
    pub x2: u16,
    pub y1: u16,
    pub y2: u16,
    pub rows: Vec<Row>,
    /// For typing
    pub cursor_position: Position,
    pub string: Option<String>,
    pub cur_moved: bool,
}

impl Window {
    /// Param order: x1, x2, y1, y2
    pub fn new(name: String, x1: u16, x2: u16, y1: u16, y2: u16) -> Self {
        Self {
            name,
            x1,
            x2,
            y1,
            y2,
            rows: vec![],
            cursor_position: Position { x: 0, y: 0 },
            string: None,
            cur_moved: false,
        }
    }

    pub fn get_cursor_position(&self) -> (u16, u16) {
        cursor::position().unwrap()
    }

    pub fn draw_all(&self, stdout: &mut StdoutLock) {
        stdout.flush().unwrap();
    }

    pub fn draw_text_box(&mut self, stdout: &mut StdoutLock, x: Option<u16>) -> (u16, u16) {
        let Self { x1, x2, y1, y2, .. } = *self;
        let text_box_border = "-".repeat((x2 - x1 - 2).into());
        let text_entry_border = format!("+{}+", text_box_border);
        self.rows.push(Row::from(text_entry_border.as_str()));

        let text_box = if let Some(text) = &self.string {
            // NOTE: Risk of overflow
            let spaces = " ".repeat((x2 - x1 - text.len() as u16 - 5).into());
            format!("|-> {}{}|", text, spaces)
        } else {
            let spaces = " ".repeat((x2 - x1 - 5).into());
            format!("|-> {}|", spaces)
        };

        self.rows.push(Row::from(text_box.as_str()));

        let x = if let Some(x) = x { x } else { x1 + 4 };

        queue!(
            stdout,
            cursor::Show,
            cursor::MoveTo(x1, y2),
            Print(text_entry_border),
            cursor::MoveTo(x1, y2 - 1),
            Print(text_box),
            cursor::MoveTo(x, y2 - 1),
        )
        .unwrap();

        self.get_cursor_position()
    }

    pub fn draw_border(&mut self, stdout: &mut StdoutLock) {
        let Self { x1, x2, y1, y2, .. } = *self;

        let hori_line = (x2 - x1) as usize;

        let hori_fill = "-".repeat(hori_line - 2);
        let hori_border = format!("+{}+", hori_fill);

        // Handles the horizontal top and bottom walls
        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(x1, y1),
            Print(&hori_border),
            cursor::MoveTo(x1, y2 - 2),
            Print(&hori_border),
        )
        .unwrap();

        let mut y = y1 + 1;
        // TODO: Make this list come from somewhere else.
        let commands = vec!["Save file".to_string(), "Quit".to_string()];

        // the vertical left and right walls
        let mut num = 0;
        while y < y2 - 2 {
            let repeat = if let Some(command) = commands.get(num) {
                command.len()
            } else {
                0
            } as u16;

            // results window
            let text = if num < commands.len() {
                let spaces = " ".repeat((x2 - x1 - repeat - 2).into());
                let row = format!("|{}{}|", commands.get(num).unwrap(), spaces);

                self.rows.push(Row::from(row.clone().as_str()));
                row
            } else {
                let spaces = " ".repeat((x2 - x1 - 2).into());
                let row = format!("|{}|", spaces);

                self.rows.push(Row::from(row.clone().as_str()));
                row
            };

            queue!(stdout, cursor::MoveTo(x1, y as u16), Print(text)).unwrap();

            y += 1;
            num += 1;
        }

        queue!(stdout, cursor::Show).unwrap();
    }

    pub fn draw_command_window(&mut self, stdout: &mut Stdout) {
        let Self { x1, x2, y1, y2, .. } = *self;
        let hori_line = (x2 - x1) as usize;

        let hori_fill = "-".repeat(hori_line - 2);
        let hori_border = format!("+{}+", hori_fill);

        // Handles the horizontal top and bottom walls
        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(x1, y1),
            Print(&hori_border),
            cursor::MoveTo(x1, y2 - 2),
            cursor::MoveTo(x1, y1),
        )
        .unwrap();

        let mut y = y1 + 1;
        let commands = vec!["Save file".to_string(), "Quit".to_string()];

        // the vertical left and right walls
        let mut num = 0;
        while y < y2 - 2 {
            let repeat = if let Some(command) = commands.get(num) {
                command.len()
            } else {
                0
            } as u16;

            // results window
            let text = if num < commands.len() {
                let spaces = " ".repeat((x2 - x1 - repeat - 2).into());
                let row = format!("{}{}", commands.get(num).unwrap(), spaces);
                self.rows.push(Row::from(row.clone().as_str()));

                row
            } else {
                let spaces = " ".repeat((x2 - x1 - 2).into());
                let row = format!("{}", spaces);
                self.rows.push(Row::from(row.clone().as_str()));

                row
            };

            queue!(
                stdout,
                cursor::MoveTo(x1, y as u16),
                Print("|"),
                cursor::MoveTo(x1 + 1, y as u16),
                Print(text),
                Print("|"),
            )
            .unwrap();

            y += 1;
            num += 1;
        }

        queue!(stdout, cursor::Show).unwrap();
    }
}

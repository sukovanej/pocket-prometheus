use anyhow::{Context, Error};
use std::io::{self, Stdout, Write};

use crossterm::{cursor, terminal, QueueableCommand};
use terminal_size::{terminal_size, Height, Width};

use crate::query::MetricQuery;

struct StdoutManager {
    current_row: u16,
}

impl StdoutManager {
    fn new() -> Self {
        StdoutManager { current_row: 0 }
    }

    fn clear(&mut self, mut stdout: &Stdout) -> Result<(), io::Error> {
        self.set_cursor_position(0, 0, stdout)?;
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    fn write_line(&mut self, line: &str, mut stdout: &Stdout) -> Result<(), io::Error> {
        stdout.write_all(line.as_bytes())?;
        self.current_row += 1;
        stdout.queue(cursor::MoveTo(0, self.current_row))?;
        Ok(())
    }

    fn write(&mut self, line: &str, mut stdout: &Stdout) -> Result<(), io::Error> {
        stdout.write_all(line.as_bytes())?;
        Ok(())
    }

    fn set_cursor_position(
        &mut self,
        row: u16,
        column: u16,
        mut stdout: &Stdout,
    ) -> Result<(), io::Error> {
        stdout.queue(cursor::MoveTo(column, row))?;
        self.current_row = row;
        Ok(())
    }
}

pub fn redraw_stdout(
    query: &MetricQuery,
    data: String,
    mut stdout: &Stdout,
    scroll_offset: u32,
) -> Result<(), Error> {
    let (Width(width), Height(height)) = terminal_size().context("STDOUT is not tty")?;

    let mut manager = StdoutManager::new();
    manager.clear(stdout)?;

    let box_top = format!("┌{}┐", "─".repeat((width - 2).into()));
    manager.write_line(&box_top, stdout)?;

    manager.write("│ ", stdout)?;

    let query_input = format!("Query: {}", query.name);
    manager.write_line(&query_input, stdout)?;

    manager.set_cursor_position(1, width, stdout)?;
    manager.write_line("│", stdout)?;

    let box_bottom = format!("└{}┘", "─".repeat((width - 2).into()));
    manager.write_line(&box_bottom, stdout)?;

    manager.write_line(
        &format!(
            "  Help: <UP> / <DOWN> to move around, <ESC> to quit; Offset: {}",
            scroll_offset
        ),
        stdout,
    )?;
    manager.write_line("", stdout)?;

    let lines: Vec<&str> = data
        .split('\n')
        .skip(scroll_offset as usize)
        .take((height - 5) as usize)
        .collect::<Vec<&str>>();

    for line in lines {
        manager.write_line(line, stdout)?;
    }

    manager.set_cursor_position(1, (query_input.len() + 2) as u16, stdout)?;

    stdout.flush()?;
    Ok(())
}

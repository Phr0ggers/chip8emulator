use crossterm::{
  cursor::MoveTo, event::{read, Event, KeyCode},
  execute,
  queue,
  style::Print,
  terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
  ErrorKind
};
use std::io::{stdout, Write};

use super::Frontend;
use crate::display::{HEIGHT, WIDTH};


const ON: &str = "██";
const OFF: &str = "  ";

pub struct CrosstermFrontend;

impl CrosstermFrontend {
  pub fn new() -> Self {
    CrosstermFrontend
  }

  pub fn draw_square(&self) -> crossterm::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All))?;

    let square_size = 10;

    for row in 0..square_size {
      queue!(stdout, MoveTo(0, row))?;

      for col in 0..square_size {
        if row == 0 || row == square_size - 1 || col == 0 || col == square_size - 1 {
          queue!(stdout, Print(ON))?;
        } else {
          queue!(stdout, Print(OFF))?;
        }
      }
    }

    stdout.flush()?;

    let _ = enable_raw_mode();

    loop {
      if let Event::Key(event) = read()? {
        if event.code == KeyCode::Char('q') {
          break;
        }
      }
    }

    let _ = disable_raw_mode();

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    Ok(())

  }
}

impl Frontend for CrosstermFrontend{
  type Error = ErrorKind;
  fn draw_display(&self, display: [[bool; WIDTH]; HEIGHT]) -> Result<(), Self::Error> {
    Ok(())
  }
}



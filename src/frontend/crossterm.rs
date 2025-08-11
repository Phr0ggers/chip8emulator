
use super::Frontend;

use crossterm::{
  cursor::MoveTo, event::{read, Event, KeyCode}, execute, queue, style::Print, terminal::{Clear, ClearType}
};

use std::io::{stdout, Write};

pub struct CrosstermFrontend;

impl CrosstermFrontend {
  pub fn new() -> Self {
    CrosstermFrontend
  }


  pub fn draw_display() -> crossterm::Result<()> {
    Ok(())
  }
}

impl Frontend for CrosstermFrontend {
}


pub fn draw_square() -> crossterm::Result<()> {
  let mut stdout = stdout();

  execute!(stdout, Clear(ClearType::All))?;

  let square_size = 10;

  for row in 0..square_size {
    queue!(stdout, MoveTo(0, row))?;

    for col in 0..square_size {
      if row == 0 || row == square_size - 1 || col == 0 || col == square_size - 1 {
        queue!(stdout, Print("#"))?;
      } else {
        queue!(stdout, Print(" "))?;
      }
    }
  }

  stdout.flush()?;

  loop {
    if let Event::Key(event) = read()? {
      if event.code != KeyCode::Null {
        break;
      }
    }
  }

  execute!(stdout, Clear(ClearType::All))?;

  Ok(())

}

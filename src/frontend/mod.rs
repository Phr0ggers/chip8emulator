use crate::display::{HEIGHT, WIDTH};

pub mod crossterm;

/// UI util
pub trait Frontend {
  type Error;

  /// Draws the current pixel occupancy of 'display'
  fn draw_display(&self, display: &[[bool; WIDTH]; HEIGHT]) -> Result<(), Self::Error>;

  // TODO: some input handling fn
}

use crate::display::{HEIGHT, WIDTH};

pub mod crossterm;

pub trait Frontend {
  type Error;
  fn draw_display(&self, display: [[bool; WIDTH]; HEIGHT]) -> Result<(), Self::Error>;
}

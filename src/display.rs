pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
  // 64x32 frame buffer
  frame_buffer: [[bool; WIDTH]; HEIGHT]
}

impl Display {
    pub fn new() -> Self {
    Display { frame_buffer: [[false; WIDTH]; HEIGHT] }
  }

  pub fn clear(&mut self) {
    for row in self.frame_buffer.iter_mut() {
      for pixel in row.iter_mut() {
        *pixel = false;
      }
    }
  }

  pub fn get_pixel(&mut self, row: usize, col: usize) -> bool {
    self.frame_buffer[row % HEIGHT][col % WIDTH]
  }

  pub fn set_pixel(&mut self, row: usize, col: usize, val: bool) {
    self.frame_buffer[row % HEIGHT][col % WIDTH] = val;
  }

  pub fn get_display(&mut self) -> &[[bool; WIDTH]; HEIGHT] {
    &self.frame_buffer
  }
}

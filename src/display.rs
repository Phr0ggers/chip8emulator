use crossterm::terminal::ScrollDown;

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

  pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: [u8; 5], rows: u8) -> bool {
    let mut coll = false;
    for k in 0..rows {
      coll = coll && self.draw_row_from_u8(x as usize % WIDTH, (y + k) as usize % HEIGHT, sprite[k as usize])
    };
    coll
  }

  fn draw_row_from_u8(&mut self, x: usize, y: usize, pixels: u8) -> bool {
    let mut t = 0b1000_0000;
    let mut coll = false;
    for i in 0..8 {
      if (t & pixels) != 0 {
        if self.frame_buffer[x][y + 1] { coll = true; }
        self.frame_buffer[x][y + i] = !self.frame_buffer[x][y + 1];

      }
      t = t >> 1;
    };
    coll
  }

  // INFO: Very important fn!!!
  pub fn draw_seal(&mut self) {
    //baseline
    for i in 0..20 {
      self.set_pixel(18, (i + 3) as usize, true);
    }

    //row up
    self.set_pixel(17, 3, true);
    self.set_pixel(17, 9, true);
    self.set_pixel(17, 16, true);
    self.set_pixel(17, 22, true);

    self.set_pixel(16, 3, true);
    self.set_pixel(16, 22, true);

    self.set_pixel(15, 3, true);
    self.set_pixel(15, 4, true);
    self.set_pixel(15, 21, true);
    self.set_pixel(15, 22, true);

    self.set_pixel(14, 4, true);
    self.set_pixel(14, 5, true);
    self.set_pixel(14, 20, true);
    self.set_pixel(14, 21, true);

    self.set_pixel(13, 5, true);
    self.set_pixel(13, 6, true);
    self.set_pixel(13, 19, true);
    self.set_pixel(13, 20, true);

    for i in 10..13 {
      self.set_pixel(i, 6, true);
      self.set_pixel(i, 19, true);
    }

    self.set_pixel(10, 7, true);
    self.set_pixel(10, 18, true);

    self.set_pixel(9, 7, true);
    self.set_pixel(9, 18, true);

    self.set_pixel(8, 7, true);
    self.set_pixel(8, 8, true);
    self.set_pixel(8, 17, true);
    self.set_pixel(8, 18, true);

    self.set_pixel(7, 10, true);
    self.set_pixel(7, 8, true);
    self.set_pixel(7, 9, true);
    self.set_pixel(7, 17, true);
    self.set_pixel(7, 15, true);
    self.set_pixel(7, 16, true);

    for i in 0..6 {
      self.set_pixel(6, i + 10, true);
    }

    //face
    for i in 0..2 {
      self.set_pixel(11 + i, 9, true);
      self.set_pixel(11 + i, 10, true);

      self.set_pixel(11 + i, 15, true);
      self.set_pixel(11 + i, 16, true);
    }

    for i in 0..2 {
      self.set_pixel(13, 12 + i, true);
    }

    for i in 0..4 {
      self.set_pixel(14, 11 + i, true);
    }
  }
}

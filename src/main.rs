use chip8emulator::frontend::{crossterm, Frontend};

fn main() {
  let ct = crossterm::CrosstermFrontend::new();

  let _ = ct.draw_square();
}

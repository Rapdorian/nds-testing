pub enum Driver {
    Main,
    Sub,
}

pub struct On;
pub struct Off;

pub struct TopLcd<P> {
    power: P,
}

impl TopLcd<Off> {
    pub fn on(self) -> TopLcd<On> {
        TopLcd { power: On }
    }
}

impl TopLcd<On> {
    pub fn off(self) -> TopLcd<Off> {
        TopLcd { power: Off }
    }
}

impl BottomLcd<Off>

//  let screen = hw.bottom_lcd();
//  screen.framebuffer();
//
//  while let Some(event) = hw.input.next() {
//      if let Event::KeyDown(key) {
//          match key {
//              Key::Up => left_bar.move(Dir::Up),
//              Key::Down => left_bar.move(Dir::Down),
//          }
//      } else if let Event::KeyUp(key) {
//          match key {
//              Key::Up => left_bar.stop(Dir::Up),
//              Key::Down => left_bar.stop(Dir::Up)
//          }
//      }
//      left_bar.update_pos();
//      left_bar.draw(screen);
//  }

use core::iter::Iterator;

pub trait Font {
    fn get(c: char) -> [bool; 64];
}

pub struct TestFont;
impl TestFont {
    const NONE: [bool; 64] = [false; 64];
}

macro_rules! token {
    (-) => {
        false
    };
    (_) => {
        false
    };
    (X) => {
        true
    };
}

macro_rules! glyph {
        ($($el:tt)*) => {[$(token!($el),)*]};
}

impl Font for TestFont {
    fn get(c: char) -> [bool; 64] {
        match c {
            'A' => [
                false, false, true, true, false, false, false, false, false, true, false, false,
                true, false, false, false, true, false, false, false, false, true, false, false,
                true, false, false, false, false, true, false, false, true, true, true, true, true,
                true, false, false, true, false, false, false, false, true, false, false, true,
                false, false, false, false, true, false, false, true, false, false, false, false,
                true, false, false,
            ],
            _ => Self::NONE,
        }
    }
}

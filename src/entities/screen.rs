pub struct Screen {
    pub width: i32,
    pub height: i32
}

impl Screen {
    pub fn new(width: i32, height: i32) -> Screen {
        Screen{width: width, height: height}
    }
}
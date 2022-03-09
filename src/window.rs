pub struct WindowInfoRessource {
    pub width: u32,
    pub height: u32,
}

impl WindowInfoRessource {
    pub fn width(&mut self, value: u32) {
        self.width = value;
    }

    pub fn height(&mut self, value: u32) {
        self.height = value;
    }
}


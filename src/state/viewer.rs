pub struct ViewerState {
    pub scroll_y: usize,
    pub scroll_x: usize,
}

impl ViewerState {
    pub fn new() -> Self {
        Self {
            scroll_y: 0,
            scroll_x: 0,
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll_y += 1;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_y = self.scroll_y.saturating_sub(1);
    }

    pub fn scroll_right(&mut self) {
        self.scroll_x += 4;
    }

    pub fn scroll_left(&mut self) {
        self.scroll_x = self.scroll_x.saturating_sub(4);
    }
}

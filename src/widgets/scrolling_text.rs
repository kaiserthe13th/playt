#[cfg(feature = "wide")]
use unicode_segmentation::UnicodeSegmentation;
use pancurses::Window;

use super::Area;

pub struct ScrollingText {
    pub pos: (i32, i32),
    pub area: (usize, usize),
    pub text: String,
    idx: usize,
    pub scroll_amount: usize,
}

impl ScrollingText {
    pub fn new(text: &str, area: (usize, usize), pos: (i32, i32)) -> Self {
        Self {
            pos,
            area,
            text: text.to_string(),
            idx: 0,
            scroll_amount: 1,
        }
    }

    pub fn scroll(&mut self) {
        let rem = &self.text[self.idx..];
        if self.area.0 * self.area.1 > rem.len() {
            self.idx = 0;
        } else {
            self.idx += self.scroll_amount;
        }
    }

    pub fn scroll_left(&mut self) {
        if self.idx > 0 {
            self.idx -= self.scroll_amount;
        } else {
            self.idx = self.text.len().checked_sub(self.area.0 * self.area.1).unwrap_or(0);
        }
    }

    #[cfg(feature = "wide")]
    pub fn draw(&self, win: &Window) {
        let mut rem = self.text.graphemes(true).skip(self.idx);
        for i in 0..self.area.0 {
            for j in 0..self.area.1 {
                if let Some(s) = rem.next() {
                    win.mvprintw(self.pos.0 + i as i32, self.pos.1 + j as i32, s);
                } else {
                    win.mvaddch(self.pos.0 + i as i32, self.pos.1 + j as i32, ' ');
                }
            }
        }
    }

    #[cfg(not(feature = "wide"))]
    pub fn draw(&self, win: &Window) {
        let mut rem = self.text.chars().skip(self.idx);
        for i in 0..self.area.0 {
            for j in 0..self.area.1 {
                if let Some(ch) = rem.next() {
                    win.mvaddch(self.pos.0 + i as i32, self.pos.1 + j as i32, ch);
                } else {
                    win.mvaddch(self.pos.0 + i as i32, self.pos.1 + j as i32, ' ');
                }
            }
        }
    }
}

impl Area for ScrollingText {
    fn area(&self) -> (usize, usize) {
        self.area
    }
}

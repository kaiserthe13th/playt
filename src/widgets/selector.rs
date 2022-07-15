use pancurses::{Window, chtype};
use crate::color;

use super::Area;

pub struct Selector {
    selections: Vec<String>,
    selections_max_len: usize,
    normal_attrs: chtype,
    selected_attrs: chtype,
    selected: usize,
    pub pos: (i32, i32),
}

impl Selector {
    pub fn new(selections: &[&str], pos: (i32, i32)) -> Self {
        Self {
            selections_max_len: selections.iter()
                .map(|a| a.len())
                .max().expect("expected a length over 0"),
            selections: selections.into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>(),
            selected: 0,
            normal_attrs: color::WHITE_ON_BLACK.into(),
            selected_attrs: color::BLACK_ON_WHITE.into(),
            pos,
        }
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn no_select(&mut self) {
        self.selected = self.selections.len();
    }

    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
    
    pub fn down(&mut self) {
        if self.selected < self.selections.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn set_selections(&mut self, selections: &[&str]) {
        self.selections_max_len = selections.iter()
            .map(|a| a.len())
            .max().expect("expected a length over 0");
        self.selections = selections.into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>();
        self.selected = 0;
    }

    pub fn set_normal_attrs<T: Into<chtype>>(&mut self, normal_attrs: T) {
        self.normal_attrs = normal_attrs.into();
    }

    pub fn set_selected_attrs<T: Into<chtype>>(&mut self, selected_attrs: T) {
        self.selected_attrs = selected_attrs.into();
    }

    pub fn selections(&self) -> &Vec<String> {
        &self.selections
    }

    pub fn draw(&self, win: &Window) {
        for (index, selection) in self.selections.iter().enumerate() {
            let color_choice = if index == self.selected { self.selected_attrs } else { self.normal_attrs };
            win.attron(color_choice);
            win.mvprintw(self.pos.0 + index as i32, self.pos.1, selection);
            win.attroff(color_choice);
        }
    }
}

impl Area for Selector {
    fn area(&self) -> (usize, usize) {
        (self.selections.len(), self.selections_max_len)
    }
}

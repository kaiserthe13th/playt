//! Contains the [`Game`](crate::game::Game) struct

use pancurses::{cbreak, curs_set, endwin, has_colors, initscr, noecho, start_color, Window};

use crate::{color, prelude::Stage};

/// A Game is used to manage global data of your game
/// It also holds information about the game [`Window`](pancurses::Window)
pub struct Game<G> {
    globals: G,
    running: bool,
    win: Window,
    nodelay: bool,
}

impl<G> Drop for Game<G> {
    fn drop(&mut self) {
        endwin();
    }
}

impl<G> Game<G> {
    /// Initialize a new [`Game`](Self) with its initial data
    pub fn new(initial_globals: G) -> Self {
        let win = Self::init();
        Self {
            globals: initial_globals,
            running: true,
            nodelay: true,
            win,
        }
    }

    /// Initialize a new [`Game`](Self) with its initial data
    /// and try to add colors to the [`Game`](Self). If not successfull return [`None`].
    pub fn with_colors(initial_globals: G) -> Option<Self> {
        let win = Self::init();
        if !has_colors() {
            endwin();
            return None;
        }
        start_color();
        color::init_colors();
        Some(Self {
            globals: initial_globals,
            running: true,
            nodelay: true,
            win,
        })
    }

    #[inline]
    pub fn nodelay(&mut self, nodelay: bool) {
        self.nodelay = nodelay;
        self.win.nodelay(self.nodelay);
    }

    fn init() -> Window {
        let win = initscr();
        win.keypad(true);
        win.nodelay(true);
        cbreak();
        noecho();
        curs_set(0);
        win
    }

    /// Get a reference to your global data
    pub fn globals(&self) -> &G {
        &self.globals
    }
    /// Get a mutable reference to your global data
    pub fn globals_mut(&mut self) -> &mut G {
        &mut self.globals
    }
    /// Refresh the window and check if running
    pub fn is_running(&self) -> bool {
        self.win().refresh();
        self.running
    }
    /// Check if running
    pub fn running(&self) -> bool {
        self.running
    }
    /// Start game, running = true
    pub fn start(&mut self) {
        self.running = true;
    }
    /// Stop game, running = false
    pub fn stop(&mut self) {
        self.running = false;
    }
    /// Get a reference to the game [`Window`](pancurses::Window)
    pub fn win(&self) -> &Window {
        &self.win
    }
    /// Get a mutable reference to the game [`Window`](pancurses::Window)
    pub fn win_mut(&mut self) -> &mut Window {
        &mut self.win
    }

    /// Performs(draws and updates) a [`Stage`](crate::stage::Stage)
    pub fn perform<T, E>(&mut self, stage: &mut Stage<T, E, G>) -> Result<(), E> {
        if stage.nodelay != self.nodelay {
            self.nodelay = stage.nodelay;
            self.win.nodelay(self.nodelay);
        }
        stage.draw(self)?;
        stage.update(self)?;
        Ok(())
    }
}

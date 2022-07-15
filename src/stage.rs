//! Contains the [`Stage`](crate::stage::Stage) struct
use pancurses::{Input, Window};

use crate::game::Game;

type UpdateFn<T, E, G> = fn(&mut Game<G>, &mut T, Option<Input>) -> Result<(), E>;
type DrawFn<T, E, G> = fn(&Game<G>, &mut T, &Window) -> Result<(), E>;

pub struct StageBuilder<T, E, G> {
    state: T,
    update: UpdateFn<T, E, G>,
    draw: DrawFn<T, E, G>,
    clear_on_resize: bool,
    nodelay: bool,
}

impl<T, E, G> StageBuilder<T, E, G> {
    pub fn new(initial_state: T) -> Self {
        Self {
            state: initial_state,
            update: |_, _, _| Ok(()),
            draw: |_, _, _| Ok(()),
            clear_on_resize: false,
            nodelay: false,
        }
    }

    pub fn clear_on_resize(self, clear_on_resize: bool) -> Self {
        Self {
            clear_on_resize,
            ..self
        }
    }

    pub fn nodelay(self, nodelay: bool) -> Self {
        Self {
            nodelay,
            ..self
        }
    }

    pub fn update(self, f: UpdateFn<T, E, G>) -> Self {
        Self { update: f, ..self }
    }
    pub fn draw(self, f: DrawFn<T, E, G>) -> Self {
        Self { draw: f, ..self }
    }
    pub fn build(self) -> Stage<T, E, G> {
        Stage {
            state: self.state,
            update: self.update,
            draw: self.draw,
            clear_on_resize: self.clear_on_resize,
            nodelay: self.nodelay,
        }
    }
}

pub struct Stage<T, E, G> {
    state: T,
    clear_on_resize: bool,
    pub(crate) nodelay: bool,
    update: UpdateFn<T, E, G>,
    draw: DrawFn<T, E, G>,
}

impl<T, E, G> Stage<T, E, G> {
    pub fn new(initial_state: T) -> StageBuilder<T, E, G> {
        StageBuilder::new(initial_state)
    }
    pub fn state(&self) -> &T {
        &self.state
    }
    pub fn state_mut(&mut self) -> &mut T {
        &mut self.state
    }
    pub fn update(&mut self, game: &mut Game<G>) -> Result<(), E> {
        let i = game.win().getch();
        if self.clear_on_resize {
            if let Some(Input::KeyResize) = i {
                game.win().clear();
            }
        }
        (self.update)(game, self.state_mut(), i)
    }
    pub fn draw(&mut self, game: &Game<G>) -> Result<(), E> {
        (self.draw)(game, self.state_mut(), game.win())
    }
}

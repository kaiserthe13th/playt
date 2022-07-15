//! Contains the [`Stage`](crate::stage::Stage) struct
use std::sync::Arc;

use pancurses::{Input, Window};

use crate::game::Game;

pub struct StageBuilder<T, E, G> {
    state: T,
    update: Option<Arc<dyn Fn(&mut Game<G>, &mut T, Option<Input>) -> Result<(), E>>>,
    draw: Option<Arc<dyn Fn(&Game<G>, &mut T, &Window) -> Result<(), E>>>,
    clear_on_resize: bool,
    nodelay: bool,
}

impl<T, E, G> StageBuilder<T, E, G> {
    pub fn new(initial_state: T) -> Self {
        Self {
            state: initial_state,
            update: None,
            draw: None,
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

    pub fn update<F: 'static>(self, f: F) -> Self
    where F: Fn(&mut Game<G>, &mut T, Option<Input>) -> Result<(), E>
    {
        Self {
            update: Some(Arc::new(f)),
            ..self
        }
    }
    pub fn draw<F: 'static>(self, f: F) -> Self
    where F: Fn(&Game<G>, &mut T, &Window) -> Result<(), E>
    {
        Self {
            draw: Some(Arc::new(f)),
            ..self
        }
    }
    pub fn build(self) -> Stage<T, E, G> {
        Stage {
            state: self.state,
            update: self.update.unwrap_or(Arc::new(|_, _, _| Ok(()))),
            draw: self.draw.unwrap_or(Arc::new(|_, _, _| Ok(()))),
            clear_on_resize: self.clear_on_resize,
            nodelay: self.nodelay,
        }
    }
}

pub struct Stage<T, E, G> {
    state: T,
    clear_on_resize: bool,
    pub(crate) nodelay: bool,
    update: Arc<dyn Fn(&mut Game<G>, &mut T, Option<Input>) -> Result<(), E>>,
    draw: Arc<dyn Fn(&Game<G>, &mut T, &Window) -> Result<(), E>>,
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
    pub fn state_consume(self) -> T {
        self.state
    }
    pub fn update(&mut self, game: &mut Game<G>) -> Result<(), E> {
        let i = game.win().getch();
        if self.clear_on_resize {
            if let Some(Input::KeyResize) = i {
                game.win().clear();
            }
        }
        let update = Arc::clone(&self.update);
        update(game, self.state_mut(), i)
    }
    pub fn draw(&mut self, game: &Game<G>) -> Result<(), E> {
        let draw = Arc::clone(&self.draw);
        draw(game, self.state_mut(), game.win())
    }
}

//! **Read like plate, Playt with fun**
//!
//! Playt is a game library for the terminal. <br>
//! It functions as a basic wrapper over pancurses by providing utility functions and extending types.
//! It also helps you manage the state of your game by separating it into [`Stage`](crate::stage::Stage)s
//! and putting global data in a [`Game`](crate::game::Game) instance.

pub mod color;
pub mod ext;
pub mod game;
pub mod stage;

pub mod prelude {
    use crate::{ext, game, stage};

    pub use crate::color;
    pub use ext::*;
    pub use game::Game;
    pub use stage::Stage;
}

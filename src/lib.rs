//! **Read like plate, Playt with fun**
//! 
//! Playt is a game library for the terminal. <br>
//! It functions as a basic wrapper over pancurses by providing utility functions and extending types.
//! It also helps you manage the state of your game by separating it into [`Stage`](crate::stage::Stage)s
//! and putting global data in a [`Game`](crate::game::Game) instance.

pub mod color;
pub mod stage;
pub mod game;
pub mod ext;

pub mod prelude {
    use crate::{
        stage,
        game,
        ext
    };

    pub use crate::color;
    pub use stage::Stage;
    pub use game::Game;
    pub use ext::*;
}

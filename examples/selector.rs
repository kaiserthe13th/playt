use std::convert::Infallible;

use pancurses::Input;
use playt::{prelude::*, widgets::Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::with_colors(()).expect("couldn't initialize with colors");

    let mut select = Selector::new(&[
        "Do Something",
        "Make something with...",
        "Nothing",
        "Quit",
    ], (0, 0));
    select.set_selected_attrs(color::GREEN_ON_BLACK | pancurses::Attribute::Bold);

    let mut stage = Stage::new(select)
        .clear_on_resize(true)
        .nodelay(false)
        .draw(|_, s, win| -> Result<(), Infallible> {
            s.pos = win.get_mid_yx(s.area_i32());
            s.draw(win);
            
            let s = "[Space/Enter] Select | [W/Up] Up | [S/Down] Down | [Q] Quit";
            win.mvprintw(win.get_max_y() - 1, win.get_mid_x(s.len() as i32), s);
            Ok(())
        })
        .update(|game, s, input| {
            match input.unwrap() {
                Input::Character('q') => game.stop(),
                Input::Character('w') | Input::KeyUp => s.up(),
                Input::Character('s') | Input::KeyDown => s.down(),
                Input::Character(' ') | Input::Character('\n') => {
                    if s.selected() == 3 {
                        game.stop();
                    }
                    game.win().clear();
                    let s = &s.selections()[s.selected()];
                    game.win().mvprintw(0, game.win().get_mid_x(s.len() as i32), s);
                },
                _ => (),
            }
            Ok(())
        })
        .build();

    while game.is_running() {
        game.perform(&mut stage)?;
    }
    Ok(())
}

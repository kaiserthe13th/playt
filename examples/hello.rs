use pancurses::Input;
use playt::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::with_colors(())
        .expect("couldn't initialize with colors");
    
    let mut stage = Stage::new(())
        .clear_on_resize(true)
        .draw(|_, _, win| -> Result<(), ()> {
            win.with_attr(color::GREEN_ON_BLACK, |win| {
                let greeting = "Hello, World!";
                win.mvprintw(win.get_mid_y(0), win.get_mid_x(greeting.len() as i32), greeting);
            });
            Ok(())
        })
        .update(|game, _, input| {
            if let Some(Input::Character('q')) = input {
                game.stop();
            }
            Ok(())
        })
        .build();
    
    while game.is_running() {
        game.perform(&mut stage).unwrap();
    }
    Ok(())
}

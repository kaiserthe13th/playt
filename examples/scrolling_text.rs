use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, self};
use std::{convert::Infallible, sync::Arc};

use std::time::Duration;
use std::thread;

use pancurses::Input;
use playt::{prelude::*, widgets::ScrollingText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::with_colors(()).expect("couldn't initialize with colors");

    let st = Arc::new(Mutex::new(ScrollingText::new("Lorem ipsum dolor sit amet. Consectetur ...", (2, 6), (0, 0))));
    let stt = Arc::clone(&st);
    let running = Arc::new(AtomicBool::new(true));
    let runningt = Arc::clone(&running);
    
    thread::spawn(move || {
        while runningt.load(atomic::Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
            stt.lock().expect("couldn't obtain lock").scroll();
        }
    });

    let mut stage = Stage::new(st)
        .clear_on_resize(true)
        .nodelay(true)
        .draw(move |_, s, win| -> Result<(), Infallible> {
            let mut stl = s.lock().expect("couldn't obtain lock");
            stl.pos = win.get_mid_yx(stl.area_i32());
            win.attron(color::MAGENTA_ON_BLACK);
            stl.draw(win);
            win.attroff(color::MAGENTA_ON_BLACK);
            
            let s = "[Left] Move Left | [Right] Move Right | [Q] Quit";
            win.mvprintw(win.get_max_y() - 1, win.get_mid_x(s.len() as i32), s);
            Ok(())
        })
        .update(move |game, st, inp| {
            match inp {
                Some(Input::Character('q')) => {
                    running.store(false, atomic::Ordering::Relaxed);
                    game.stop();
                },
                Some(Input::KeyLeft) => {
                    st.lock().expect("couldn't obtain lock").scroll_left();
                },
                Some(Input::KeyRight) => {
                    st.lock().expect("couldn't obtain lock").scroll();
                }
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

//! Color utilities for [`pancurses`]

use pancurses::{init_pair, ColorPair};

/// Initializes every [`ColorPair`](pancurses::ColorPair) in this module.
///
/// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
/// <strong>Warning:</strong> But you don't have to call it yourself if you use <a href="file:///home/k13/code/rs/playt/target/doc/playt/game/struct.Game.html#method.with_colors"><code>Game::with_colors</code></a>
/// </p>
pub fn init_colors() {
    for i in 0..8 {
        for j in 0..8 {
            init_pair(i * 8 + j, i, j);
        }
    }
}

pub const BLACK_ON_BLACK: ColorPair = ColorPair(0);
pub const BLACK_ON_RED: ColorPair = ColorPair(1);
pub const BLACK_ON_GREEN: ColorPair = ColorPair(2);
pub const BLACK_ON_YELLOW: ColorPair = ColorPair(3);
pub const BLACK_ON_BLUE: ColorPair = ColorPair(4);
pub const BLACK_ON_MAGENTA: ColorPair = ColorPair(5);
pub const BLACK_ON_CYAN: ColorPair = ColorPair(6);
pub const BLACK_ON_WHITE: ColorPair = ColorPair(7);

pub const RED_ON_BLACK: ColorPair = ColorPair(8);
pub const RED_ON_RED: ColorPair = ColorPair(9);
pub const RED_ON_GREEN: ColorPair = ColorPair(10);
pub const RED_ON_YELLOW: ColorPair = ColorPair(11);
pub const RED_ON_BLUE: ColorPair = ColorPair(12);
pub const RED_ON_MAGENTA: ColorPair = ColorPair(13);
pub const RED_ON_CYAN: ColorPair = ColorPair(14);
pub const RED_ON_WHITE: ColorPair = ColorPair(15);

pub const GREEN_ON_BLACK: ColorPair = ColorPair(16);
pub const GREEN_ON_RED: ColorPair = ColorPair(17);
pub const GREEN_ON_GREEN: ColorPair = ColorPair(18);
pub const GREEN_ON_YELLOW: ColorPair = ColorPair(19);
pub const GREEN_ON_BLUE: ColorPair = ColorPair(20);
pub const GREEN_ON_MAGENTA: ColorPair = ColorPair(21);
pub const GREEN_ON_CYAN: ColorPair = ColorPair(22);
pub const GREEN_ON_WHITE: ColorPair = ColorPair(23);

pub const YELLOW_ON_BLACK: ColorPair = ColorPair(24);
pub const YELLOW_ON_RED: ColorPair = ColorPair(25);
pub const YELLOW_ON_GREEN: ColorPair = ColorPair(26);
pub const YELLOW_ON_YELLOW: ColorPair = ColorPair(27);
pub const YELLOW_ON_BLUE: ColorPair = ColorPair(28);
pub const YELLOW_ON_MAGENTA: ColorPair = ColorPair(29);
pub const YELLOW_ON_CYAN: ColorPair = ColorPair(30);
pub const YELLOW_ON_WHITE: ColorPair = ColorPair(31);

pub const BLUE_ON_BLACK: ColorPair = ColorPair(32);
pub const BLUE_ON_RED: ColorPair = ColorPair(33);
pub const BLUE_ON_GREEN: ColorPair = ColorPair(34);
pub const BLUE_ON_YELLOW: ColorPair = ColorPair(35);
pub const BLUE_ON_BLUE: ColorPair = ColorPair(36);
pub const BLUE_ON_MAGENTA: ColorPair = ColorPair(37);
pub const BLUE_ON_CYAN: ColorPair = ColorPair(38);
pub const BLUE_ON_WHITE: ColorPair = ColorPair(39);

pub const MAGENTA_ON_BLACK: ColorPair = ColorPair(40);
pub const MAGENTA_ON_RED: ColorPair = ColorPair(41);
pub const MAGENTA_ON_GREEN: ColorPair = ColorPair(42);
pub const MAGENTA_ON_YELLOW: ColorPair = ColorPair(43);
pub const MAGENTA_ON_BLUE: ColorPair = ColorPair(44);
pub const MAGENTA_ON_MAGENTA: ColorPair = ColorPair(45);
pub const MAGENTA_ON_CYAN: ColorPair = ColorPair(46);
pub const MAGENTA_ON_WHITE: ColorPair = ColorPair(47);

pub const CYAN_ON_BLACK: ColorPair = ColorPair(48);
pub const CYAN_ON_RED: ColorPair = ColorPair(49);
pub const CYAN_ON_GREEN: ColorPair = ColorPair(50);
pub const CYAN_ON_YELLOW: ColorPair = ColorPair(51);
pub const CYAN_ON_BLUE: ColorPair = ColorPair(52);
pub const CYAN_ON_MAGENTA: ColorPair = ColorPair(53);
pub const CYAN_ON_CYAN: ColorPair = ColorPair(54);
pub const CYAN_ON_WHITE: ColorPair = ColorPair(55);

pub const WHITE_ON_BLACK: ColorPair = ColorPair(56);
pub const WHITE_ON_RED: ColorPair = ColorPair(57);
pub const WHITE_ON_GREEN: ColorPair = ColorPair(58);
pub const WHITE_ON_YELLOW: ColorPair = ColorPair(59);
pub const WHITE_ON_BLUE: ColorPair = ColorPair(60);
pub const WHITE_ON_MAGENTA: ColorPair = ColorPair(61);
pub const WHITE_ON_CYAN: ColorPair = ColorPair(62);
pub const WHITE_ON_WHITE: ColorPair = ColorPair(63);

use pancurses::chtype;
use pancurses::Window;

/// Extensions over [`Window`](pancurses::Window)
pub trait WindowExt {
    /// Get the middle of the [`Window`](pancurses::Window) if there was a len height thing in the middle
    fn get_mid_yx(&self, len: (i32, i32)) -> (i32, i32);
    /// Get the middle of the [`Window`](pancurses::Window) if there was a len width thing in the middle
    fn get_mid_y(&self, len: i32) -> i32;
    /// Get the middle of the [`Window`](pancurses::Window) if there was a (height, width) thing in the middle
    fn get_mid_x(&self, len: i32) -> i32;
    /// Do something with [`Attributes`](pancurses::Attributes) temporarily
    fn with_attr<T: Into<chtype>>(&self, attrs: T, f: fn(&Window));
}

/// Extensions over [`Window`](pancurses::Window)
impl WindowExt for Window {
    #[inline]
    fn get_mid_y(&self, len: i32) -> i32 {
        (self.get_max_y() - len) / 2
    }
    #[inline]
    fn get_mid_x(&self, len: i32) -> i32 {
        (self.get_max_x() - len) / 2
    }
    #[inline]
    fn get_mid_yx(&self, (y, x): (i32, i32)) -> (i32, i32) {
        (self.get_mid_y(y), self.get_mid_x(x))
    }
    fn with_attr<T: Into<chtype>>(&self, attrs: T, f: fn(&Window)) {
        let i = attrs.into();
        self.attron(i);
        f(self);
        self.attroff(i);
    }
}

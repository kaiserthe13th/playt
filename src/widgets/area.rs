pub trait Area {
    fn area(&self) -> (usize, usize);
    fn area_i32(&self) -> (i32, i32) {
        let (y, x) = self.area();
        (y as i32, x as i32)
    }
}

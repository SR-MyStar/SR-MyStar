pub struct Counter {
    count: u32,
}
impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

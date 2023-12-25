fn main() {
    let rectangle1 = Rectangle::new(30, 50);
    println!("{}\n{rectangle1:?}\n{rectangle1:#?}", rectangle1.area());
    let rectangle2 = Rectangle::new(40, 60);
    let rectangle3 = Rectangle::new(20, 40);
    println!(
        "{}\n{}",
        rectangle2.is_can_hold(&rectangle3),
        rectangle3.is_can_hold(&rectangle2),
    );
    let square = Rectangle::square(10);
    println!("{:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}
impl Rectangle {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    fn square(edge_length: usize) -> Self {
        Self {
            width: edge_length,
            height: edge_length,
        }
    }
}
impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
    fn is_can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }
}

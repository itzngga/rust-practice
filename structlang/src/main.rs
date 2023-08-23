#[derive(Debug)]
struct  Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        return Self {width, height}
    }

    fn area (&self) -> i32{
        return self.width * self.height
    }

    fn width(&self) -> bool {
        return self.width > 0
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);

    // dbg!("{:#?}", rect.area());
    println!("The are aof rectagle is {} square pixels", rect.area());
    println!("{}", rect.width());
}


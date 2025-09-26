#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn fit_in(&self, other_rect: Rectangle) -> bool {
        self.width * self.height > other_rect.width * other_rect.height
    }
}

fn main () {
    let rect_main = Rectangle {
        width: 50,
        height: 25,
    };

    let rect2 = Rectangle {
        width: 80,
        height: 100,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 3,
    };

    dbg!(&rect_main);

    println!("Can rectangle 1 hold rectangle 2? {}", rect_main.fit_in(rect2));
    println!("Can rectangle 1 hold rectangle 3? {}", rect_main.fit_in(rect3));
}
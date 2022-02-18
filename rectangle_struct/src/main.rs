
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn parimiter(&self) -> u32 {
        (self.width  + self.height) * 2
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 20,
    };
    println!("area: {}, parimiter: {}",rect1.area(),rect1.parimiter());
}


struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn parimiter(rectangle: &Rectangle) -> u32 {
    (rectangle.width  + rectangle.height) * 2
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 20,
    };
    println!("area: {}, parimiter: {}",area(&rect1),parimiter(&rect1));
}

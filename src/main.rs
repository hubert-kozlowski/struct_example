#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn extend(&self, scale: u32) -> Rectangle {
        Rectangle {
            width: self.width * scale,
            height: self.height * scale,
        }
    }
}




fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect1);
    let rect2 = rect1.extend(scale);
    dbg!(&rect2);
    
}

use std::io;
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



struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn distance(&self, other: &Point) -> f64 {
        // Pythagorean theorem
        let x_squared = (self.x - other.x).pow(2); 
        let y_squared = (self.y - other.y).pow(2); 
        let pre_dist = (x_squared + y_squared) as f64;
        let dist = pre_dist.sqrt();
        dist
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
    



    let p1 = Point { x: 13, y: 45 };
    let p2 = Point { x: -3, y: 18 };

    println!("Distance between p1 and p2: {}", p1.distance(&p2));


    // pause the program so it doesn't exit immediately
    io::stdin().read_line(&mut String::new()).unwrap(); 
}

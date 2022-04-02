fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_d(rect1)
    );

    let rect = Rectangle{w: 30, h: 50};
    // dbg!(&rect1);
    println!("Rect is {:?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area_s(&rect)
    );

    println!(
        "Can rect hold (20,10)? {}.",
        rect.can_hold(&Rectangle{w: 20, h: 10})
    );
    println!(
        "Can rect hold (100,200)? {}.",
        rect.can_hold(&Rectangle{w: 100, h: 200})
    );

    // *****************

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_d(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}
fn area_s(r: &Rectangle) -> u32 {
    r.w * r.h
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w >= other.w && self.h >= other.h
    }
}

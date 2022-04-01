#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, y: 0.1 };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("RECT AREA {}", rect_area(_rectangle));
    println!("SQUARE {:?}", square(Point{x: 3.0, y:2.0}, 1.0));
}

fn rect_area(rect: Rectangle) -> f32 {
  //let {top_left, bottom_right} = rect

  let top_left = rect.top_left;
  let bottom_right = rect.bottom_right;

  println!("Top Left: {:?}", top_left);
  println!("Bottom Right: {:?}", bottom_right);
  let base = top_left.x- bottom_right.x;
  let height = top_left.y - bottom_right.y;

  println!("Base: {:?} - Height: {:?}", base, height);
  base * height
}

fn square(p: Point, len: f32) -> Rectangle {
  let Point {x: xx, y: yy} = p;
  return Rectangle {
    top_left: p,
    bottom_right: Point {x: xx+len, y: yy-len}
  }
}
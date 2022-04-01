use std::fmt::{self, Formatter, Display};


fn main() {
  println!("Hello World!");
  println!("answer: {}",42);

  println!("My name is {0}, {1} {0}", "Bond", "James");

 
  println!("This struct `{:?}` won't print...", Structure(3));

  let pi = 3.141592;
  println!("Pi is {:.2}", pi);


  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };

  println!("{:#?}", peter);


  for city in [
      City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
      City { name: "Oslo", lat: 59.95, lon: 10.75 },
      City { name: "Vancouver", lat: 49.25, lon: -123.1 },
  ].iter() {
      println!("{}", *city);
  }
  for color in [
      Color { red: 128, green: 255, blue: 90 },
      Color { red: 0, green: 3, blue: 254 },
      Color { red: 0, green: 0, blue: 0 },
  ].iter() {
      // Switch this to use {} once you've added an implementation
      // for fmt::Display.
      println!("{}", *color);
  }


}


#[allow(dead_code)]
#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", "hi")
  }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8
}

// impl fmt::Display for Person<'a> {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{0} ({1})", self.name, self.age)
//   }
// }

#[allow(dead_code)]
struct City {
  name: &'static str,
  // Latitude
  lat: f32,
  // Longitude
  lon: f32,
}

impl Display for City {
  // `f` is a buffer, and this method must write the formatted string into it
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
      let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

      // `write!` is like `format!`, but it will write the formatted string
      // into a buffer (the first argument)
      write!(f, "{}: {:.3}°{} {:.3}°{}",
             self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
  }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RGB ({0}, {1}, {2}) 0x{0:0>2X}{1:0>2X}{2:0>2X}", self.red, self.green, self.blue)
  }
}
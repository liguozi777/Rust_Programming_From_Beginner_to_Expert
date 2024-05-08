use std::fmt::{Display, Formatter, Result};
trait Geometry {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rectangle: ({},{})", self.width, self.height)
    }
}
struct Circle {
    radius: f64,
}
impl Geometry for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * 3.14 * self.radius
    }
}
impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Circle: ({})", self.radius)
    }
}
// fn print(geometry: impl Geometry + Display) {
//     println!("{}", geometry);
//     println!("Area: {}", geometry.area());
//     println!("Perimeter: {}", geometry.perimeter());
// }
fn print<T: Geometry + Display>(geometry: T) {
    println!("{}", geometry);
    println!("Area: {}", geometry.area());
    println!("Perimeter: {}", geometry.perimeter());
}
// fn area_add(geo1:impl Geometry, geo2:impl Geometry){
//     println!("rect.area:{},circle.area:{},total area:{}",geo1.area(),geo2.area(),geo1.area()+geo2.area());
// }
// fn area_add<T: Geometry, U: Geometry>(geo1: T, geo2: U) {
//     println!(
//         "rect.area:{}circle.area:{}total area:{}",
//         geo1.area(),
//         geo2.area(),
//         geo1.area() + geo2.area(),
//     );
// }
fn area_add<T, U>(geo1: T, geo2: U)
where
    T: Geometry + Display,
    U: Geometry,
{
    println!(
        "geo1.area:{},geo2.area:{},total area:{}",
        geo1.area(),
        geo2.area(),
        geo1.area() + geo2.area()
    );
}
fn return_geometry() -> impl Geometry {
    Rectangle {
        width: 12.5,
        height: 5.5,
    }
}
fn main() {
    let rect = Rectangle {
        width: 8.8,
        height: 2.2,
    };
    println!("Rectangle area: {}", rect.area());
    println!("Rectangle perimeter: {}", rect.perimeter());
    let circle = Circle { radius: 3.0 };
    println!("Circle area: {}", circle.area());
    println!("Circle perimeter: {}", circle.perimeter());
    let rect = Rectangle {
        width: 10.5,
        height: 5.5,
    };
    // print(circle);
    area_add(rect, circle);
}

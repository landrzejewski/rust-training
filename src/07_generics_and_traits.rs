use std::{any::type_name, fmt::{Display, Formatter}, ops::Add, ops::Sub};

fn main() {
    println!("Value i32 as string {}", to_string(32));
    println!("Value bool as string {}", to_string(true));
    println!("Value i64 as string {}", to_string(64i64));

    let point = Point {
        x: 30,
        y: 20,
    };


    let other_point = Point {
        x: 30.0,
        y: 20.0,
    };

    // point.show();
    other_point.show();
    println!("{}", point);
    other_point.print_info();

    shape_factory(true).print_info();
    let circle_shape = shape_factory(false);
    circle_shape.print_info();

    draw_shape(circle_shape.as_ref());

}

fn i32_to_string(value: i32) -> String {
    format!("{value}:i32")
}

fn f64_to_string(value: f64) -> String {
    format!("{value}:f64")
}

fn to_string<V: Display>(value: V) -> String {
    format!("{value}:{}", type_name::<V>())
}


struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {

    fn show(&self) {
        println!("({},{})", self.x, self.y)
    }

    /*fn to_string<V: Display>(value: V) -> String {
        format!("{value}")
    }*/
}

trait Show {

    fn get_info(&self) -> String;

    fn print_info(&self) {
        println!("Info {}", self.get_info());
    }

}

// impl<T: Display + Clone> Display for Point<T> {
impl<T> Display for Point<T> where T: Display + Clone {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
    }

}

impl<T: Display + Clone> Show for Point<T> {

    fn get_info(&self) -> String {
        format!("({}:{})", self.x, self.y)
    }
    
}

trait Shape {

    fn print_info(&self);

}

struct Rectangle;

struct Circle;

impl Circle {

    fn print_circle_info(&self) {
        println!("Rectangle")
    }

}

impl Shape for Rectangle {

    fn print_info(&self) {
        println!("Rectangle")
    }

}

impl Shape for Circle {

    fn print_info(&self) {
        println!("Circle")
    }

}

fn shape_factory(is_rectangle: bool) -> Box<dyn Shape> {
    if is_rectangle { Box::new(Rectangle) } else { Box::new(Circle) }
}

fn draw_shape(shape: &dyn Shape) {
    shape.print_info();
}

//fn generic_ops<T>(first_value: T, second_value: T) -> T where T: Add<Output=T> + Sub<Output=T> + Display {
fn generic_ops<T: Add<Output=T> + Sub<Output=T> + Display>(first_value: T, second_value: T) -> T {
    println!("{}", first_value);
    println!("{}", second_value);
    first_value + second_value
}

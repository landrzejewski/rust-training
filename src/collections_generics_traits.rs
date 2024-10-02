use std::any::type_name;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

pub fn run() {
    // collections();
    generics_and_traits();
}

fn collections() {
    // Vectors

    // let mut numbers = Vec::<i32>::new();
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.pop();
    numbers.insert(0, 0);

    let mut letters = vec!['a', 'b'];
    // let letter = letters[10]; // panic

    if let Some(letter) = letters.get(10) {
        println!("Letter: {letter}");
    }

    for letter in &mut letters {
        println!("Letter: {letter}");
    }

    let letters_slice = &letters[0..1];

    let modified_values: Vec<i32> = numbers
        .iter()
        .map(|number| number * 2)
        .filter(|number| number % 2 == 0)
        .collect();

    _ = numbers
        .iter_mut()
        .map(|number| *number = *number * 2)
        .collect::<Vec<_>>();

    for number in &numbers {
        println!("Number: {number}");
    }

    // HashMap

    // let mut ratings = HashMap::<&str, i32>::new();
    let mut ratings: HashMap<&str, i32> = HashMap::new();
    ratings.insert("a", 1);
    ratings.insert("b", 2);

    println!("Value for key a: {}", ratings.entry("a").or_insert(12));

    // ratings["z"]; // panic

    if let Some(rating) = ratings.get("a") {
        println!("Rating: {rating}");
    }

    for (key, value) in &ratings {
        let k = *key;
        println!("key: {key}, value: {value}");
    }
}

fn generics_and_traits() {
    println!("Value i32 as string {}", to_string(32));
    println!("Value bool as string {}", to_string(true));
    println!("Value i64 as string {}", to_string(64i64));

    let point = Point { x: 30, y: 20 };

    let other_point = Point { x: 30.0, y: 20.0 };

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

impl<T> Display for Point<T>
where
    T: Display + Clone,
{
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
    if is_rectangle {
        Box::new(Rectangle)
    } else {
        Box::new(Circle)
    }
}

fn draw_shape(shape: &dyn Shape) {
    shape.print_info();
}

//fn generic_ops<T>(first_value: T, second_value: T) -> T where T: Add<Output=T> + Sub<Output=T> + Display {
fn generic_ops<T: Add<Output = T> + Sub<Output = T> + Display>(
    first_value: T,
    second_value: T,
) -> T {
    println!("{}", first_value);
    println!("{}", second_value);
    first_value + second_value
}

// Can implement Debug implicitly (automatically-generated implementation).
#[derive(Debug)]
pub struct Coord {
    long: f32,
    lat: f32,
}

impl Coord {
    pub fn new(long: f32, lat: f32) -> Coord {
        Coord { long, lat }
    }
}

// Alternatively, can implement Debug explicitly.
/*
impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Coord with custom debug format: [{}, {}]", self.long, self.lat)
    }
}
*/

// Can use various Formatter helper functions to help with formatting.
/*
impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Coordinate structure")
         .field("Longitude", &self.long)
         .field("Latitude", &self.lat)
         .finish()
    }
}
*/

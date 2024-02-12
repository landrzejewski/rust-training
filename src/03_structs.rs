/* Structs

 - grupują elementy dowolnego typy, ale w przeciwieństwie do krotek, pozwalają na ich nazwanie
 - pozwalają na stworzenie wielu instancji mających takie same właściwości (odpowiednik obiektów z innych języków)
 - dostęp do elementów struktury odbywa się za pomocą operatora .
 - jeśli instancja jest mutowalna można modyfikować jej pola
 */

 // unit struct
struct Directory;

// unnamed/tuple structs (nazwanych krotek) oraz pustych struktur (pod kątem Traits)
struct Point(i32, i32);

struct Point2d {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Account {
    active: bool,
    email: String,
    password: String,
}

fn main() {
    // let origin = Point(0, 0);
    // let x = origin.0;

    let point = Point2d {
        x: 2,
        y: 2,
    };
    //let y = point.y;

    let Point2d { x, y } = point;
    println!("x, y: {x}, {y}");
    let Point2d { x: a, y: b } = point;
    println!("x, y: {a}, {b}");
    
    let active = true;
    let mut account = Account {
        email: String::from("jan@training.pl"),
        password: String::from("123"),
        active // to samo co active: active
    };

    println!("{:#?}", account);
    account.active = false;
    println!("{:#?}", account);

    let other_account = Account {
        email: String::from("marek@training.pl"),
        ..account//.clone()
    };
    println!("Other account: {:#?}", other_account);
    // println!("{:?}", account.password); // błąd - po skopiowaniu elementów do other_account straciliśmy częściowo własność (typy referencyjne)
    // println!("{:?}", account); // błąd - po skopiowaniu elementów do other_account straciliśmy częściowo własność (typy referencyjne)

    match point {
        Point2d { x, y: 0 } => println!("On the x axis at {}", x),
        Point2d { x: 0, y } => println!("On the y axis at {}", y),
        Point2d { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    match point {
        Point2d { x, .. } => println!("x is {}", x),  // ignorowanie pozostałych elementów struktury
    }

    let rectangle = Rectangle {
        width: 100,
        height: 50,
    };
    println!("Rectangle area: {}", rectangle.area()); // == Rectangle::area(&rectangle);

    let square = Rectangle::square(10);
    println!("The area of the rectangle is {} square pixels.", square.area()); // == Rectangle::area(&rectangle);

}

#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 { // &self to skrót self: &Self lub self: &Rectangle
        self.width * self.height
    }

    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self { // Associated function, często używana do tworzenia nowych instancji, jak String::new, String::from
        Self {  // W tym przypadku Self to alias do Rectangle
            width: size,
            height: size,
        }
    }
    
}

impl Rectangle {

    fn width(&self) -> u32 {  // można rozdzielić metody na wiele bloków impl
        self.width
    }
    
}

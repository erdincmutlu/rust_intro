use std::collections::HashMap;

struct Point {
    x: f64,
    y: f64,
}

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let a: i32 = 10;
    let b: i32 = 15;
    println!("Hello, world!, {} {}", a, b);

    let unsigned: u32 = 10;
    let signed: i32 = -10;
    let float: f64 = 0.32;

    println!("Different numbers => {}, {}, {}", unsigned, signed, float);

    let character: char = 'a';
    println!("Character => {}", character);

    let boolean: bool = true;
    println!("Boolean => {}", boolean);

    let tuple: (i32, i32, f64, i32, bool) = (1, -2, 3.0, 4, true);
    println!("Tuple => {:?}", tuple);

    // Array is used for grouping same data type
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array => {:?}", array);

    // string is used for grouping characters
    let string: &str = "Hello World";
    println!("String => {}", string);

    // vector is used for grouping same data types and it is dynamic
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    vector.push(6);
    println!("Vector => {:?}", vector);

    // hash map is used for grouping 2 different data types as key value pair
    let mut hash_map: HashMap<&str, i32> = std::collections::HashMap::new();
    hash_map.insert("Solana", 100);
    hash_map.insert("age", 2);
    println!("Hash Map => {:?}", hash_map);

    let mut counter: i32 = 0;
    loop {
        println!("Looping...");

        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    let mut i: i32 = 0;
    while i < 10 {
        println!("Looping => {}", i);
        i += 1;
    }

    for j in 0..10 {
        println!("For looping=> {}", j);
    }

    // Looping over an array
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Looping over an array => {}", element);
    }

    // Loping over an iterator
    let array2: [i32; 5] = [100, 200, 300, 400, 500];
    for (index, value) in array2.iter().enumerate() {
        println!("Value at index {}: {}", index, value);
    }

    // Match
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("It's something else"),
    }

    // Match with complex data structures
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8), // tuple struct variant
    }

    let color = Color::RGB(0, 0, 255);

    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RGB(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
    }

    // Match with pattern
    let number = 5;
    match number {
        1..=5 => println!("One through Five"),
        _ => println!("Something else"),
    }

    // match with combined conditions
    let pair = (0, -2);
    match pair {
        (0, y) => println!("First is 0 and y is {:?}", y),
        (x, 0) => println!("x is {:?} and y is 0", x),
        _ => println!("It doesn't matter what they are"),
    }

    print_greetings();

    print_number(5);

    print_numbers(5, 10);

    let sum = add(5, 10);
    println!("The sum is {}", sum);

    let difference = subtract(10, 5);
    println!("The difference is {}", difference);

    basic_implementation();

    more_implementation();

    trait_implementation();
}

fn print_greetings() {
    println!("Hello World")
}

fn print_number(n: i32) {
    println!("The number is {}", n)
}

fn print_numbers(x: i32, y: i32) {
    println!("The number are {} and {}", x, y)
}

// Function with return type
fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon since it is a return statement
}

// Function with return type and explicit return statement
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

fn basic_implementation() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {}", rect.area());
}

// Implementation with more than one method
fn more_implementation() {
    struct Circle {
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            3.14 * self.radius * self.radius
        }

        fn perimeter(&self) -> f64 {
            2.0 * 3.14 * self.radius
        }
    }

    let circle = Circle { radius: 10.0 };
    println!("The area of the circle is: {}", circle.area());
    println!("The perimeter of the circle is: {}", circle.perimeter());
}

// Implementing traits
fn trait_implementation() {
    trait HasArea {
        fn area(&self) -> f64;
    }
    struct Square {
        side: f64,
    }
    impl HasArea for Square {
        fn area(&self) -> f64 {
            {
                self.side * self.side
            }
        }
    }
}

    // variables can change their value
    // if there's a "mut" keyword

    // constants canNOT do that
    // explicit type and assigning value are a must
    // they do NOT occupy a specific location in memory
    const MAX_PLAYERS: u8 = 10;

    // static variable can be marked as "mut"
    // however it is unsafe
    // such variables DO occupy a specific location in memory
    static CASINO_NAME: &str = "Rusty Casino";

fn main() {
    let x = MAX_PLAYERS;
    let y = CASINO_NAME;

    println!("In '{}' only {} people can play...", y, x);

    let string = "Hello, world!";
    // all string literals are string slices in Rust
    // so they they have '&str' in declaration
    println!("{string}");

    let string_1 = String::from("Hello");
    println!("{string_1}");

    // tuples
    let tuple_1 = (1, 2, 3);
    let tuple_2 = (5, 5.4, "word");
    println!(" > tuple 1: {:?}", tuple_1);
    println!(" > tuple 2: {:?}", tuple_2);

    let string_2 = tuple_1.2;
    println!(" >> tuple 1, element 2 as a &str: {}", string_2);

    // assigning varialbe names to all values in tuple_2 
    let (a, b, c) = tuple_2;
    println!(" > tuple 2 again: {a}, {b}, {c}");

    // emplty tuple = unit type
    // used to explicitly return a value when nothing meaningful can be returned
    let unit = ();
    println!(" > empty tuple: {:?}", unit);

    // type aliasing => "creating" new types to get more readable code
    type Age = u32;
    let a1: Age = 57;
    println!(" age: {a1}");
 
    println!(" > a returned value (x2): {}", my_function(17));

}

fn my_function(x: i32) -> i32 {
    println!(" > my_function called with {x}");
    x * 2
}
